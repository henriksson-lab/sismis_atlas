use bytes::Buf;
use my_web_app::{UmapData, UmapMetadata};
use wasm_bindgen::JsCast;
use web_sys::{DomRect, EventTarget, HtmlCanvasElement, HtmlSelectElement, WebGlRenderingContext as GL};
use yew::{html, Callback, Component, Context, Event, Html, MouseEvent, NodeRef, WheelEvent};
use yew::Properties;

use crate::umap_index::UmapPointIndex;
use crate::{core_model::get_host_url};


// see https://github.com/yewstack/yew/blob/master/examples/webgl/src/main.rs




////////////////////////////////////////////////////////////
/// x
#[derive(Debug, PartialEq)]
pub enum CurrentTool {
    Zoom,
    ZoomAll,
    Select
}



////////////////////////////////////////////////////////////
/// x
#[derive(Debug, PartialEq)]
pub struct Camera2D {
    x: f32,
    y: f32,
    zoom_x: f32,
    zoom_y: f32,
}
impl Camera2D {

    ////////////////////////////////////////////////////////////
    /// x
    pub fn new() -> Camera2D {
        Camera2D {
            x: 0.0,
            y: 0.0,
            zoom_x: 1.0,
            zoom_y: 1.0,
        }
    }

    ////////////////////////////////////////////////////////////
    /// x
    pub fn cam2world(&self, cx: f32, cy:f32) -> (f32,f32) {
        (
            cx/self.zoom_x + self.x,  
            cy/self.zoom_y + self.y
        )
    }


    ////////////////////////////////////////////////////////////
    /// x
    pub fn world2cam(&self, wx: f32, wy:f32) -> (f32,f32) {
        (
            (wx-self.x)*self.zoom_x,
            (wy-self.y)*self.zoom_y
        )
    }


    ////////////////////////////////////////////////////////////
    /// x
    pub fn fit_umap(&mut self, umap: &UmapData) {

        /*
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;

        //Figure out UMAP point range
        let num_points = umap.num_point;
        for i in 0..num_points {
            let px = *umap.data.get(i*2+0).unwrap();
            let py = *umap.data.get(i*2+1).unwrap();

            max_x = max_x.max(px);
            max_y = max_y.max(py);
            min_x = min_x.min(px);
            min_y = min_y.min(py);
        }
 */
        
        self.x = (umap.min_x + umap.max_x)/2.0;
        self.y = (umap.min_y + umap.max_y)/2.0;

        let world_dx = umap.max_x - umap.min_x;
        let world_dy = umap.max_y - umap.min_y;

        let margin = 0.9;
        self.zoom_x = margin/(world_dx/2.0);
        self.zoom_y = margin/(world_dy/2.0);
    }


    ////////////////////////////////////////////////////////////
    /// Zoom around this position.
    /// i.e. it should be in the same position in camera coordinates after zoom has been applied
    /// 
    /// world2cam(mouse_pos, zoom1) = world2cam(mouse_pos, zoom2)
    /// for: world2cam(wx,zoom_x) = (wx-cam_x)*zoom_x
    /// 
    /// Derivation:
    /// (wx-cam_x1)*zoom1 = (wx-cam_x2)*zoom2
    /// (wx-cam_x1)*zoom1/zoom2 = wx - cam_x2
    /// cam_x2 = wx - (wx-cam_x1)*zoom1/zoom2
    pub fn zoom_around(&mut self, wx: f32, wy: f32, scale: f32) {
        let zoom1_x = self.zoom_x;
        let zoom1_y = self.zoom_y;

        //Apply zoom
        self.zoom_x *= scale;
        self.zoom_y *= scale;

        //Correct position
        self.x = wx - (wx-self.x)*zoom1_x/self.zoom_x;
        self.y = wy - (wy-self.y)*zoom1_y/self.zoom_y;            
    }

}



////////////////////////////////////////////////////////////
/// x
#[derive(Debug, PartialEq)]
pub struct Rectangle2D {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32
}
impl Rectangle2D {
    pub fn range_x(&self) -> (f32, f32) {
        if self.x1<self.x2 {
            (self.x1,self.x2)
        } else {
            (self.x2,self.x1)
        }
    }

