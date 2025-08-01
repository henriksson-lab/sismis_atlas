use my_web_app::UmapData;
use wasm_bindgen::JsCast;
use web_sys::{DomRect, EventTarget, HtmlCanvasElement, WebGlRenderingContext as GL};
use yew::{html, Callback, Component, Context, Html, MouseEvent, NodeRef};

use crate::{core_model::get_host_url, umap_index::UmapPointIndex};


// see https://github.com/yewstack/yew/blob/master/examples/webgl/src/main.rs


// Wrap gl in Rc (Arc for multi-threaded) so it can be injected into the render-loop closure.
pub struct UmapView {
    node_ref: NodeRef,
    umap: Option<UmapData>,
    last_pos: (i32,i32),
    last_cell: Option<String>,
    umap_index: UmapPointIndex
}


////////////////////////////////////////////////////////////
/// Message sent to the event system for updating the page
#[derive(Debug)]
pub enum MsgUMAP {

    GetCoord(),
    SetCoord(Option<UmapData>),

    MouseMove(i32,i32),
    MouseClick,

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

    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_message(MsgUMAP::GetCoord());

        Self {
            node_ref: NodeRef::default(),
            umap: None,
            last_pos: (0,0),
            last_cell: None,
            umap_index: UmapPointIndex::new()
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
            }

        }
    }








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



        let mouseclicked = ctx.link().callback(move |_e: MouseEvent | { 
            MsgUMAP::MouseClick
        });


        html! {
            <canvas ref={self.node_ref.clone()} style="border:1px solid #000000;" onmousemove={mousemoved} onclick={mouseclicked}/>
        }
    }


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

            let num_points = umap.num_point; //data.len()/2;

            let vertices = &umap.data;    

            let vertex_buffer = gl.create_buffer().unwrap();
            let verts = js_sys::Float32Array::from(vertices.as_slice());

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

            let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
            gl.shader_source(&vert_shader, vert_code);
            gl.compile_shader(&vert_shader);

            let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
            gl.shader_source(&frag_shader, frag_code);
            gl.compile_shader(&frag_shader);

            let shader_program = gl.create_program().unwrap();
            gl.attach_shader(&shader_program, &vert_shader);
            gl.attach_shader(&shader_program, &frag_shader);
            gl.link_program(&shader_program);

            gl.use_program(Some(&shader_program));

            // Attach the position vector as an attribute for the GL context.
            let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
            gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);  // size 2!! not 3. so 2d coord
            gl.enable_vertex_attrib_array(position);

            // Attach the time as a uniform for the GL context.
            //let time = gl.get_uniform_location(&shader_program, "u_time");
            //let timestamp = 0.0;
            //gl.uniform1f(time.as_ref(), timestamp as f32);

            gl.draw_arrays(GL::POINTS, 0, num_points as i32);
        }


    }
}

impl UmapView {


}

