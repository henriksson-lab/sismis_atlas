
use actix_web::web::Data;
use my_web_app::{Cluster, SequenceRequest};
use rusqlite::{Result};
use std::sync::Mutex;

use crate::ServerData;




/*
CREATE TABLE IF NOT EXISTS "clusters"(
"sequence_id" TEXT, "cluster_id" TEXT, "start" TEXT, "end" TEXT,
 "average_p" TEXT, "max_p" TEXT, "proteins" TEXT, "domains" TEXT,
 "type" TEXT, "filepath" TEXT);
 */

////////////////////////////////////////////////////////////
/// Get entries from the strain table given search criteria
pub fn get_sequence_sql(
    server_data: &Data<Mutex<ServerData>>,
    req: &SequenceRequest //String
) -> Result<Vec<Cluster>> {
     
//    let _search_id = req.

//    let q = "SELECT * from clusters where sequence_id LIKE ?1".to_string();  // WHERE clusterid='foo'  _search_id
    
    let server_data =server_data.lock().unwrap();
        
    let mut stmt = server_data.conn.prepare("SELECT * from clusters where sequence_id LIKE ?1")?;
     
    let rows = stmt.query_map([req.sequence_id.clone()], |row| {
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

    let mut all_rows = Vec::new();
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
    return Ok(all_rows);
//    Ok(None)
}
        








// .mode tabs
// .import data.tsv my-table

/*

/husky/carroll/exotoxin_atlas

all:
	echo -e "foo1\tfoo2\tfoo3\tfoo4\tfoo5\tfoo6\tfoo7\tfoo8\tfoo9\tfoo10" > headercluster.tsv.part
	head -n 1000 final_C11_W10_FET0.05_motus3.clusters.tsv >>  headercluster.tsv.part

	rm newtab.sqlite
	sqlite3 newtab.sqlite
	.mode tabs
	.import headercluster.tsv.part clusters



	sqlite3 newtab.sqlite
	.mode tabs
	.import headercluster.tsv.part clusters


    cluster_id

*/


/*

query: it should be by gene cluster

It should be a unique column, each row is a gene cluster


query by: GUT_GENOME002323-scaffold_1_cluster_1

ANAN16-1_SAMN03842430_METAG_005352-scaffold_3	ANAN16-1_SAMN03842430_METAG_005352-scaffold_3_cluster_1	5987	6682	0.949295150147262	0.949295150147262	ANAN16-1_SAMN03842430_METAG_005352-scaffold_3_8	PF00005;PF02463;PF13304	Unknown	../out_bad2-1/ANAN16-1_SAMN03842430_METAG_005352_out/ANAN16-1_SAMN03842430_METAG_005352.clusters.tsv
GUT_GENOME002323-scaffold_1	GUT_GENOME002323-scaffold_1_cluster_1	122843	124534	0.958102639632251	0.958102639632251	GUT_GENOME002323-scaffold_1_113	PF00005;PF00664	Unknown	../out_bad2-1/GUT_GENOME002323_out/GUT_GENOME002323.clusters.tsv

*/





