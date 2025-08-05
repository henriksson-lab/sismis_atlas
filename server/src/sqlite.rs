
use actix_web::web::Data;
use my_web_app::{Cluster, ClusterRequest};
use rusqlite::{Result};
use std::sync::Mutex;

use crate::ServerData;



////////////////////////////////////////////////////////////
/// Get entries from the strain table given search criteria
pub fn get_sequence_sql(
    server_data: &Data<Mutex<ServerData>>,
    req: &ClusterRequest //String
) -> Result<Vec<Cluster>> {
     
    let server_data =server_data.lock().unwrap();
        
    let mut stmt = server_data.conn.prepare("SELECT * from clusters where cluster_id = ?1")?;  //note, in sqlite = is fine and required to use the index it seems. other databases need LIKE
     

    let mut all_rows = Vec::new();
    for sequence_id in &req.cluster_id {

        let rows = stmt.query_map([sequence_id], |row| {
            let out = Cluster {
                sequence_id: row.get(0)?,
                cluster_id: row.get(1)?,
                start: row.get(2)?,
                end: row.get(3)?,
                average_p: row.get(4)?,
                max_p: row.get(5)?,
                proteins: row.get(6)?,
                domains: row.get(7)?,
                type2: row.get(8)?,
                filepath: row.get(9)?,
            };
            Ok(out)
        })?;

        for row in rows {
            match row {
                Ok(row) => {
                    all_rows.push(row);
                },
                Err(e) => {
                    eprintln!("Error: {e:?}")
                }
            }
        }        
    }
    return Ok(all_rows);
}
        


