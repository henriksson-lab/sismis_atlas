use std::path::{PathBuf};
use std::io::{BufRead, BufReader};

use my_web_app::ConfigFile;
use rusqlite::{Connection, Statement};



fn insert_sql(stmt: &mut Statement, id: &String, data: &String){
    stmt.execute(
        (&id, &data),
    ).expect("Failed to insert");

    //println!("f: {}",data); //check that \n is handled!!
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


pub fn convert_genbank_sqlite(fname: &PathBuf, config_file: &ConfigFile) -> anyhow::Result<()> {

    let mut done_files = 0;

    // Open SQL database
    let path_store = std::path::Path::new(&config_file.data);
    let path_sql = path_store.join(std::path::Path::new("genbank.sqlite"));

    if path_sql.exists() {
        println!("genbank sqlite already exists");
        return Ok(());
    }

    let conn = Connection::open(&path_sql).expect("Could not open SQL database");

    conn.execute(
        "CREATE TABLE genbank (
            cluster_id   TEXT NOT NULL, 
            data         TEXT NOT NULL
        )",
        (), // empty list of parameters.
    ).expect("failed to create table"); // PRIMARY KEY -- can add later

    let mut stmt_insert = conn.prepare("INSERT INTO genbank VALUES (?1,?2)").expect("Failed to prepare statement");  //note, in sqlite = is fine and required to use the index it seems. other databases need LIKE


    let f = std::fs::File::open(fname)?;
    let mut reader = BufReader::new(f);

    let mut b=String::new();//BytesMut::new();
    let mut name: Option<String> = None;

    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;
        if len>0 {
            if line.starts_with("//") {
                //We have reached the end of this genbank. put it in the list
                b.push_str(line.as_str());
                if let Some(pname) = name {
                    //println!("One gbk done {}", pname);
                    insert_sql(&mut stmt_insert, &pname, &b);

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
                b.push_str(line.as_str());
            }
        } else { 
            //we are done
            break;
        }

    }

    //If there is a final entry
    if let Some(pname) = name {
        println!("One final gbk done");
        insert_sql(&mut stmt_insert, &pname, &b);
        b.clear();
    }

    println!("# files done total: {}, indexing",done_files);

    conn.execute(
        "CREATE INDEX ind
             ON genbank (cluster_id);
        ",
        (), // empty list of parameters.
    ).expect("failed to index table");

    println!("sqlite genbank finalized");

    Ok(())
}
