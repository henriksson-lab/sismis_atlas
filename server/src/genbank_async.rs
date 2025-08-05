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
pub async fn query_genbank(
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
//    Ok("booo".to_string())
}
        








// unzip -c genbank.zip GUT_GENOME286869-scaffold_4_cluster_1

/*







/// Pipe streams are blocking, we need separate threads to monitor them without blocking the primary thread.
fn child_stream_to_vec<R>(mut stream: R) -> Arc<Mutex<Vec<u8>>>
where
    R: Read + Send + 'static,
{
    let out = Arc::new(Mutex::new(Vec::new()));
    let vec = out.clone();
    thread::Builder::new()
        .name("child_stream_to_vec".into())
        .spawn(move || loop {
            let mut buf = [0];
            match stream.read(&mut buf) {
                Err(err) => {
                    println!("{}] Error reading from stream: {}", line!(), err);
                    break;
                }
                Ok(got) => {
                    if got == 0 {
                        break;
                    } else if got == 1 {
                        vec.lock().expect("!lock").push(buf[0])
                    } else {
                        println!("{}] Unexpected number of bytes: {}", line!(), got);
                        break;
                    }
                }
            }
        })
        .expect("!thread");
    out
}



*/