
use std::{collections::{BTreeMap, HashMap}, path::PathBuf};

use serde::{Deserialize, Serialize};



////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Cluster {
    pub sequence_id: String,
    pub cluster_id: String,
    pub start: String,
    pub end: String,
    pub average_p: String,
    pub max_p: String,
    pub proteins: String,
    pub domains: String,
    pub type2: String,
    pub filepath: String,
}


////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseColumnMeta {
    pub name: String,
    pub list_levels: Vec<String>, //0...n, different factor levels
    pub map_levels: BTreeMap<String, u8>, 
    pub values: Vec<u8> //if we need more levels, should add specialized types
}

// #[serde(skip)]  // if we want to restore map_levels ourselves


impl DatabaseColumnMeta {

    ////////////////////////////////////////////////////////////
    /// Constructor
    pub fn new(name: &String) -> DatabaseColumnMeta {
        DatabaseColumnMeta {
            name: name.clone(),
            list_levels: Vec::new(),
            map_levels: BTreeMap::new(),
            values: Vec::new(),
        }
    }

    ////////////////////////////////////////////////////////////
    /// Convert list of strings into factors; update this object to keep levels
    pub fn factorize(&mut self, inlist: &Vec<String>) {
        self.list_levels.clear();
        self.map_levels.clear();

        let mut outlist = Vec::new();
        outlist.reserve(inlist.len());
        for e in inlist {
            if let Some(i) = self.map_levels.get(e) {
                outlist.push(*i as u8);
            } else {
                let i = self.list_levels.len() as u8;
                self.map_levels.insert(e.clone(), i);
                self.list_levels.push(e.clone());
                outlist.push(i);
            }
        }
        self.values=outlist;
    }
}



////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct UmapMetadata {
    pub colorings: HashMap<String, DatabaseColumnMeta>
}

impl UmapMetadata {

    pub fn new() -> UmapMetadata {
        UmapMetadata {
            colorings: HashMap::new()
        }
    }


    pub fn add_and_factorize(&mut self, name: &String, values: &Vec<String>) {
        let mut col = DatabaseColumnMeta::new(&name);
        col.factorize(values);
        self.colorings.insert(name.clone(), col);
    }
}






////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct UmapData {
    pub num_point: usize,
    pub data: Vec<f32>,
    pub ids: Vec<String>,

    pub max_x: f32,
    pub max_y: f32,
    pub min_x: f32,
    pub min_y: f32,
}



////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Genbank {
    pub data: String,
}



////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct ClusterRequest {
    pub cluster_id: Vec<String>,
}




////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct SequenceRequest {
    pub sequence_id: Vec<String>,
}



////////////////////////////////////////////////////////////
/// x
#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigFile {
    pub data: PathBuf,
    pub bind: String,
    pub port: u16
}





