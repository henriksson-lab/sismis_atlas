use std::path::{PathBuf};
use std::io::{BufRead, BufReader, Read};
use std::sync::Mutex;

use actix_web::web::BytesMut;
use actix_web::web::BufMut;
use actix_web::web::Data;

use my_web_app::{ClusterRequest, Genbank};
use tokio::fs::File;
use crate::ServerData;

use archflow::{
 compress::FileOptions, compress::tokio::archive::ZipArchive, compression::CompressionMethod,
 error::ArchiveError,
};



////////////////////////////////////////////////////////////
/// x
fn parse_name(line: &String) -> Option<String> {
    let line = &line["LOCUS       ".len()..];
    let mut iter = line.split_ascii_whitespace();
    let name = iter.next().expect("Could not get name");
    println!("  name: {}",name);
    Some(name.to_string())
//  "LOCUS       GUT_GENOME002323-scaffold_16_cluster_1"  then space etc
}

pub async fn convert_genbank(fname: &PathBuf, fname_zip: &PathBuf) -> Result<(), ArchiveError> {

    let mut done_files = 0;

    let file_zip = File::create(fname_zip).await?;
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
                b.put(line.as_bytes());
                if let Some(pname) = name {
                    //println!("One gbk done");
                    archive.append(pname.as_str(), &options, &mut b.as_ref()).await?;
                    name = None;
                    b.clear();

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

        done_files += 1;
        if done_files%10 == 0 {
            println!("# files done: {}",done_files);
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





////////////////////////////////////////////////////////////
/// x
pub fn query_genbank(
    server_data: &Data<Mutex<ServerData>>,
    req: &ClusterRequest
) -> anyhow::Result<Vec<Genbank>> { //>Result<Vec<u8>,Error>


    let server_data =server_data.lock().unwrap();
    let file = std::fs::File::open(&server_data.path_zip).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    //GUT_GENOME277127-scaffold_21_cluster_1

//    let e ="GUT_GENOME277127-scaffold_21_cluster_1".to_string();


    //Gather all files
    let mut list_out = Vec::new();
    for search_id in &req.cluster_id {

        let mut zfile = archive.by_name(search_id);
        let mut out = Vec::new();
        if let Ok(zfile) = &mut zfile {
            //let mut reader = BufReader::new(zfile);
            let _cnt = zfile.read_to_end(&mut out)?;

            let ret = Genbank {
                data: String::from_utf8_lossy(out.as_slice()).to_string()
            };
            list_out.push(ret);
        } else {
            println!("Failed to get one genbank file");
        }
    }

    Ok(list_out)
//    Ok("booo".to_string())
}
        
