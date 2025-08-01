


use my_web_app::{Cluster, ClusterRequest, Genbank, SequenceRequest};
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

    GetSequence(String),
    SetMetaTable(Option<Vec<Cluster>>),

    GetGenbank(Vec<String>),
    SetGenbank(Option<Vec<Genbank>>),

    HoverSequence(Option<String>),
    ClickSequence(Option<String>),

}



////////////////////////////////////////////////////////////
/// State of the page
pub struct Model {
    pub current_page: CurrentPage,

    pub current_genbank: Option<Vec<Genbank>>,
    pub current_table_meta: Option<Vec<Cluster>>,

    pub hover_sequence: Option<String>
        
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();


    ////////////////////////////////////////////////////////////
    /// Create a new component
    fn create(_ctx: &Context<Self>) -> Self {

        //ctx.link().send_message(Msg::GetCluster("GUT_GENOME277127-scaffold_21_cluster_1".to_string()));
        //ctx.link().send_message(Msg::GetGenbank("GUT_GENOME277127-scaffold_21_cluster_1".to_string()));

        Self {
            current_page: CurrentPage::Home,
            current_genbank: None,
            current_table_meta: None,
            hover_sequence: None,
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

            Msg::SetMetaTable(data) => {
                //log::debug!("got {:?}",data);
                self.current_table_meta = data;
                self.current_genbank = None;
                true
            },

            Msg::GetSequence(id) => {

                let s=SequenceRequest {
                    sequence_id: id
                };

                let json = serde_json::to_string(&s).expect("Failed to generate json");
                //log::debug!("sending {}", json);
                async fn get_data(json: String) -> Msg {
                    let client = reqwest::Client::new();
                    let res: Vec<Cluster> = client.post(format!("{}/get_sequence",get_host_url()))
                        .header("Content-Type", "application/json")
                        .body(json)
                        .send()
                        .await
                        .expect("Failed to send request")
                        .json()
                        .await
                        .expect("Failed to get table data");

                    Msg::SetMetaTable(Some(res))
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


            Msg::HoverSequence(id) => {
                self.hover_sequence = id;
                true
            }

            Msg::ClickSequence(id) => {

                //Load metadata!

                self.hover_sequence = id.clone();

                if let Some(id) = &id {
                    ctx.link().send_message(Msg::GetSequence(id.clone()));
                } else {
                    
                }


                true
            }

        }
    }


    ////////////////////////////////////////////////////////////
    /// Top renderer of the page
    fn view(&self, ctx: &Context<Self>) -> Html {


        let on_cell_hovered= ctx.link().callback(move |name: Option<String>| {
            //format!("Bye {}", name)
            //log::debug!("meah");
            Msg::HoverSequence(name)
        });

        let on_cell_clicked= ctx.link().callback(move |name: Option<String>| {
            //format!("Bye {}", name)
            //log::debug!("meah");
            Msg::ClickSequence(name)
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
                { format!("{}", if let Some(c) = &self.hover_sequence {c.clone()} else {"No cell hovered".to_string()} )  }

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
    pub fn view_cluster_table(&self, ctx: &Context<Self>) -> Html {

        if let Some(list_cluser) = &self.current_table_meta {

            let list_rows = list_cluser.iter().map(|c| {

                let cluster_id=c.cluster_id.clone();
                let on_click_cluster= ctx.link().callback(move |_e: MouseEvent| {
                    //format!("Bye {}", name)
                    log::debug!("get genbank");
                    Msg::GetGenbank(vec![cluster_id.clone()])
                });


                html! {
                    <tr> 
                        <td> { c.sequence_id.clone() } </td>
                        <td>
                            <a onclick={on_click_cluster}>
                             { c.cluster_id.clone() }
                            </a>
                        </td>
                        <td> { c.start.clone() } </td>
                        <td> { c.end.clone() } </td>
                        <td> { c.average_p.clone() } </td>

                        <td> { c.max_p.clone() } </td>
                        <td> { c.proteins.clone() } </td>
                        <td> { c.domains.clone() } </td>
                        <td> { c.type2.clone() } </td>
                        <td> { c.filepath.clone() } </td>
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