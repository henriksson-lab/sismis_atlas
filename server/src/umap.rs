use std::{fs::File, io::BufReader, path::Path};

use my_web_app::UmapData;
use serde::{Deserialize, Serialize};






////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct CsvUmapPoint {
    pub sequence_id: String,
    pub x: f32,
    pub y: f32,
}




pub fn load_umap_data() -> UmapData {


    let path_meta = Path::new("testdata/umap_coord.tsv");
    let f_meta = File::open(path_meta).expect("Could not open btyperdb_include");
    let reader = BufReader::new(f_meta);

    /////////// Other metadata from CSV-file
    let mut ids: Vec<String> = Vec::new();
    let mut vertices: Vec<f32> = Vec::new();
    let mut num_points = 0;


    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(reader);
    for result in reader.deserialize() {
        let record: CsvUmapPoint = result.unwrap();
        vertices.push(record.x);
        vertices.push(record.y);
        ids.push(record.sequence_id);
        num_points += 1;
    }

/* 

    // This list of vertices
    let num_points = 10;
    let mut vertices: Vec<f32> = vec![];
    let mut ids: Vec<String> = Vec::new();

    use rand::Rng;
    let mut rng = rand::rng();
    for i in 0..(num_points*2) {
        let v = rng.random_range(0.0..1023.0);//.round();
//        let v = rng.random_range(-1.0..1.0);
        vertices.push(v);
        ids.push(format!("c{}",i));

        //vertices.len()/2,
    }*/
 
    UmapData {
        num_point: num_points, 
        data: vertices,
        ids: ids,
    }



}