    pub fn range_y(&self) -> (f32, f32) {
        if self.y1<self.y2 {
            (self.y1,self.y2)
        } else {
            (self.y2,self.y1)
        }
    }
}




////////////////////////////////////////////////////////////
/// Message sent to the event system for updating the page
#[derive(Debug)]
pub enum MsgUMAP {

    GetCoord,
    SetCoord(Option<UmapData>),

    MouseMove(f32,f32, bool),
    MouseClick,
    MouseWheel(f32),

    MouseStartSelect(f32,f32),
    MouseEndSelect(f32,f32),

    GetColoring,
    SetColoring(UmapMetadata),

    SetCurrentColoring(String),

    SelectCurrentTool(CurrentTool),
}


////////////////////////////////////////////////////////////
/// x
#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_cell_hovered: Callback<Option<String>>,
    pub on_cell_clicked: Callback<Vec<String>>,
}


////////////////////////////////////////////////////////////
/// 
/// Wrap gl in Rc (Arc for multi-threaded) so it can be injected into the render-loop closure.
pub struct UmapView {
    node_ref: NodeRef,
    umap: Option<UmapData>,
    last_pos: (f32,f32),
    last_cell: Option<String>,
    umap_index: UmapPointIndex,

    coloring: UmapMetadata,
    current_coloring: String,

    current_tool: CurrentTool,
    camera: Camera2D,

    current_selection: Option<Rectangle2D>,
}



