use crate::core_model::*;

use yew::prelude::*;

impl Model {

    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_about_pane(&self, _ctx: &Context<Self>) -> Html {

        html! {
            <div>
                <div class="App-divider">
                    {"About xxx"}
                </div>
                <div class="landingdiv">
                    <h1>
                        {"What is xxx?"}
                    </h1>
                    <p>
                    </p>
                </div>
                <br />
            </div>
        }        
    }



}
