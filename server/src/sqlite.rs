
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
        














/*


////////////////////////////////////////////////////////////
/// x
fn parse_name(line: &String) -> Option<String> {
    let line = &line["LOCUS       ".len()..];
    let mut iter = line.split_ascii_whitespace();
    let name = iter.next().expect("Could not get name");
    //println!("  name: {}",name);
    Some(name.to_string())
//  "LOCUS       GUT_GENOME002323-scaffold_16_cluster_1"  then space etc
}

pub fn convert_genbank_to_sqlite(fname: &PathBuf, fname_sqlite: &PathBuf) -> Result<(), ArchiveError> {

    let mut done_files = 0;









    let file_zip = File::create(fname_sqlite).await?;
    let options = FileOptions::default().compression_method(CompressionMethod::Deflate());
    let mut archive = ZipArchive::new_streamable(file_zip);

    let f = std::fs::File::open(fname)?;
    let mut reader = BufReader::new(f);

    let mut b=BytesMut::new();
    let mut name: Option<String> = None;

    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;
        if len>0 {
            if line.starts_with("//") {
                //We have reached the end of this genbank. put it in the list
                b.put(line.as_bytes());
                if let Some(pname) = name {
                    //println!("One gbk done {}", pname);
                    archive.append(pname.as_str(), &options, &mut b.as_ref()).await?;

                    //Start with the next one
                    name = None;
                    b.clear();

                    done_files += 1;
                    if done_files%1000 == 0 {
                        println!("# files done: {}",done_files);
                    }

                } else {
                    panic!("Missing name")
                }
            } else {
                //print!("{}", &line);
                if name.is_none() {
                    name = parse_name(&line);
                }
                b.put(line.as_bytes());
            }
        } else { 
            //we are done
            break;
        }

    }

    //If there is a final entry
    if let Some(pname) = name {
        println!("One final gbk done");
        archive.append(pname.as_str(), &options, &mut b.as_ref()).await?;
        b.clear();
    }

    println!("# files done total: {}, finalizing",done_files);

    archive.finalize().await?;

    println!("Archive finalized");

    Ok(())
}



*/