////////////////////////////////////////////////////////////
/// x
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
            last_pos: (0.0,0.0),
            last_cell: None,
            umap_index: UmapPointIndex::new(), //tricky... adapt to umap size??
            coloring: UmapMetadata::new(),
            current_coloring: String::new(),
            current_tool: CurrentTool::Select,
            camera: Camera2D::new(),
            current_selection: None
        }
    }





    ////////////////////////////////////////////////////////////
    /// Handle an update message
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {

            MsgUMAP::SetCoord(data) => {
                //log::debug!("got {:?}",data);
                if let Some(umap) = &data {
                    //Figure out mindist; 5% of umap size 
                    let world_dx = umap.max_x - umap.min_x;
                    let world_dy = umap.max_y - umap.min_y;
                    let span = world_dx.min(world_dy);      //this is a bit nasty. umap better be somewhat square

                    self.umap_index.build_point_index(&umap, span*0.05); 
                } else {
                    self.umap_index.clear();
                }

                if let Some(umap) = &data {
                    self.camera.fit_umap(&umap);
                }

                self.umap = data;
                true
            },

            MsgUMAP::GetCoord => {
                //log::debug!("sending {}", json);
                async fn get_data() -> MsgUMAP {
                    let client = reqwest::Client::new();
                    //log::debug!("asking for umap");
                    let res = client.get(format!("{}/get_umap",get_host_url()))
                        .header("Content-Type", "application/json")
                        .body("") // no body
                        .send()
                        .await
                        .expect("Failed to send request").bytes().await.expect("Could not get binary data");
                    //log::debug!("got umap");
                    let res = serde_cbor::from_reader(res.reader()).expect("Failed to deserialize");
                    //log::debug!("deserialized umap");
                    MsgUMAP::SetCoord(Some(res))
                }
                ctx.link().send_future(get_data());
                false
            },


            MsgUMAP::MouseMove(x,y, press_left) => {
                let mut do_update = false;
                let last_pos = self.last_pos;
                self.last_pos = (x,y);
//                log::debug!(".. {:?}", last_pos);

                //Handle pointer in world coordinates
                let (wx,wy) = self.camera.cam2world(x as f32, y as f32);

                //Handle hovering
                let cp = self.umap_index.get_closest_point(wx, wy);
                //log::debug!("p: {:?}",cp);
                //log::debug!("{} {}",x,y);

                let mut point_name = None;
                if let Some(umap) = &self.umap {
                    if let Some(cp) = cp {
                        point_name = Some(umap.ids.get(cp).unwrap().clone());                      
                    }
                }

                //If we hover a new point, emit signal
                let point_changed = self.last_cell != point_name;
                self.last_cell = point_name.clone();
                if point_changed {
                    ctx.props().on_cell_hovered.emit(point_name);
                    do_update=true;
                }

                if let Some(sel) = &mut self.current_selection {
                    sel.x2=wx;
                    sel.y2=wy;
                    //log::debug!("sel-move {:?}",sel);
                }

                //Handle panning
                if self.current_tool == CurrentTool::Zoom && press_left {
                    let dx = x - last_pos.0;
                    let dy = y - last_pos.1;
                    //log::debug!("dd {:?}", (dx,dy));
                    self.camera.x -= (dx as f32) / self.camera.zoom_x;
                    self.camera.y -= (dy as f32) / self.camera.zoom_y;
                    return true;
                }

                //Always update view if a selection is going on
                if let Some(_sel) = &self.current_selection {
                    do_update=true;
                }
                do_update
            },


            MsgUMAP::MouseWheel(dy) => {
                let (cx,cy) = self.last_pos;
                let (wx, wy) = self.camera.cam2world(cx, cy);
                let scale = (10.0f32).powf(dy / 100.0);
                self.camera.zoom_around(wx,wy, scale);
                true
            },



            MsgUMAP::MouseClick => {
                false
            },



            MsgUMAP::GetColoring => {
                //log::debug!("sending {}", json);
                async fn get_data() -> MsgUMAP {
                    let client = reqwest::Client::new();
                    //log::debug!("get coloring");
                    let res = client.get(format!("{}/get_coloring",get_host_url()))
                        .header("Content-Type", "application/json")
                        .body("") // no body
                        .send()
                        .await
                        .expect("Failed to send request").bytes().await.expect("Could not get binary data");
                    //log::debug!("got bytes");
                    let res = serde_cbor::from_reader(res.reader()).expect("Failed to deserialize");
                    //log::debug!("got deserialized");
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
                if t==CurrentTool::ZoomAll {
                    if let Some(umap) = &self.umap {
                        self.camera.fit_umap(umap);
                    }
                } else {
                    self.current_tool=t;
                }
                true
            },


            MsgUMAP::MouseStartSelect(cx,cy) => {
                if self.current_tool==CurrentTool::Select {
                    let (wx,wy) = self.camera.cam2world(cx as f32, cy as f32);
                    self.current_selection = Some(Rectangle2D {
                        x1: wx,
                        x2: wx,
                        y1: wy,
                        y2: wy
                    });
                    //log::debug!("sel-start {:?}",self.current_selection);
                    true
                } else {
                    false
                }
            }


            MsgUMAP::MouseEndSelect(cx,cy) => {
                if let Some(rect) = &mut self.current_selection {
                    let (wx,wy) = self.camera.cam2world(cx as f32, cy as f32);
                    rect.x2=wx;
                    rect.y2=wy;

                    if let Some(umap) = &self.umap {

                        let (x1,x2) =rect.range_x();
                        let (y1,y2) =rect.range_y();

                        if x1==x2 && y1==y2 {
                            log::debug!("this is a click");

                            if self.current_tool==CurrentTool::Select {
                                if let Some(cell) = &self.last_cell {
                                    ctx.props().on_cell_clicked.emit(vec![cell.clone()]);
                                }
                            }

                        } else {
                            log::debug!("this is a rect select");

                            //log::debug!("wrect {} -- {}     {} -- {}", x1,x2,    y1,y2);

                            //Scan all points to see if they are within the selection 
                            let mut selected_vert = Vec::new();
                            let num_points = umap.num_point;
                            let vertices = &umap.data;    
                            for i in 0..num_points {
                                let px = *vertices.get(i*2+0).unwrap();
                                let py = *vertices.get(i*2+1).unwrap();
                                //log::debug!("{} {}", px, py);
                                if px>x1 && px<x2 && py>y1 && py<y2 { /////////////////////// TODO - invert y axis??   ////////////////// points halfway down are at y=500
                                    let point_name = umap.ids.get(i).unwrap().clone();
    //                                selected_vert.push(i);
                                    selected_vert.push(point_name);
                                }
                            }
                            //log::debug!("sel-end {:?}",rect);
                            log::debug!("sel-en!! {:?}",selected_vert);

                            ctx.props().on_cell_clicked.emit(selected_vert);                            
                        }
                    }
                    self.current_selection=None;
                }
                true
            }

        }
    }




    ////////////////////////////////////////////////////////////
    /// x
    fn view(&self, ctx: &Context<Self>) -> Html {

        let mousemoved = ctx.link().callback(move |e: MouseEvent | { 
            e.prevent_default();
            let (x_cam, y_cam) = mouseevent_get_cx(&e);
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

        let click_zoomall = ctx.link().callback(move |_e: MouseEvent | { 
            MsgUMAP::SelectCurrentTool(CurrentTool::ZoomAll)
        });



        let onmousedown = ctx.link().callback(move |e: MouseEvent | { 
            e.prevent_default();
            let (x_cam, y_cam) = mouseevent_get_cx(&e);
            MsgUMAP::MouseStartSelect(x_cam, y_cam)
        });

        let onmouseup = ctx.link().callback(move |e: MouseEvent | { 
            e.prevent_default();
            let (x_cam, y_cam) = mouseevent_get_cx(&e);
            MsgUMAP::MouseEndSelect(x_cam, y_cam)
        });

        
    
        fn tool_style(pos: usize, selected: bool) -> String {
            let c=if selected {"#0099FF"} else {"lightgray"};
            format!("position: absolute; left:{}px; top:10px; display: flex; border-radius: 3px; border: 2px solid gray; padding: 5px; background-color: {};", pos, c)
        }



        // Render selection box
        let html_select = if let Some(rect) = &self.current_selection {

            let (x1,x2) = rect.range_x();
            let (y1,y2) = rect.range_y();

            let (x1,y1) = self.camera.world2cam(x1, y1);
            let (x2,y2) = self.camera.world2cam(x2, y2);
            html! {
                <rect x={x1.to_string()} y={y1.to_string()} width={(x2-x1).to_string()} height={(y2-y1).to_string()}    fill-opacity="0.1" fill="blue" stroke-width="2" stroke="black" stroke-dasharray="5,5"/> //fillstyle="fill:rgba(0,0,0,0.1);stroke-width:1;"
            }
        } else {
            html! {""}
        };



        html! {
            <div style="display: flex; height: 500px; position: relative;">

                <div style="position: absolute; left:0; top:0; display: flex; ">  // width: 80%
                    <canvas ref={self.node_ref.clone()} style="border:1px solid #000000;" onmousemove={mousemoved} onclick={mouseclicked} onwheel={mousewheel} onmousedown={onmousedown} onmouseup={onmouseup} width="800" height="600"/>
                </div>

                //Overlay SVG
                <div style="position: absolute; left:0; top:0; display: flex; pointer-events: none; ">  
                    <svg style="width: 800px; height: 500px; pointer-events: none;"> // note: WxH must cover canvas!!  
                        <text x=10 y=15>
                            { format!("{}", if let Some(c) = &self.last_cell {c.clone()} else {String::new()}) }
                        </text>
                        { html_select }
                        //<rect x="100" y="100" width="300" height="300" fill="blue"/>
                    </svg>
                </div>

                // Button: Select
                <div style={tool_style(760, self.current_tool==CurrentTool::Select)} onclick={click_select}>
                    <svg data-icon="polygon-filter" height="16" role="img" viewBox="0 0 16 16" width="16"><path d="M14 5c-.24 0-.47.05-.68.13L9.97 2.34c.01-.11.03-.22.03-.34 0-1.1-.9-2-2-2S6 .9 6 2c0 .04.01.08.01.12L2.88 4.21C2.61 4.08 2.32 4 2 4 .9 4 0 4.9 0 6c0 .74.4 1.38 1 1.72v4.55c-.6.35-1 .99-1 1.73 0 1.1.9 2 2 2 .74 0 1.38-.4 1.72-1h4.55c.35.6.98 1 1.72 1 1.1 0 2-.9 2-2 0-.37-.11-.7-.28-1L14 9c1.11-.01 2-.9 2-2s-.9-2-2-2zm-4.01 7c-.73 0-1.37.41-1.71 1H3.73c-.18-.3-.43-.55-.73-.72V7.72c.6-.34 1-.98 1-1.72 0-.04-.01-.08-.01-.12l3.13-2.09c.27.13.56.21.88.21.24 0 .47-.05.68-.13l3.35 2.79c-.01.11-.03.22-.03.34 0 .37.11.7.28 1l-2.29 4z" fill-rule="evenodd"></path></svg>
                </div>

                // Button: Zoom
                <div style={tool_style(730, self.current_tool==CurrentTool::Zoom)} onclick={click_zoom}>
                    <svg data-icon="zoom-in" height="16" role="img" viewBox="0 0 16 16" width="16"><path d="M7.99 5.99v-2c0-.55-.45-1-1-1s-1 .45-1 1v2h-2c-.55 0-1 .45-1 1s.45 1 1 1h2v2c0 .55.45 1 1 1s1-.45 1-1v-2h2c.55 0 1-.45 1-1s-.45-1-1-1h-2zm7.56 7.44l-2.67-2.68a6.94 6.94 0 001.11-3.76c0-3.87-3.13-7-7-7s-7 3.13-7 7 3.13 7 7 7c1.39 0 2.68-.42 3.76-1.11l2.68 2.67a1.498 1.498 0 102.12-2.12zm-8.56-1.44c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5z" fill-rule="evenodd"></path></svg>
                </div>

                // Button: Zoom all
                <div style={tool_style(700, self.current_tool==CurrentTool::ZoomAll)} onclick={click_zoomall}>
                    <svg data-icon="zoom-in" height="16" width="16" xmlns="http://www.w3.org/2000/svg"><path style="fill:none;stroke:#000;stroke-width:2.01074px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1" d="M14.733 8.764v5.973H9.586m-8.29-5.973v5.973h5.146m8.29-7.5V1.264H9.587m-8.29 5.973V1.264h5.146"/></svg>
                </div>
                


                <div style="position: absolute; left:820px; top:0; width: 30%;"> //display: flex; 
                    <div>
                        <p style="font-family: 'Roboto', sans-serif; color: black">
                            {"Color by: "}
                        </p>
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
            canvas.set_height(500); ///////////////////////////////////////////////////////////////////////////// TODO: adapt somehow?

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
            let u_camera_zoom_x = gl.get_uniform_location(&shader_program, "u_camera_zoom_x");
            let u_camera_zoom_y = gl.get_uniform_location(&shader_program, "u_camera_zoom_y");
            gl.uniform1f(u_camera_x.as_ref(), self.camera.x as f32);
            gl.uniform1f(u_camera_y.as_ref(), self.camera.y as f32);
            gl.uniform1f(u_camera_zoom_x.as_ref(), self.camera.zoom_x as f32);
            gl.uniform1f(u_camera_zoom_y.as_ref(), self.camera.zoom_y as f32);


            //log::debug!("canvas {} {}   {:?}", canvas.width(), canvas.height(), self.camera);

            let u_display_w = gl.get_uniform_location(&shader_program, "u_display_w");
            let u_display_h = gl.get_uniform_location(&shader_program, "u_display_h");
            gl.uniform1f(u_display_w.as_ref(), canvas.width() as f32);
            gl.uniform1f(u_display_h.as_ref(), canvas.height() as f32);

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


////////////////////////////////////////////////////////////
/// x
pub fn rgbvec2string(c: Vec3) -> String {
    let red=(c.0*255.0) as u8;
    let green=(c.1*255.0) as u8;
    let blue=(c.2*255.0) as u8;
    format!("#{:02X}{:02X}{:02X}", red, green, blue)
}




fn mouseevent_get_cx(e: &MouseEvent) -> (f32,f32) {
    let target: Option<EventTarget> = e.target();
    let canvas: HtmlCanvasElement = target.and_then(|t| t.dyn_into::<HtmlCanvasElement>().ok()).expect("wrong type");

    let rect:DomRect = canvas.get_bounding_client_rect();
    let x = e.client_x() - (rect.left() as i32);
    let y = e.client_y() - (rect.top() as i32);

    let w = rect.width() as f32;
    let h = rect.height() as f32;

    let x_cam = (x as f32 - w/2.0)/(w/2.0);
    let y_cam = (y as f32 - h/2.0)/(h/2.0);

//    log::debug!("getcx  {} {}", x_cam, y_cam);

    (x_cam, y_cam)
}
