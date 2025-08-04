

/*
 sequence_meta.tsv   is arbitrary data for each sequence
 */

use my_web_app::UmapMetadata;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};

use crate::ConfigFile;





////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct CsvSeqMeta {
    pub sequence_id: String,
    pub source: String,
    pub other: String,
}


////////////////////////////////////////////////////////////
/// NOTE!!! assumed same order as in umap
pub fn load_sequence_meta(config_file: &ConfigFile) -> UmapMetadata {

    let path_meta = config_file.data.join(Path::new("sequence_meta.tsv"));
    let f_meta = File::open(path_meta).expect("Could not open sequence_meta.tsv");
    let reader = BufReader::new(f_meta);


    let mut list_source = Vec::new();
    let mut list_other = Vec::new();


    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(reader);
    for result in reader.deserialize() {
        let record: CsvSeqMeta = result.unwrap();
        list_source.push(record.source);
        list_other.push(record.other);
    }

    let mut umeta = UmapMetadata::new();
    umeta.add_and_factorize(&"source".to_string(), &list_source);
    umeta.add_and_factorize(&"other".to_string(), &list_other);

    umeta

}



