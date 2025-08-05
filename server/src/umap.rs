use std::{fs::File, io::BufReader, path::Path};

use my_web_app::{UmapData, UmapMetadata};
use serde::{Deserialize, Serialize};

use crate::ConfigFile;



////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct CsvSeqMeta {

    #[serde(rename(deserialize = "GCF_ID"))]
    pub gcf_id: String,

    pub cluster_id: String,
    pub x: f32,
    pub y: f32,

    #[serde(rename(deserialize = "Seurat"))]
    pub seurat: String,

    #[serde(rename(deserialize = "VFDB"))]
    pub vfdb: String,
}


////////////////////////////////////////////////////////////
/// NOTE!!! assumed same order as in umap
pub fn load_umap_data(config_file: &ConfigFile) -> (UmapData, UmapMetadata) {

    let path_meta = config_file.data.join(Path::new("umap_coords_plus_metadata.tsv"));
    let f_meta = File::open(path_meta).expect("Could not open umap_coords_plus_metadata.tsv");
    let reader = BufReader::new(f_meta);


    let mut list_seurat = Vec::new();
    let mut list_vfdb = Vec::new();
    //let mut list_gfc_id = Vec::new(); /// too large

    let mut ids: Vec<String> = Vec::new();

    let mut vertices: Vec<f32> = Vec::new();
    let mut num_points = 0;


    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(reader);
    for result in reader.deserialize() {
        let record: CsvSeqMeta = result.unwrap();

        //General metadata
        list_seurat.push(record.seurat);
        list_vfdb.push(record.vfdb);
        //list_gfc_id.push(record.gcf_id);

        //Point data
        vertices.push(record.x);
        vertices.push(record.y);
        ids.push(record.cluster_id);
        num_points += 1;
    }

    //Factorize strings to reduce data size
    let mut umeta = UmapMetadata::new();
    umeta.add_and_factorize(&"Seurat".to_string(), &list_seurat);
    umeta.add_and_factorize(&"VFDB".to_string(), &list_vfdb);
    //umeta.add_and_factorize(&"GCF_ID".to_string(), &list_gfc_id);



    //Figure out UMAP point range
    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    for i in 0..num_points {
        let px = *vertices.get(i*2+0).unwrap();
        let py = *vertices.get(i*2+1).unwrap();

        max_x = max_x.max(px);
        max_y = max_y.max(py);
        min_x = min_x.min(px);
        min_y = min_y.min(py);
    }

    //Pack point data
    let umap_xy = UmapData {
        num_point: num_points, 
        data: vertices,
        ids: ids,

        max_x: max_x,
        max_y: max_y,
        min_x: min_x,
        min_y: min_y,
    };

    (umap_xy, umeta)

}



