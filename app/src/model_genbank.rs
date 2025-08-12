
use std::io::Cursor;

use crate::core_model::*;

use my_web_app::Genbank;
use yew::prelude::*;
use gb_io::{reader::SeqReader, seq::{Feature, Seq}};

use web_sys::{window, HtmlElement};
use wasm_bindgen::JsCast;
use web_sys::js_sys::{Array};
use web_sys::{wasm_bindgen::JsValue, Blob, BlobPropertyBag};

//see https://github.com/gamcil/clinker
//https://github.com/art-egorov/lovis4u
//https://crates.io/crates/gb-io




impl Model {

    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_genbank_table(&self, _ctx: &Context<Self>) -> Html {
        if let Some(list_genbank) = &self.current_genbank {

            list_genbank.iter().map(|val| {

                let cloned_gb = val.clone();
                let mouseclicked = Callback::from(move |_e: MouseEvent | { 
                    download_genbank(&cloned_gb);
                });


                html!{
                    <div>
                        <p>
                        </p>
                        <div class="downloadbutton" onclick={mouseclicked}>
                            {"Download GenBank"}
                        </div>
                        <p>
                        </p>
                        <pre> 
                            { val.data.clone() } 
                        </pre>
                    </div>
                }
            }).collect::<Html>()

        } else {
            html! { "" }
        }
    }




    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_genbank_svgs(&self, ctx: &Context<Self>) -> Html {
        if let Some(list_genbank) = &self.current_genbank {

            list_genbank.iter().map(|val| {
                html!{
                    <pre> 
                        { self.view_genbank_one_svg(ctx, val) } 
                    </pre>
                }
            }).collect::<Html>()

        } else {
            html! { "" }
        }
    }


    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_genbank_one_svg(&self, _ctx: &Context<Self>, gb: &Genbank) -> Html {

        let rdr=Cursor::new(gb.data.as_bytes());

        let mut the_seq = None;
        for seq in SeqReader::new(rdr) {
            let seq = seq.unwrap();
            the_seq = Some(seq);
        }
        if let Some(seq) = the_seq {

            //log::debug!("got gb {:?}",seq);

            let view = GenbankView::from(&seq);

            let mut list_features = Vec::new();

            let lane_height = 12.0;
            let arrow_height = 10.0 as f32;

            for (lane_i, curlane) in view.lanes.iter().enumerate() {

                let cur_y = 2.0 + lane_i as f32 * lane_height ;
                for f in &curlane.features {

                    if let Ok(bounds) = f.location.find_bounds() {

                        let is_box = f.kind != "CDS";

                        let x1 = (bounds.0 as f32) * view.scale_x;
                        let x2 = (bounds.1 as f32) * view.scale_x;
                        let xmid = if is_box {
                            x2
                        } else {
                            (x2-5.0).max(x1)
                        };


                        let y1 = cur_y;
                        let y2 = cur_y + arrow_height;
                        let ymid = (y1+y2)/2.0;

/*
                        let h = html! { 
                            <polygon points={format!("{},{} {},{} {},{} {},{} {},{}",
                                x1,y1,
                                xmid,y1,
                                x2,ymid,
                                xmid,y2,
                                x1,y2,                        
                            )} stroke="black" fill="gray"/>
                        };
*/

                        let h = if f.kind != "CDS" {
                            html! { 
                                <polygon points={format!("{},{} {},{} {},{} {},{} {},{}",
                                    x1,y1,
                                    xmid,y1,
                                    x2,ymid,
                                    xmid,y2,
                                    x1,y2,                        
                                )} stroke="black" fill="#cd1076"/>
                        
                            }
                        } else {
                            html! { 
                                <polygon points={format!("{},{} {},{} {},{} {},{} {},{}",
                                    x1,y1,
                                    xmid,y1,
                                    x2,ymid,
                                    xmid,y2,
                                    x1,y2,                        
                                )} stroke="black" fill="#1e90ff"/>
                            }
                        };



                        list_features.push(h);


                        let xtext = x1+2.0;
                        let ytext = y2-2.0;

                        //let q = format!("{:?}", f.qualifiers);

                        //let is_cds = f.kind == "CDS";


                        let mut show_text = String::new();
                        for q in &f.qualifiers {
                            if let Some(val) = &q.1 {
                                if q.0 == "locus_tag" {
                                    show_text.push_str(val.as_str());
                                    show_text.push_str("; ");
                                }
                                if q.0 == "function" {
                                    show_text.push_str(val.as_str());
                                    show_text.push_str("; ");
                                }
                                if q.0 == "standard_name" {
                                    show_text.push_str(val.as_str());
                                    show_text.push_str("; ");
                                }
                            }
                        }

                        let h= html! { 
                            <text x={xtext.to_string()}  y={ytext.to_string()} fill="white" font-size="8" font-family="'Roboto', sans-serif">
                                { show_text.clone() }
                                <title>
                                    { show_text }
                                </title>
                            </text>
                        };
                        list_features.push(h);
                    }
                }

            }


            html! { 
                <svg viewBox={format!("0 0 1000 {}", 10.0 + view.lanes.len() as f32 * lane_height)}>
                    { list_features }
                </svg>
            }


        } else {
            html! { "" }
        }


    }

    



}


