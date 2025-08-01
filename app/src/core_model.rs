


use my_web_app::{Cluster, ClusterRequest, Genbank};
use web_sys::window;
use yew::prelude::*;

use crate::webgl::UmapView;

////////////////////////////////////////////////////////////
/// Which page is currently being shown?
#[derive(Debug)]
#[derive(PartialEq)]
pub enum CurrentPage {
    Home
}


////////////////////////////////////////////////////////////
/// 
#[derive(Debug)]
pub enum IncludeData {
    All,
    Selected
}

////////////////////////////////////////////////////////////
/// Message sent to the event system for updating the page
#[derive(Debug)]
pub enum Msg {

    OpenPage(CurrentPage),

    GetCluster(String),
    SetCluster(Option<Vec<Cluster>>),

    GetGenbank(String),
    SetGenbank(Option<Vec<Genbank>>),

}



////////////////////////////////////////////////////////////
/// State of the page
pub struct Model {
    pub current_page: CurrentPage,

    pub current_genbank: Option<Vec<Genbank>>,
    pub current_cluster: Option<Vec<Cluster>>,
        
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();


    ////////////////////////////////////////////////////////////
    /// Create a new component
    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_message(Msg::GetCluster("GUT_GENOME277127-scaffold_21_cluster_1".to_string()));
        ctx.link().send_message(Msg::GetGenbank("GUT_GENOME277127-scaffold_21_cluster_1".to_string()));

        Self {
            current_page: CurrentPage::Home,
            current_genbank: None,
            current_cluster: None
        }
    }




    ////////////////////////////////////////////////////////////
    /// Handle an update message
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {

            Msg::OpenPage(page) => {
                self.current_page = page;
                true
            },

            Msg::SetCluster(data) => {
                //log::debug!("got {:?}",data);
                self.current_cluster = data;
                true
            },

            Msg::GetCluster(id) => {

                let s=ClusterRequest {
                    cluster_id: id
                };

                let json = serde_json::to_string(&s).expect("Failed to generate json");
                //log::debug!("sending {}", json);
                async fn get_data(json: String) -> Msg {
                    let client = reqwest::Client::new();
                    let res: Vec<Cluster> = client.post(format!("{}/get_cluster",get_host_url()))
                        .header("Content-Type", "application/json")
                        .body(json)
                        .send()
                        .await
                        .expect("Failed to send request")
                        .json()
                        .await
                        .expect("Failed to get table data");

                    Msg::SetCluster(Some(res))
                }
                ctx.link().send_future(get_data(json));
                false
            },

            Msg::SetGenbank(data) => {
                //log::debug!("got {:?}",data);
                self.current_genbank = data;
                true
            },

            Msg::GetGenbank(id) => {

                let s=ClusterRequest {
                    cluster_id: id
                };

                let json = serde_json::to_string(&s).expect("Failed to generate json");
                //log::debug!("sending {}", json);
                async fn get_data(json: String) -> Msg {
                    let client = reqwest::Client::new();
                    let res: Vec<Genbank> = client.post(format!("{}/get_genbank",get_host_url()))
                        .header("Content-Type", "application/json")
                        .body(json)
                        .send()
                        .await
                        .expect("Failed to send request")
                        .json()
                        .await
                        .expect("Failed to get table data");

                    Msg::SetGenbank(Some(res))
                }
                ctx.link().send_future(get_data(json));
                false
            },



        }
    }


    ////////////////////////////////////////////////////////////
    /// Top renderer of the page
    fn view(&self, ctx: &Context<Self>) -> Html {


        let on_cell_hovered: Callback<Option<String>, ()> = Callback::from(move |_name: Option<String>| {
            //format!("Bye {}", name)
            log::debug!("meah");
        });

        let on_cell_clicked: Callback<Option<String>, ()> = Callback::from(move |_name: Option<String>| {
            //format!("Bye {}", name)
            log::debug!("wheee");
        });


        let current_page = match self.current_page { 
            CurrentPage::Home => self.view_landing_page(&ctx),
        };


        let html_top_buttons = html! {
            <header class="App-header">
                <div id="topmenu" class="topnav">
                    <div class="topnav-right">
                        <a class={active_if(self.current_page==CurrentPage::Home)}       onclick={ctx.link().callback(|_| Msg::OpenPage(CurrentPage::Home))}>{"Home"}</a> 
                    </div>
                </div>
            </header>        
        };

        html! {
            <div>
                { html_top_buttons }
                { current_page }

                <UmapView on_cell_hovered={on_cell_hovered} on_cell_clicked={on_cell_clicked}/> //// we really do not want to re-render this if needed! how to avoid?

                { self.view_cluster_table(ctx) }
                { self.view_genbank_svgs(ctx) }


                { self.view_genbank_table(ctx) }


            </div>
        }
    }



}




impl Model {


    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_cluster_table(&self, _ctx: &Context<Self>) -> Html {

        if let Some(list_cluser) = &self.current_cluster {

            let list_rows = list_cluser.iter().map(|c| {
                html! {
                    <tr> 
                        <td> { c.sequence_id.clone() } </td>
                        <td> { c.cluster_id.clone() } </td>
                        <td> { c.start.clone() } </td>
                        <td> { c.end.clone() } </td>
                        <td> { c.average_p.clone() } </td>

                        <td> { c.max_p.clone() } </td>
                        <td> { c.proteins.clone() } </td>
                        <td> { c.domains.clone() } </td>
                        <td> { c.type2.clone() } </td>
                        <td> { c.filepath.clone() } </td>


//                        { format!("{:?}", val.clone()) } 
                    </tr>
                }
            }).collect::<Html>();

            html! {
                <table>
                    <tr>
                        <th> {"sequence_id"} </th>
                        <th> {"cluster_id"} </th>
                        <th> {"start"} </th>
                        <th> {"end"} </th>
                        <th> {"average_p"} </th>

                        <th> {"max_p"} </th>
                        <th> {"proteins"} </th>
                        <th> {"domains"} </th>
                        <th> {"type2"} </th>
                        <th> {"filepath"} </th>

                    </tr>
                    { list_rows }
                </table>
            }

        } else {
            html! { "" }
        }

    }


}




////////////////////////////////////////////////////////////
/// If condition is met, return "active", otherwise "". For CSS styling of which control is active
pub fn active_if(cond: bool) -> String {
    if cond {
        "active".to_string()
    } else {
        "".to_string()
    }
}




////////////////////////////////////////////////////////////
/// Show an alert message
pub fn alert(s: &str) {
    let window = window().expect("no window");
    window.alert_with_message(s).unwrap();
}


pub fn get_host_url() -> String {
    let document = window().expect("no window").document().expect("no document on window");
    let location = document.location().expect("no location");
    let protocol = location.protocol().expect("no protocol");
    let host = location.host().expect("no host");

    let url = format!("{}//{}", protocol, host);
    //log::debug!("{}",url);
    url
}

// https://yew.rs/docs/next/advanced-topics/struct-components/hoc