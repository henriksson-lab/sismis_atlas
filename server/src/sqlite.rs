
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
     
    println!("get_sequence_sql {:?}", req);

    let mut all_rows = Vec::new();
    for sequence_id in &req.cluster_id {

        let rows = stmt.query_map([sequence_id], |row| {
            println!("got one row");
            let out = Cluster {
                gcf_id: row.get(0)?,
                sequence_id: row.get(1)?,
                cluster_id: row.get(2)?,
                start: row.get(3)?,
                end: row.get(4)?,
                average_p: row.get(5)?,
                max_p: row.get(6)?,
                sismis_type: row.get(7)?,
                gtdb_phylum: row.get(8)?,
                gtdb_species: row.get(9)?,
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

    println!("done {:?}", all_rows);

    return Ok(all_rows);
}
        

////////////////////////////////////////////////////////////
/// Get GCF ids from the strain table given search criteria
pub fn get_gcfid_for_sequence_sql(
    server_data: &Data<Mutex<ServerData>>,
    req: &ClusterRequest //String
) -> Result<Vec<String>> {
     
    let server_data =server_data.lock().unwrap();
        
    let mut stmt = server_data.conn.prepare("SELECT DISTINCT gcf_id from clusters where cluster_id = ?1")?;  //note, in sqlite = is fine and required to use the index it seems. other databases need LIKE
     

    let mut all_rows = Vec::new();
    for sequence_id in &req.cluster_id {

        let rows = stmt.query_map([sequence_id], |row| {
            Ok(row.get(0)?)
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


////////////////////////////////////////////////////////////
/// Get entries from the strain table given search criteria
pub fn get_sequence_allgcf_sql(
    server_data: &Data<Mutex<ServerData>>,
    req: &ClusterRequest //String
) -> Result<Vec<Cluster>> {
     

    //First get all GCF ids for cluster_ids
    let list_gcf= get_gcfid_for_sequence_sql(&server_data, &req)?;

    //
    let server_data =server_data.lock().unwrap();        
    let mut stmt = server_data.conn.prepare("SELECT * from clusters where gcf_id = ?1")?;  //note, in sqlite = is fine and required to use the index it seems. other databases need LIKE     


    use serde_rusqlite::*;


    let mut all_rows = Vec::new();
    for sequence_id in &list_gcf {


        let res2 = from_rows::<Cluster>(stmt.query([sequence_id]).unwrap());
        for row in res2 {
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
        
