use std::fs::File;
use std::path::{PathBuf};
use std::io::{BufRead, BufReader};
use std::io::Write;

use zip::write::SimpleFileOptions;
use zip::ZipWriter;



fn write_zip(zip: &mut ZipWriter<File>, fname: &String, content: &Vec<Vec<u8>> ) -> anyhow::Result<()> {
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .large_file(true);

    zip.start_file(fname, options).expect("Failed to start new zip file entry");
    for c in content {
        zip.write_all(c).expect("Faile to write to zip file entry");
    }
    Ok(())
}



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

pub fn convert_genbank_rszip(fname: &PathBuf, fname_zip: &PathBuf) -> anyhow::Result<()> {

    let mut done_files = 0;

    let file = std::fs::File::create(fname_zip)?;
    let mut zip = zip::ZipWriter::new(file);

    let f = std::fs::File::open(fname).expect("Could not open genbank");
    let mut reader = BufReader::new(f);

    let mut b=Vec::new();
    let mut name: Option<String> = None;

    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;
        if len>0 {
            if line.starts_with("//") {
                //We have reached the end of this genbank. put it in the list
                b.push(line.as_bytes().to_vec());
                if let Some(pname) = name {
                    //println!("One gbk done {}", pname);
                    write_zip(&mut zip, &pname, &b).expect("Failed to write_zip");

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
                b.push(line.as_bytes().to_vec());
            }
        } else { 
            //we are done
            break;
        }

    }

    //If there is a final entry
    if let Some(pname) = name {
        println!("One final gbk done");
        write_zip(&mut zip, &pname, &b).expect("Failed to write_zip");
        b.clear();
    }

    println!("# files done total: {}, finalizing",done_files);

    zip.finish().expect("Failed to finish zip");

    println!("Archive finalized");

    Ok(())
}
