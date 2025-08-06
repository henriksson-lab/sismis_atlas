use std::io::{Read};
use std::sync::{Mutex};

use actix_web::web::Data;

use my_web_app::{ClusterRequest, Genbank};
use crate::ServerData;







////////////////////////////////////////////////////////////
/// x    can only read file using rust zip lib. archflow does not support
pub fn query_genbank_asynczip(
    server_data: &Data<Mutex<ServerData>>,
    req: &ClusterRequest
) -> anyhow::Result<Vec<Genbank>> { 


    println!("genbank about to open zip");

    let path_zip = {
        let server_data =server_data.lock().unwrap();
        server_data.path_zip.clone()
    };

    let file = std::fs::File::open(&path_zip).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    //GUT_GENOME277127-scaffold_21_cluster_1
//    let e ="GUT_GENOME277127-scaffold_21_cluster_1".to_string();

    println!("genbank zip is open");

    //Gather all files
    let mut list_out = Vec::new();
    for search_id in &req.cluster_id {

        println!("genbank get one in zip");

        let mut zfile = archive.by_name(search_id);
        let mut out = Vec::new();
        if let Ok(zfile) = &mut zfile {
            println!("genbank reading");
            let _cnt = zfile.read_to_end(&mut out)?;
            println!("genbank reading done");

            let ret = Genbank {
                data: String::from_utf8_lossy(out.as_slice()).to_string()  //support storing as bytes?
            };
            println!("genbank pushing");

            list_out.push(ret);
        } else {
            println!("Failed to get one genbank file");
        }
    }

    Ok(list_out)
}
        


