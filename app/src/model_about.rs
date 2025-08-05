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
                        {"🤗 Welcome to the FLExo Atlas!"}
                    </h1>
                    <p>
                        {"The FLExo Atlas contains >5 million exotoxins from nearly 700k prokaryotic (meta)genomes 
                        (i.e., the entire mOTUs v3 database), detected using FLExo, our machine-learning-based exotoxin annotation tool!"}
                    </p>

                    <h1>
                        {"✏️ Citation"}
                    </h1>
                    <p>
                        {"If you found FLExo and/or the FLExo Atlas useful, please cite:"}
                    </p>
                    <blockquote>
                        {"The FLExo Atlas. XXX."}
                    </blockquote>

                    <h1>
                        {"💾 Developers"}
                    </h1>
                    <p>
                        {"FLExo and the FLExo Atlas were developed by the 
                        CompMicroLab and HenLab, both at Umeå University in beautiful Umeå, Sweden!"}
                    </p> 
                </div>
                <br />
            </div>
        }        
    }



}
