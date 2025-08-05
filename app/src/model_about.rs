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
                        {"What is the FLExo Atlas?"}
                    </h1>
                    <p>
                        {"The FLExo Atlas contains >5 million exotoxins from nearly 700k prokaryotic (meta)genomes 
                        (i.e., the mOTUs v3 database), detected using FLExo, our machine-learning-based exotoxin annotation tool!"}
                    </p>
                    <h1>
                        {"Citation"}
                    </h1>
                    <p>
                        {"If you found FLExo and/or the FLExo Atlas useful, please cite:"}
                    </p>
                    <blockquote>
                        {"The FLExo Atlas. XXX."}
                    </blockquote>
                </div>
                <br />
            </div>
        }        
    }



}
