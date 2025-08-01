use my_web_app::{UmapData, UmapMetadata};
use wasm_bindgen::JsCast;
use web_sys::{DomRect, EventTarget, HtmlCanvasElement, HtmlSelectElement, WebGlRenderingContext as GL};
use yew::{html, Callback, Component, Context, Event, Html, MouseEvent, NodeRef};

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
}


////////////////////////////////////////////////////////////
/// Message sent to the event system for updating the page
#[derive(Debug)]
pub enum MsgUMAP {

    GetCoord(),
    SetCoord(Option<UmapData>),

    MouseMove(i32,i32),
    MouseClick,

    GetColoring,
    SetColoring(UmapMetadata),

    SetCurrentColoring(String),
}



use yew::Properties;

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

        ctx.link().send_message(MsgUMAP::GetCoord());
        ctx.link().send_message(MsgUMAP::GetColoring);

        Self {
            node_ref: NodeRef::default(),
            umap: None,
            last_pos: (0,0),
            last_cell: None,
            umap_index: UmapPointIndex::new(),
            coloring: UmapMetadata::new(),
            current_coloring: String::new(),
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

            MsgUMAP::GetCoord() => {
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


            MsgUMAP::MouseMove(x,y) => {

                self.last_pos = (x,y);
                let cp = self.umap_index.get_closest_point(x as f32, y as f32, 100.0);
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

                false
            },

            MsgUMAP::MouseClick => {

                ctx.props().on_cell_clicked.emit(self.last_cell.clone());

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
                log::debug!("{:?}",coloring);
                self.coloring = coloring;                
                self.current_coloring = self.coloring.colorings.keys().next().expect("No colors available").clone();
                true
            },

            MsgUMAP::SetCurrentColoring(c) => {
                //log::debug!("xxx {}",c);
                self.current_coloring=c;
                true                
            }

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

            MsgUMAP::MouseMove(x_cam,y_cam)
        });


        let select_factor = ctx.link().callback(move |e: Event | { 
            let target: Option<EventTarget> = e.target();
            let input: HtmlSelectElement = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok()).expect("wrong type"); 
            MsgUMAP::SetCurrentColoring(input.value())
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
            let mul= 255/coloring.list_levels.len();
            for (i,lev) in coloring.list_levels.iter().enumerate() {
                let hue=i*mul;
                list_levels.push(html! {
                    <li>
                        <span style={format!("color:hsl({}, 100%, 50%)",hue)}> //////////// equation disagrees; convert HSV properly
                            { "**** " }                       
                        </span> { lev.clone() }
                    </li>
                });
            }

        }


        html! {
            <div>
                <div>
                    {"Color by:"}
                    <select onchange={select_factor}>
                        { list_factors }
                    </select>                    
                </div>
                <canvas ref={self.node_ref.clone()} style="border:1px solid #000000;" onmousemove={mousemoved} onclick={mouseclicked}/>
                <ul>
                    { list_levels }
                </ul>
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

            // can we attach one more vector???

            gl.draw_arrays(GL::POINTS, 0, num_points as i32);
        }


    }
}

