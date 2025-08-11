use yew::prelude::*;

use crate::core_model::*;
use crate::component_umap::UmapView;




////////////////////////////////////////////////////////////
/// If condition is met, return "selected", otherwise "". For OPTION
pub fn selected_if(cond: bool) -> String {
    if cond {
        "selected".to_string()
    } else {
        "".to_string()
    }
}   // can do true false https://yew.rs/docs/concepts/html 



impl Model {


    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_atlas_page(&self, ctx: &Context<Self>) -> Html {

        let on_cell_hovered = Callback::from(move |_name: Option<String>| {
        });

        let on_cell_clicked= ctx.link().callback(move |name: Vec<String>| {
            Msg::ClickSequence(name)
        });


        html! {

            <div>
            
                <div class="App-divider">
                    {"FLExo UMAP"}
                </div>

                <UmapView on_cell_hovered={on_cell_hovered} on_cell_clicked={on_cell_clicked}/> //// we really do not want to re-render this if needed! how to avoid?

                <div class="App-divider">
                    {"Selected UMAP point(s): metadata"}
                </div>
                { self.view_cluster_table(ctx) }
                <div class="App-divider">
                    {"Selected metadata row: Pfam domains"}
                </div>
                { self.view_genbank_svgs(ctx) }
                <div class="App-divider">
                    {"Selected metadata row: GenBank"}
                </div>
                
                { self.view_genbank_table(ctx) }


            </div>
        }
    }



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
                                <td> { c.gcf_id.clone() } </td>
                                <td> { c.sequence_id.clone() } </td>
                                <td>
                                    <a onclick={on_click_cluster} style="color:blue; cursor: pointer;">
                                        { c.cluster_id.clone() }
                                    </a>
                                </td>
                                <td> { c.start.clone() } </td>
                                <td> { c.end.clone() } </td>
                                <td> { c.average_p.clone() } </td>
                                <td> { c.gtdb_phylum.clone() } </td>
                                <td> { c.gtdb_species.clone() } </td>
                            </tr>
                }
            }).collect::<Html>();

            html! {
                <div style="overflow-x:auto;">
                <table>
                    <tr>
                        <th> {"GCF_id"} </th>
                        <th> {"sequence_id"} </th>
                        <th> {"cluster_id"} </th>
                        <th> {"start"} </th>
                        <th> {"end"} </th>
                        <th> {"average_p"} </th>
                        <th> {"GTDB_phylum"} </th>
                        <th> {"GTDB_species"} </th>
                    </tr>
                    { list_rows }
                </table>
                </div>
            }

        } else {
            html! { "" }
        }

    }


}
