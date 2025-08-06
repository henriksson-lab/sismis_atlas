use std::path::{PathBuf};
use std::io::{BufRead, BufReader};

use actix_web::web::BytesMut;
use actix_web::web::BufMut;

use tokio::fs::File;


use archflow::{
 compress::FileOptions, compress::tokio::archive::ZipArchive, compression::CompressionMethod,
 error::ArchiveError,
};


////////// TODO

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

pub async fn convert_genbank_sqlite(fname: &PathBuf, fname_zip: &PathBuf) -> Result<(), ArchiveError> {

    let mut done_files = 0;

    let file_zip = File::create(fname_zip).await?;
    let options = FileOptions::default().compression_method(CompressionMethod::Deflate()).large_file(true);
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
