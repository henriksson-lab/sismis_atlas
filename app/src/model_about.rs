use crate::core_model::*;

use yew::prelude::*;

impl Model {

    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_about_pane(&self, _ctx: &Context<Self>) -> Html {

        html! {
            <div>
                <div class="App-divider">
                    {"About the Sismis Atlas"}
                </div>
                <div class="landingdiv">
                    <h1>
                        {"🦇 Welcome to the Sismis Atlas!"}
                    </h1>
                    <p>
                        {"The Sismis Atlas contains single-locus secretion systems from nearly 700k prokaryotic (meta)genomes 
                        (i.e., the entire mOTUs v3 database). Secretion systems were detected using Sismis (Šišmiš, pronounced 'shish-mish'), 
                        our machine-learning-based secretion system annotation tool!"}
                    </p>

                    <h1>
                        {"✏️ Citation"}
                    </h1>
                    <p>
                        {"If you found Sismis and/or the Sismis Atlas useful, please cite:"}
                    </p>
                    <blockquote>
<<<<<<< HEAD
                        {"Martin Larralde, Florian Albrecht, Josefin Blom, 
                        Johan Henriksson, Laura M. Carroll. 2025. 
                        Scalable and interpretable secretion system annotation with Sismis. 
                        bioRxiv doi: https://doi.org/10.1101/2025.09.09.675188."
                        }
=======
                        {"Martin Larralde, Florian Albrecht, Josefin Blom, Johan Henriksson, Laura M Carroll. 2025. Scalable and interpretable secretion system annotation with Sismis. bioRxiv 2025.09.09.675188. doi: https://doi.org/10.1101/2025.09.09.675188."}
>>>>>>> 91b813f70ead5ca2bce72a7156749b75d9c959b8
                    </blockquote>

                    <h1>
                        {"🔍 Usage"}
                    </h1>
                    <p style="font-style: italic;">
                        {"To browse the Sismis Atlas via the interactive UMAP:"}
                    </p>
                    <p>
                        <ol style="text-align: left;">
                            <li>{"In the toolbar at the top of the page, click the tab labeled `Atlas`; you will see a UMAP, consisting of points, 
                            each of which represents a gene cluster family (GCF) containing one or more secretion systems."}</li>
                            <li>{"Click on any point (GCF) in the UMAP (you can use your mouse scrolling function to zoom in and out); 
                            metadata for the corresponding secretion system(s) within that GCF will appear in a table below."}</li>
                            <li>{"In the metadata table, click on any `cluster_id` to view the corresponding entry's gene(s), protein domain(s), and GenBank."}</li>
                        </ol>
                    </p>

                    <h1>
                        {"💾 Developers"}
                    </h1>
                    <p>
                        {"Sismis and the Sismis Atlas were developed by the "}
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
