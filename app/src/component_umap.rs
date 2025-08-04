use my_web_app::{UmapData, UmapMetadata};
use wasm_bindgen::JsCast;
use web_sys::{DomRect, EventTarget, HtmlCanvasElement, HtmlSelectElement, WebGlRenderingContext as GL};
use yew::{html, Callback, Component, Context, Event, Html, MouseEvent, NodeRef, WheelEvent};
use yew::Properties;

use crate::{core_model::get_host_url, umap_index::UmapPointIndex};


// see https://github.com/yewstack/yew/blob/master/examples/webgl/src/main.rs


// Wrap gl in Rc (Arc for multi-threaded) so it can be injected into the render-loop closure.
pub struct UmapView {
    node_ref: NodeRef,
    umap: Option<UmapData>,
    last_pos: (i32,i32),
    last_cell: Option<String>,
    umap_index: UmapPointIndex,

    coloring: UmapMetadata,
    current_coloring: String,

    current_tool: CurrentTool,
    camera: Camera2D,
}


#[derive(Debug, PartialEq)]
pub enum CurrentTool {
    Zoom,
    Select
}

pub struct Camera2D {
    x: f32,
    y: f32,
    zoom: f32
}
impl Camera2D {
    pub fn new() -> Camera2D {
        Camera2D {
            x: 0.0,
            y: 0.0,
            zoom: 1.0
        }
    }

    pub fn cam2world(&self, cx: f32, cy:f32) -> (f32,f32) {
        (
            cx/self.zoom + self.x,
            cy/self.zoom + self.y
        )

    }
}


////////////////////////////////////////////////////////////
/// Message sent to the event system for updating the page
#[derive(Debug)]
pub enum MsgUMAP {

    GetCoord,
    SetCoord(Option<UmapData>),

    MouseMove(i32,i32, bool),
    MouseClick,
    MouseWheel(f32),

    GetColoring,
    SetColoring(UmapMetadata),

    SetCurrentColoring(String),

    SelectCurrentTool(CurrentTool),
}




#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_cell_hovered: Callback<Option<String>>,
    pub on_cell_clicked: Callback<Option<String>>,
}


impl Component for UmapView {
    type Message = MsgUMAP;
    type Properties = Props;

    ////////////////////////////////////////////////////////////
    /// x
    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_message(MsgUMAP::GetCoord);
        ctx.link().send_message(MsgUMAP::GetColoring);

