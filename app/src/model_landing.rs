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
    pub fn view_landing_page(&self, ctx: &Context<Self>) -> Html {

        let on_cell_hovered= ctx.link().callback(move |name: Option<String>| {
            Msg::HoverSequence(name)
        });

        let on_cell_clicked= ctx.link().callback(move |name: Option<String>| {
            Msg::ClickSequence(name)
        });


        html! {

            <div>

                <UmapView on_cell_hovered={on_cell_hovered} on_cell_clicked={on_cell_clicked}/> //// we really do not want to re-render this if needed! how to avoid?
                <p>
                    { format!("{}", if let Some(c) = &self.hover_sequence {c.clone()} else {"".to_string()} )  }
                </p>

                <div class="App-divider">
                    {"Cluster metadata"}
                </div>
                { self.view_cluster_table(ctx) }
                <div class="App-divider">
                    {"Genbank representations"}
                </div>
                { self.view_genbank_svgs(ctx) }
                <div class="App-divider">
                    {"Genbank sequences"}
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
