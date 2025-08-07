use std::fs::File;
use std::path::{PathBuf};
use std::io::{BufRead, BufReader};

use my_web_app::{Cluster, ConfigFile};
use rusqlite::{Connection, Statement};

use serde_rusqlite::*;


////////////////////////////////////////////////////////////
/// x
fn insert_genbank_sql(stmt: &mut Statement, id: &String, data: &String){
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


////////////////////////////////////////////////////////////
/// x
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
                    insert_genbank_sql(&mut stmt_insert, &pname, &b);

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
        insert_genbank_sql(&mut stmt_insert, &pname, &b);
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











////////////////////////////////////////////////////////////
/// x
/// this is extremely slow compared to the sqlite import-from-tsv command!!!!. don't use it. run commands below instead
/// 
/// .mode tabs
/// .import clusters.tsv clusters
/// CREATE INDEX ind
/// ON clusters (cluster_id);
/// 
pub fn convert_clusters_sqlite(config_file: &ConfigFile) -> anyhow::Result<()> {

    // Open SQL database
    let path_store = std::path::Path::new(&config_file.data);
    let path_sql = path_store.join(std::path::Path::new("clusters.sqlite"));
    let path_tsv = path_store.join(std::path::Path::new("clusters.tsv"));

    if path_sql.exists() {
        println!("clusters sqlite already exists");
        return Ok(());
    } else {
        println!("making clusters.sqlite");
    }

    let conn = Connection::open(&path_sql).expect("Could not open SQL database");

    conn.execute(
        "CREATE TABLE clusters (
            gcf_id TEXT NOT NULL,
            sequence_id TEXT NOT NULL,
            cluster_id TEXT NOT NULL,
            start TEXT NOT NULL,
            end TEXT NOT NULL,
            average_p TEXT NOT NULL,
            GTDB_phylum TEXT NOT NULL,
            GTDB_species TEXT NOT NULL
        )",
        (), // empty list of parameters.
    ).expect("failed to create table"); // PRIMARY KEY -- can add later



    let mut stmt_insert = conn
        .prepare("INSERT INTO clusters VALUES (?1,?2,?3,?4,?5,?6,?7,?8)")
        .expect("Failed to prepare statement");  //note, in sqlite = is fine and required to use the index it seems. other databases need LIKE


    let file_tsv = File::open(&path_tsv).expect("cluster.tsv missing");
    let buf_reader = BufReader::new(file_tsv);
    let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(buf_reader);
    for result in reader.deserialize() {
        let record: Cluster = result.unwrap();

        stmt_insert.execute(
            (
                &record.gcf_id, 
                &record.sequence_id, 
                &record.cluster_id,
                &record.start,
                &record.end,
                &record.average_p,
                &record.gtdb_phylum,
                &record.gtdb_species
            ),
        ).expect("Failed to insert");
    }

    println!("clusters.tsv insert done");

    conn.execute(
        "CREATE INDEX ind_clusters
             ON clusters (cluster_id);
        ",
        (), // empty list of parameters.
    ).expect("failed to index table, 1");


    conn.execute(
        "CREATE INDEX ind_clusters2
             ON clusters (gcf_id);
        ",
        (), // empty list of parameters.
    ).expect("failed to index table, 2");


    println!("sqlite clusters finalized");

    Ok(())
}
