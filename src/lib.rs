
use std::{collections::{BTreeMap, HashMap, HashSet}, path::PathBuf};

use serde::{Deserialize, Serialize};



////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Cluster {
    pub gcf_id: String,
    pub sequence_id: String,
    pub cluster_id: String,
    pub start: String,
    pub end: String,
    pub average_p: String,
    pub max_p: String,
    pub sismis_type: String,

    #[serde(rename = "GTDB_phylum")]
    pub gtdb_phylum: String,
    
    #[serde(rename = "GTDB_species")]
    pub gtdb_species: String,
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

        //Gather all possible levels
        let mut set_colors = HashSet::new();
        for e in inlist {
            set_colors.insert(e.clone());
        }
        self.list_levels = set_colors.iter().cloned().collect();
        self.list_levels.sort_by(|a, b| human_sort::compare(a.as_str(),b.as_str()));
//        self.list_levels = list_levels;            

        //Sort levels, set up map
        for (i,e) in self.list_levels.iter().enumerate() {
            self.map_levels.insert(e.clone(), i as u8);
        }

        //Map to factors
        let mut outlist = Vec::new();
        outlist.reserve(inlist.len());
        for e in inlist {
            let i = self.map_levels.get(e).unwrap();
            outlist.push(*i as u8);
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
    pub ids: Vec<String>, //cluster_id

    //pub gcf_ids: Vec<String>, ////if we want to save space: could turn into into int, then convert from/to GCF0000001. can also skip serialization to keep on server-side

    pub max_x: f32,
    pub max_y: f32,
    pub min_x: f32,
    pub min_y: f32,
}



////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize, Clone)]
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
/// x
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigFile {
    pub data: PathBuf,
    pub bind: String,
    pub port: u16
}