pub struct GenbankView {
    lanes: Vec<FeatureLane>,
    scale_x: f32
}
impl GenbankView {

    pub fn from(seq: &Seq) -> GenbankView {

        let mut lanes: Vec<FeatureLane> = Vec::new();
        let scale_x = 1000.0 / (seq.len() as f32);

        //Figure out what features to show
        for f in &seq.features {
            if let Ok(bounds) = f.location.find_bounds() {

                //Figure out which lane this would fit
                let mut cur_lane_i = 0;
                loop {
                    //Create new lanes as needed
                    if cur_lane_i == lanes.len() {
                        lanes.push(FeatureLane::new());
                    }
                    let cur_lane = lanes.get_mut(cur_lane_i).unwrap();

                    //See if the feature fits. If so, add feature
                    if cur_lane.max_x < bounds.0 {
                        cur_lane.features.push(f.clone());
                        cur_lane.max_x = bounds.1;
                        break;
                    } else {
                        cur_lane_i += 1;
                        log::debug!("eeep {} {} {}",cur_lane_i, bounds.0, bounds.1);
                    }
                }
            }
        }

        GenbankView {
            lanes: lanes,
            scale_x: scale_x,
        }
    }

}

struct FeatureLane {
    features: Vec<Feature>,
    max_x: i64
}
impl FeatureLane {
    pub fn new() -> FeatureLane {
        FeatureLane { 
            features: Vec::new(), 
            max_x: -100000,
        }
    }
}











////////////////////////////////////////////////////////////
/// Download table data
pub fn download_genbank(gb_data: &Genbank){

    let window = window().expect("no window");
    let document = window.document().expect("should have a document on window");

    let csv_data = &gb_data.data;

    // Creating a Blob for having a csv file format and passing the data with type
    let blob_properties = BlobPropertyBag::new();
    blob_properties.set_type("text/csv");  

    let blob_parts = Array::new();
    blob_parts.push(&JsValue::from_str(csv_data.as_str()));
    let blob = Blob::new_with_buffer_source_sequence_and_options(&blob_parts, &blob_properties).unwrap();

    // Creating an object for downloading url
    let url = web_sys::Url::create_object_url_with_blob(&blob).expect("Could not create url");

    // Creating an anchor(a) tag of HTML
    let a:HtmlElement = document.create_element("a").expect("could not create a").dyn_into().unwrap();

    // Passing the blob downloading url 
    a.set_attribute("href", &url).expect("Could not set attribute");

    // Setting the anchor tag attribute for downloading and passing the download file name
    a.set_attribute("download", "gene.gb").expect("Could not set attribute");

    // Performing a download with click
    a.click();
}

