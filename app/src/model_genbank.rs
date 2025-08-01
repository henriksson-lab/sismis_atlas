
use std::io::Cursor;

use crate::core_model::*;

use my_web_app::Genbank;
use yew::prelude::*;
use gb_io::reader::SeqReader;


//see https://github.com/gamcil/clinker
//https://github.com/art-egorov/lovis4u
//https://crates.io/crates/gb-io




impl Model {

    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_genbank_table(&self, _ctx: &Context<Self>) -> Html {
        if let Some(list_genbank) = &self.current_genbank {

            list_genbank.iter().map(|val| {
                html!{
                    <pre> 
                        { val.data.clone() } 
                    </pre>
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

            let mut list_features = Vec::new();

            let mut cur_y = 10;
            for f in &seq.features {

                let q = format!("{:?}", f.qualifiers);
                

                if let Ok(bounds) = f.location.find_bounds() {
                    let h= html! { 
                        <line x1={format!("{}",bounds.0)} x2={format!("{}",bounds.1)} y1={format!("{}",cur_y)} y2={format!("{}",cur_y)} stroke="red"/>
                    };
                    list_features.push(h);
                    cur_y += 10;

                    let h= html! { 
                        <text x={format!("{}",bounds.0)}  y={format!("{}",cur_y)} fill="red" font-size="3">
                            { q }
                        </text>
                    };
                    list_features.push(h);
                    cur_y += 10;
                }
            }

            html! { 
                <svg viewBox={format!("0 0 {} {}", seq.len(), cur_y+10)}>
                    <line x1=0 x2={format!("{}",seq.len())} y1=1 y2=1 stroke="black"/>
                    { list_features }
                </svg>
            }


        } else {
            html! { "" }
        }


    }
    

}
