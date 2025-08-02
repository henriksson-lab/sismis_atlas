use my_web_app::{Cluster, ClusterRequest, Genbank, SequenceRequest};
use web_sys::window;
use yew::prelude::*;


////////////////////////////////////////////////////////////
/// Which page is currently being shown?
#[derive(Debug)]
#[derive(PartialEq)]
pub enum CurrentPage {
    Home,
    About
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

    pub hover_sequence: Option<String>,        
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
            },




        }
    }


    ////////////////////////////////////////////////////////////
    /// Top renderer of the page
    fn view(&self, ctx: &Context<Self>) -> Html {

        let current_page = match self.current_page { 
            CurrentPage::Home => self.view_landing_page(&ctx),
            CurrentPage::About => self.view_about_pane(&ctx),
        };


        let html_top_buttons = html! {
            <header class="App-header">
                <div id="topmenu" class="topnav">
                    <div class="topnav-right">
                        <a class={active_if(self.current_page==CurrentPage::Home)}       onclick={ctx.link().callback(|_| Msg::OpenPage(CurrentPage::Home))}>{"Home"}</a> 
                        <a class={active_if(self.current_page==CurrentPage::About)}      onclick={ctx.link().callback(|_| Msg::OpenPage(CurrentPage::About))}>{"About"}</a> 
                    </div>
                </div>
            </header>        
        };

        html! {
            <div>
                { html_top_buttons }
                { current_page }
            </div>
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