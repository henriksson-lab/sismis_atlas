
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
pub struct Genbank {
    pub data: String,
}



////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct ClusterRequest {
    pub cluster_id: String,
}

