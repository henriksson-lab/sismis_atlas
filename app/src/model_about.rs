use crate::core_model::*;

use yew::prelude::*;

impl Model {

    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_about_pane(&self, _ctx: &Context<Self>) -> Html {

        html! {
            <div>
                <div class="App-divider">
                    {"About the FLExo Atlas"}
                </div>
                <div class="landingdiv">
                    <h1>
                        {"🤗 Welcome to the FLExo Atlas!"}
                    </h1>
                    <p>
                        {"The FLExo Atlas contains >5 million exotoxins and associated genes from nearly 700k prokaryotic (meta)genomes 
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
                        {"🔍 Usage"}
                    </h1>
                    <p style="font-weight: bold; font-style: italic;">
                        {"To browse the FLExo Atlas via the interactive UMAP:"}
                    </p>
                    <p>
                        <ol style="text-align: left;">
                            <li>{"In the toolbar at the top of the page, click the tab labeled `Atlas`; you will see a UMAP, consisting of points, 
                            each of which represents a gene cluster family (GCF) containing one or more exotoxins/associated genes."}</li>
                            <li>{"Click on any point (GCF) in the UMAP (you can use your mouse scrolling function to zoom in and out); 
                            metadata for the corresponding exotoxins/associated genes within that GCF will appear in a table below."}</li>
                            <li>{"In the metadata table, click on any `cluster_id` to view the corresponding entry's gene(s), protein domain(s), and GenBank."}</li>
                        </ol>
                    </p>

                    <h1>
                        {"💾 Developers"}
                    </h1>
                    <p>
                        {"FLExo and the FLExo Atlas were developed by the "}
                        <a href="http://microbe.dev">
                            {"CompMicroLab"}
                        </a>
                        {" and "}
                        <a href="http://www.henlab.org">
                            {"HenLab"}
                        </a> 
                        {", both at "}
                        <a href="https://www.umu.se/">
                            {"Umeå University"}
                        </a>
                        {" in beautiful "}
                        <a href="https://visitumea.se/en">
                            {"Umeå, Sweden!"}
                        </a>
                    </p> 
                </div>
                <br />
            </div>
        }        
    }



}
