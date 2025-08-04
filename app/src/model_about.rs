use crate::core_model::*;

use yew::prelude::*;

impl Model {

    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_about_pane(&self, _ctx: &Context<Self>) -> Html {

        html! {
            <div>
                <div class="App-divider">
                    {"About FLExo"}
                </div>
                <div class="landingdiv">
                    <h1>
                        {"Hello, world!"}
                    </h1>
                    <p>
                        {"(It's me)"}
                    </p>
                </div>
                <br />
            </div>
        }        
    }



}
