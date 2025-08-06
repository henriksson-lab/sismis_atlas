use std::sync::{Mutex};

use actix_web::web::Data;

use my_web_app::{ClusterRequest, Genbank};
use crate::ServerData;



use rusqlite::{Connection, OpenFlags};


////////////////////////////////////////////////////////////
/// x
pub async fn query_genbank_sqlite(
    server_data: &Data<Mutex<ServerData>>,
    req: &ClusterRequest
) -> anyhow::Result<Vec<Genbank>> { 

    println!("query_genbank_sqlite");

    let config_file = {
        let server_data =server_data.lock().unwrap();
        server_data.config_file.clone()
    };

    let path_store = std::path::Path::new(&config_file.data);
    let path_sql = path_store.join(std::path::Path::new("genbank.sqlite"));
    let conn = Connection::open_with_flags(&path_sql, OpenFlags::SQLITE_OPEN_READ_ONLY).expect("Could not open SQL database");
        
    let mut stmt = conn.prepare("SELECT * from genbank where cluster_id = ?1")?;  //note, in sqlite = is fine and required to use the index it seems. other databases need LIKE
     
    let mut all_rows = Vec::new();
    for sequence_id in &req.cluster_id {

        println!("Making query");

        let rows = stmt.query_map([sequence_id], |row| {
            let out = Genbank {
                //cluster_id: row.get(0)?,
                data: row.get(1)?,
            };
            Ok(out)
        })?;

        println!("done query");


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
        


// select * from genbank where cluster_id = '1074149.SAMN02436352.AGLG01000014_cluster_1';
//cluster_id: ["1074149.SAMN02436352.AGLG01000014_cluster_1"] })


// missing: 331648.SAMN06075354.CP019344_cluster_5


// (base) mahogny@beagle:~/github/floriansite/testdata$ unzip -p genbank.zip 331648.SAMN06075354.CP019344_cluster_5
// file #1:  bad zipfile offset (local header sig):  4037

// unzip -c genbank.zip GUT_GENOME286869-scaffold_4_cluster_1
