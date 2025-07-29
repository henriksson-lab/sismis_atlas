pub mod core_model;
pub mod model_landing;
pub mod webgl;

//use crate::core_model::*;
use crate::webgl::App;

////////////////////////////////////////////////////////////
/// x
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
//    yew::Renderer::<Model>::new().render();
    yew::Renderer::<App>::new().render();

}