        Self {
            node_ref: NodeRef::default(),
            umap: None,
            last_pos: (0,0),
            last_cell: None,
            umap_index: UmapPointIndex::new(),
            coloring: UmapMetadata::new(),
            current_coloring: String::new(),
            current_tool: CurrentTool::Select,
            camera: Camera2D::new(),
        }
    }





    ////////////////////////////////////////////////////////////
    /// Handle an update message
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {

            MsgUMAP::SetCoord(data) => {
                //log::debug!("got {:?}",data);
                if let Some(umap) = &data {
                    self.umap_index = UmapPointIndex::build_point_index(&umap);
                } else {
                    self.umap_index = UmapPointIndex::new();
                }
                self.umap = data;
                true
            },

            MsgUMAP::GetCoord => {
                //log::debug!("sending {}", json);
                async fn get_data() -> MsgUMAP {
                    let client = reqwest::Client::new();
                    let res: UmapData = client.get(format!("{}/get_umap",get_host_url()))
                        .header("Content-Type", "application/json")
                        .body("") // no body
                        .send()
                        .await
                        .expect("Failed to send request")
                        .json()
                        .await
                        .expect("Failed to get table data");

                    

                    MsgUMAP::SetCoord(Some(res))
                }
                ctx.link().send_future(get_data());
                false
            },


            MsgUMAP::MouseMove(x,y, press_left) => {
                let last_pos = self.last_pos;
                self.last_pos = (x,y);
//                log::debug!(".. {:?}", last_pos);

                //Handle pointer in world coordinates
                let (wx,wy) = self.camera.cam2world(x as f32, y as f32);

                //Handle hovering
                let cp = self.umap_index.get_closest_point(wx, wy, 100.0);
                //log::debug!("p: {:?}",cp);
                //log::debug!("{} {}",x,y);

                let mut point_name = None;
                if let Some(umap) = &self.umap {
                    if let Some(cp) = cp {
                        point_name = Some(umap.ids.get(cp).unwrap().clone());                      
                    }
                }

                let point_changed = self.last_cell != point_name;
                self.last_cell = point_name.clone();

                if point_changed {
                    ctx.props().on_cell_hovered.emit(point_name);
                }

                //Handle zooming
                if self.current_tool == CurrentTool::Zoom && press_left {
                    let dx = x - last_pos.0;
                    let dy = y - last_pos.1;
                    //log::debug!("dd {:?}", (dx,dy));
                    self.camera.x -= (dx as f32) / self.camera.zoom;
                    self.camera.y -= (dy as f32) / self.camera.zoom;
                    return true;
                }
                false
            },


            MsgUMAP::MouseWheel(dy) => {
                //TODO zoom around current position
                //self.last_pos

                self.camera.zoom *= (10.0f32).powf(dy / 100.0);
                true
            },



            MsgUMAP::MouseClick => {

                if self.current_tool==CurrentTool::Select {
                    ctx.props().on_cell_clicked.emit(self.last_cell.clone());
                }

                false
            },



            MsgUMAP::GetColoring => {
                //log::debug!("sending {}", json);
                async fn get_data() -> MsgUMAP {
                    let client = reqwest::Client::new();
                    let res: UmapMetadata = client.get(format!("{}/get_coloring",get_host_url()))
                        .header("Content-Type", "application/json")
                        .body("") // no body
                        .send()
                        .await
                        .expect("Failed to send request")
                        .json()
                        .await
                        .expect("Failed to get table data");
                    MsgUMAP::SetColoring(res)
                }
                ctx.link().send_future(get_data());
                false            
            },

            MsgUMAP::SetColoring(coloring) => {
                //log::debug!("{:?}",coloring);
                self.coloring = coloring;                
                self.current_coloring = self.coloring.colorings.keys().next().expect("No colors available").clone();
                true
            },

            MsgUMAP::SetCurrentColoring(c) => {
                //log::debug!("xxx {}",c);
                self.current_coloring=c;
                true                
            },


            MsgUMAP::SelectCurrentTool(t) => {
                self.current_tool=t;
                true
            },

        }
    }







    ////////////////////////////////////////////////////////////
    /// x
    fn view(&self, ctx: &Context<Self>) -> Html {

        let mousemoved = ctx.link().callback(move |e: MouseEvent | { 
            let target: Option<EventTarget> = e.target();
            let canvas: HtmlCanvasElement = target.and_then(|t| t.dyn_into::<HtmlCanvasElement>().ok()).expect("wrong type");

            let rect:DomRect = canvas.get_bounding_client_rect();
            let x = e.client_x() - (rect.left() as i32);
            let y = e.client_y() - (rect.top() as i32);

            let x_cam = x*1024/(rect.width() as i32);
            let y_cam = y*1024/(rect.height() as i32);

            let press_left = e.buttons() & 1 > 0;


            MsgUMAP::MouseMove(x_cam,y_cam, press_left)
            //there is mouse movement! https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementX 
        });



        let select_factor = ctx.link().callback(move |e: Event | { 
            let target: Option<EventTarget> = e.target();
            let input: HtmlSelectElement = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok()).expect("wrong type"); 
            MsgUMAP::SetCurrentColoring(input.value())
        });

        
        let mousewheel = ctx.link().callback(move |e: WheelEvent | { 
            e.prevent_default();
            MsgUMAP::MouseWheel(e.delta_y() as f32)
        });


        let mouseclicked = ctx.link().callback(move |_e: MouseEvent | { 
            MsgUMAP::MouseClick
        });

        //List factors
        let mut list_factors = Vec::new();
        for i in self.coloring.colorings.keys() {
            list_factors.push(html! {
                <option value={i.clone()} selected={self.current_coloring == *i}> 
                    { i.clone() }
                </option>
            });
        }

        //List colors for this factor
        let mut list_levels = Vec::new();
        if let Some(coloring) = self.coloring.colorings.get(&self.current_coloring) {
            let num_factor_f = coloring.list_levels.len() as f32;
            for (i,lev) in coloring.list_levels.iter().enumerate() {
                let hue = (i as f32)/num_factor_f;
                let hsv = (hue, 1.0, 1.0);
                let rgb = hsv2rgb(hsv);
                let scolor = format!("background-color:{}; min-width:50px;",rgbvec2string(rgb));

                list_levels.push(html! {
                    <tr>
                        <td style={scolor}>
                            { " " }                       
                        </td> 
                        <td>
                            { lev.clone() }
                        </td>
                    </tr>
                });
            }

        }

        let click_select = ctx.link().callback(move |_e: MouseEvent | { 
            MsgUMAP::SelectCurrentTool(CurrentTool::Select)
        });

        let click_zoom = ctx.link().callback(move |_e: MouseEvent | { 
            MsgUMAP::SelectCurrentTool(CurrentTool::Zoom)
        });

    
        fn tool_style(pos: usize, selected: bool) -> String {
            let c=if selected {"#0099FF"} else {"lightgray"};
            format!("position: absolute; left:{}px; top:10px; display: flex; border-radius: 3px; border: 2px solid gray; padding: 5px; background-color: {};", pos, c)
        }


        html! {
            <div style="display: flex; height: 500px; position: relative;">

                <div style="position: absolute; left:0; top:0; display: flex; ">  // width: 80%
                    <canvas ref={self.node_ref.clone()} style="border:1px solid #000000;" onmousemove={mousemoved} onclick={mouseclicked} onwheel={mousewheel} width="800" height="600"/>
                </div>

                <div style="position: absolute; left:0; top:0; display: flex; ">  // width: 80%
                    <svg>
                        <text x=10 y=15>
                            { format!("{}", if let Some(c) = &self.last_cell {c.clone()} else {String::new()}) }
                        </text>
                    </svg>
                </div>

                // select
                <div style={tool_style(730, self.current_tool==CurrentTool::Select)} onclick={click_select}>
                    <svg data-icon="polygon-filter" height="16" role="img" viewBox="0 0 16 16" width="16"><path d="M14 5c-.24 0-.47.05-.68.13L9.97 2.34c.01-.11.03-.22.03-.34 0-1.1-.9-2-2-2S6 .9 6 2c0 .04.01.08.01.12L2.88 4.21C2.61 4.08 2.32 4 2 4 .9 4 0 4.9 0 6c0 .74.4 1.38 1 1.72v4.55c-.6.35-1 .99-1 1.73 0 1.1.9 2 2 2 .74 0 1.38-.4 1.72-1h4.55c.35.6.98 1 1.72 1 1.1 0 2-.9 2-2 0-.37-.11-.7-.28-1L14 9c1.11-.01 2-.9 2-2s-.9-2-2-2zm-4.01 7c-.73 0-1.37.41-1.71 1H3.73c-.18-.3-.43-.55-.73-.72V7.72c.6-.34 1-.98 1-1.72 0-.04-.01-.08-.01-.12l3.13-2.09c.27.13.56.21.88.21.24 0 .47-.05.68-.13l3.35 2.79c-.01.11-.03.22-.03.34 0 .37.11.7.28 1l-2.29 4z" fill-rule="evenodd"></path></svg>
                </div>

                // zoom
                <div style={tool_style(760, self.current_tool==CurrentTool::Zoom)} onclick={click_zoom}>
                    <svg data-icon="zoom-in" height="16" role="img" viewBox="0 0 16 16" width="16"><path d="M7.99 5.99v-2c0-.55-.45-1-1-1s-1 .45-1 1v2h-2c-.55 0-1 .45-1 1s.45 1 1 1h2v2c0 .55.45 1 1 1s1-.45 1-1v-2h2c.55 0 1-.45 1-1s-.45-1-1-1h-2zm7.56 7.44l-2.67-2.68a6.94 6.94 0 001.11-3.76c0-3.87-3.13-7-7-7s-7 3.13-7 7 3.13 7 7 7c1.39 0 2.68-.42 3.76-1.11l2.68 2.67a1.498 1.498 0 102.12-2.12zm-8.56-1.44c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5z" fill-rule="evenodd"></path></svg>
                </div>

                <div style="position: absolute; left:820px; top:0; width: 30%;"> //display: flex; 
                    <div>
                        {"Color by:"}
                        <select onchange={select_factor}>
                            { list_factors }
                        </select>                    
                    </div>
                    <div>
                        <table>
                            { list_levels }
                        </table>
                    </div>
                </div>
            </div>
        }
    }



    ////////////////////////////////////////////////////////////
    /// x
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {

        if let Some(umap) = &self.umap {

            // Only start the render loop if it's the first render
            // There's no loop cancellation taking place, so if multiple renders happen,
            // there would be multiple loops running. That doesn't *really* matter here because
            // there's no props update and no SSR is taking place, but it is something to keep in
            // consideration
            /*
            if !first_render {
                return;
            }

            //////// why is this needed?
 */
            

            // Once rendered, store references for the canvas and GL context. These can be used for
            // resizing the rendering area when the window or canvas element are resized, as well as
            // for making GL calls.
            let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

            canvas.set_width(800);
            canvas.set_height(500);

            let gl: GL = canvas
                .get_context("webgl")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();

            
            // This should log only once -- not once per frame

            let vert_code = include_str!("./umap.vert");
            let frag_code = include_str!("./umap.frag");

            //Get position data
            let num_points = umap.num_point;
            let vertices = &umap.data;    
            let mut vertices_color:Vec<f32> = Vec::new();

            //Get color data  .. same length??
            let coloring = self.coloring.colorings.get(&self.current_coloring);
            if let Some(coloring) = coloring {
                //coloring.values.clone() //can we avoid clone?

                let div_color = (coloring.list_levels.len()) as f32;  //+1 needed as 0=1 in HSV  1+ 
                
                vertices_color.reserve(num_points*3);
                for i in 0..num_points {
                    vertices_color.push(*vertices.get(i*2+0).unwrap());
                    vertices_color.push(*vertices.get(i*2+1).unwrap());

                    let color_index = coloring.values.get(i).expect("Color array does not match size");
                    let hue=(*color_index as f32)/div_color;
                    //log::debug!("hue {}", hue);
                    vertices_color.push(hue);
                }

                //log::debug!("provided colors! num point {} {}", num_points, coloring.list_levels.len());

            } else {

                vertices_color.reserve(num_points*3);
                for i in 0..num_points {
                    vertices_color.push(*vertices.get(i*2+0).unwrap());
                    vertices_color.push(*vertices.get(i*2+1).unwrap());
                    vertices_color.push(1.0);
                }

            };


            //Connect vertex array to GL
            let vertex_buffer = gl.create_buffer().unwrap();
            let verts = js_sys::Float32Array::from(vertices_color.as_slice());
            //let verts = js_sys::Int32Array::from(vertices_int.as_slice());
            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

            //Compile vertex shader
            let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
            gl.shader_source(&vert_shader, vert_code);
            gl.compile_shader(&vert_shader);

            /*
            let msg= gl.get_shader_info_log(&vert_shader);
            if let Some(msg)=msg {
                log::debug!("error {}", msg);
            }*/

            //Compile fragment shader
            let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
            gl.shader_source(&frag_shader, frag_code);
            gl.compile_shader(&frag_shader);

            //Attach shaders
            let shader_program = gl.create_program().unwrap();
            gl.attach_shader(&shader_program, &vert_shader);
            gl.attach_shader(&shader_program, &frag_shader);
            gl.link_program(&shader_program);
            gl.use_program(Some(&shader_program));

            // Attach the position vector as an attribute for the GL context.
            let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
            gl.vertex_attrib_pointer_with_i32(position, 3, GL::FLOAT, false, 0, 0);  // size 2!! not 3. so 2d coord
            gl.enable_vertex_attrib_array(position);

            let u_camera_x = gl.get_uniform_location(&shader_program, "u_camera_x");
            let u_camera_y = gl.get_uniform_location(&shader_program, "u_camera_y");
            let u_camera_zoom = gl.get_uniform_location(&shader_program, "u_camera_zoom");
            gl.uniform1f(u_camera_x.as_ref(), self.camera.x as f32);
            gl.uniform1f(u_camera_y.as_ref(), self.camera.y as f32);
            gl.uniform1f(u_camera_zoom.as_ref(), self.camera.zoom as f32);

            // can we attach one more vector???

            gl.draw_arrays(GL::POINTS, 0, num_points as i32);
        }


    }
}




