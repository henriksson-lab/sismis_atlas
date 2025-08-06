use std::process::Stdio;
use tokio::process::Command;
use tokio::io::{AsyncReadExt, BufReader};


//use std::io::{BufRead, BufReader, Read};
use std::sync::{Mutex};

use actix_web::web::Data;

use my_web_app::{ClusterRequest, Genbank};
use crate::ServerData;



////////////////////////////////////////////////////////////
/// x
pub async fn query_genbank_unzip(
    server_data: &Data<Mutex<ServerData>>,
    req: &ClusterRequest
) -> anyhow::Result<Vec<Genbank>> { 

    println!("genbank about to open zip");

    let path_zip = {
        let server_data =server_data.lock().unwrap();
        server_data.path_zip.clone()
    };


    let mut list_out = Vec::new();
    for search_id in &req.cluster_id {


        //Pipe to sort, then to given file
        let mut cmd = Command::new("unzip");
        cmd.arg("-p");
        cmd.arg(&path_zip);
        cmd.arg(&search_id);

        println!("{:?}", cmd);

        let mut process = cmd
            //.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn().
            expect("failed to start sorter");


        let mut buffer = Vec::new();
        let stdout = process.stdout.take().expect("no stdout");

        BufReader::new(stdout)
            .read_to_end(&mut buffer)
            .await?;

        println!("buffer len {}", buffer.len());

        let ret = Genbank {
            data: String::from_utf8_lossy(buffer.as_slice()).to_string()  //support storing as bytes?  TODO
        };

        println!("{:?}", ret);

        println!("genbank pushing");
        if !ret.data.starts_with("Caution") && buffer.len()>0 {
            list_out.push(ret);
        }
    }

    println!("ret");
    Ok(list_out)
}
        




// missing: 331648.SAMN06075354.CP019344_cluster_5


// (base) mahogny@beagle:~/github/floriansite/testdata$ unzip -p genbank.zip 331648.SAMN06075354.CP019344_cluster_5
// file #1:  bad zipfile offset (local header sig):  4037

// unzip -c genbank.zip GUT_GENOME286869-scaffold_4_cluster_1
