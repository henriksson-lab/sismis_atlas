use crate::core_model::*;

use yew::prelude::*;


////////////////////////////////////////////////////////////
/// If condition is met, return "selected", otherwise "". For OPTION
pub fn selected_if(cond: bool) -> String {
    if cond {
        "selected".to_string()
    } else {
        "".to_string()
    }
}   // can do true false https://yew.rs/docs/concepts/html 



impl Model {


    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_landing_page(&self, _ctx: &Context<Self>) -> Html {

        html! {

            <div class="landingdiv">

                <p style="color: rgb(0, 150, 255);">
                    {"test"}
                </p>

                

            </div>
        }
    }



}
