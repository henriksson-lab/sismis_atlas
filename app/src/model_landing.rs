use crate::core_model::*;

use yew::prelude::*;

impl Model {

    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_landing_page(&self, ctx: &Context<Self>) -> Html {

        html! {
            <div>
                <div class="background">
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                </div>

                <div class="landingdiv">

                    <img src="../assets/sismis_logo_purple.svg"/>
                    <p></p>

                    <p style="color: white;">
                        <span style="text-decoration: underline; font-weight: bold;">{"F"}</span>
                        {"ast "}
                        <span style="text-decoration: underline; font-weight: bold;">{"L"}</span>
                        {"ocator for "}
                        <span style="text-decoration: underline; font-weight: bold;">{"Exo"}</span>
                        {"toxins"}
                    </p>

                    <p style="color: white;">
                        {">5 million exotoxins and exotoxin-associated genes from nearly 700k prokaryotic (meta)genomes"}
                    </p>

                    <p style="color: #40dba0;">
                        {"Version 1"}
                    </p>

                    <p> </p>

                    <button class="toolbutton" onclick={ctx.link().callback(|_| Msg::OpenPage(CurrentPage::Atlas))}>
                        {"Explore the Sismis Atlas"}
                    </button>

                </div>
            </div>
        }
    }




}
