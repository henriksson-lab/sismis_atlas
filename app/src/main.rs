pub mod core_model;
pub mod model_atlas;
pub mod model_landing;
pub mod model_about;
pub mod model_genbank;
pub mod component_umap;
pub mod umap_index;

use crate::core_model::*;

////////////////////////////////////////////////////////////
/// x
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Model>::new().render();
}