type Vec3 = (f32,f32,f32);
type Vec4 = (f32,f32,f32,f32);

////////////////////////////////////////////////////////////
/// Convert RGB to HSV, 0-1 range, made to match GLSL version exactly
pub fn hsv2rgb(c: Vec3) -> Vec3 {

    //fract(x) = x - floor(x)

    //mix(x,y,a)
    //x×(1−a)+y×a
    fn mix(x:f32,y:f32,a:f32) -> f32 {
        x*(1.0-a) + y*a
    }

    //clamp(x, min,max)
    //min(max(x, minVal), maxVal)
    fn clamp(x:f32, minval:f32, maxval:f32) -> f32 {
        (x.max(minval)).min(maxval)        
    }

    //vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    let k: Vec4 = (1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);

    //vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    let p0 = ((c.0 + k.0).fract()*6.0 - k.3).abs();
    let p1 = ((c.0 + k.1).fract()*6.0 - k.3).abs();
    let p2 = ((c.0 + k.2).fract()*6.0 - k.3).abs();

    //return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
    let out0 = c.2 * mix(k.0, clamp( p0 - k.0, 0.0, 1.0), c.1);
    let out1 = c.2 * mix(k.0, clamp( p1 - k.0, 0.0, 1.0), c.1);
    let out2 = c.2 * mix(k.0, clamp( p2 - k.0, 0.0, 1.0), c.1);
    (out0, out1, out2)
}



pub fn rgbvec2string(c: Vec3) -> String {
    let red=(c.0*255.0) as u8;
    let green=(c.1*255.0) as u8;
    let blue=(c.2*255.0) as u8;
    format!("#{:02X}{:02X}{:02X}", red, green, blue)
}
