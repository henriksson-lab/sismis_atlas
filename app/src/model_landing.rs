use crate::core_model::*;

use yew::prelude::*;

impl Model {

    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_landing_page(&self, ctx: &Context<Self>) -> Html {

        html! {
            
            <div>
                <div class="landingdiv">

                    <img src="../assets/sismis_logo_blue_path.svg"/>

                    <p style="font-size: 72px; color: #88CCEE;">
                        {"The "}
                        <span style="text-decoration: underline; font-weight: bold;">{"S"}</span>
                        {"ecret"}
                        <span style="text-decoration: underline; font-weight: bold;">{"i"}</span>
                        {"on "}
                        <span style="text-decoration: underline; font-weight: bold;">{"S"}</span>
                        {"yste"}
                        <span style="text-decoration: underline; font-weight: bold;">{"m"}</span>
                        {" D"}
                        <span style="text-decoration: underline; font-weight: bold;">{"is"}</span>
                        {"covery Tool Atlas"}
                    </p>

                    <p style="color: white;">
                        {"Secretion systems from 700k prokaryotic (meta)genomes"}
                    </p>

                    <p style="color: #88CCEE;">
                        {"Version 1"}
                    </p>

                    <p> </p>

                    <button class="toolbutton" onclick={ctx.link().callback(|_| Msg::OpenPage(CurrentPage::Atlas))}>
                        {"Explore the atlas"}
                    </button>

                    <div class="wrapper">
                        <div class="gradient gradient-1"></div>
                        <div class="gradient gradient-2"></div>
                        <div class="gradient gradient-3"></div>
                    </div>

                </div>
            </div>
 
        }
    }




}
