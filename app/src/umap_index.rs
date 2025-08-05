use my_web_app::UmapData;
use rstar::{primitives::GeomWithData, RTree};

/// This data structure takes quite a while to build... 
/// do it asynch, after rendering, or provide a simpler algo?

use std::collections::HashMap;


type Sector = (i32,i32);

type UmapPoint = (f32,f32,usize);



pub struct UmapPointIndex {
    sectors: HashMap<Sector, Vec<UmapPoint>>,
    max_dist: f32
}
impl UmapPointIndex {

    pub fn new() -> UmapPointIndex {
        UmapPointIndex {
            sectors: HashMap::new(),
            max_dist: 0.0
        }
    }

    pub fn clear(&mut self) {
        self.sectors.clear();

    }

    pub fn get_sector_id(&self, x: f32, y: f32) -> Sector {
        (
            ((x as f32)/self.max_dist) as i32,
            ((y as f32)/self.max_dist) as i32,
        )
    }

    pub fn build_point_index(&mut self, umap: &UmapData, max_dist: f32) {
        self.clear();
        self.max_dist = max_dist;

        for i in 0..umap.num_point {
            let x = umap.data[i*2+0];
            let y: f32 = umap.data[i*2+1];

            let sector_id = self.get_sector_id(x,y);

            /*
            possible speedup
            self.sectors.raw_entry_mut()
                .from_key(sector_id)
                .or_insert_with(|| (sector_id, UmapPointIndexTree::new()));
 */

            let sector = self.sectors.get_mut(&sector_id);
            if let Some(sector) = sector {
                sector.push((x,y,i));
            } else {
                let mut sector = Vec::new();
                sector.push((x,y,i));
                self.sectors.insert(sector_id, sector);
            }
        }
    }



    pub fn get_closest_point(&self, x:f32, y:f32) -> Option<usize> {

        //Scan all sectors around mouse for candidate points
        let (sector_mid_x,sector_mid_y) = self.get_sector_id(x,y);
        let mut list_cand = Vec::new();
        for sector_x in (sector_mid_x-1)..(sector_mid_x+2) {
            for sector_y in (sector_mid_y-1)..(sector_mid_y+2) {
                //Find closest point in sector
                if let Some(sector) = self.sectors.get(&(sector_x, sector_y)) {
                    let mut iter = sector.iter();

                    //First point
                    let (px,py,i) = iter.next().unwrap();
                    let mut best_i = *i;
                    let mut best_dist = dist2(x,y,  *px,*py);

                    //Remaining points
                    while let Some((px,py,i)) = iter.next() {
                        let this_dist = dist2(x,y,  *px,*py);
                        if this_dist < best_dist {
                            best_dist = this_dist;
                            best_i = *i;
                        }
                    }

                    list_cand.push((best_i, best_dist));
                }
            }
        }

        //If we got candidates...
        if list_cand.len()>0 {
            //Find distance to the nearest candidate
            let mut max=f32::MAX;
            let mut return_i = 0;
            for (cand_i, d2) in list_cand {
                if d2<max {
                    max=d2;
                    return_i=cand_i;
                }                
            }

            //See if this point is close enough
            if max < self.max_dist*self.max_dist {  // can remove this extra test
                Some(return_i)
            } else {
                None
            }

        } else {
            None
        }
    }
}








fn dist2(x1:f32,y1:f32,   x2:f32,y2:f32) -> f32 {
        let dx = x1 - x2;
        let dy = y1 - y2;
        let dist2 = dx*dx + dy*dy;
        dist2
}




/*


//////////////////////////////
////////////////////////////// Below is for using RTree. it is only fast for about 100 points, maybe 1000
//////////////////////////////

type UmapPoint = GeomWithData<[f32; 2], usize>;

pub struct UmapPointIndexTree {
    index: RTree<GeomWithData<[f32; 2], usize>>
}
impl UmapPointIndexTree {

    pub fn new() -> UmapPointIndexTree {
        let index = RTree::new();

        UmapPointIndexTree {
            index: index
        }

    }

    pub fn insert(&mut self, x: f32, y: f32, index: usize) {
        self.index.insert(UmapPoint::new([x, y], index));
    }

    pub fn build_point_index(umap: &UmapData) -> UmapPointIndexTree {

        let mut index = RTree::new();
        for i in 0..umap.num_point {
            let x = umap.data[i*2+0];
            let y: f32 = umap.data[i*2+1];
            index.insert(UmapPoint::new([x, y], i));
        }

        UmapPointIndexTree {
            index
        }
    }


    pub fn get_closest_point(&self, x:f32, y:f32, max_dist:f32) -> Option<(usize, f32)> {

        let search_pos = [x, y];

        let p = self.index.nearest_neighbor(&search_pos);

        if let Some(p) = p {

            let v = p.geom();
            let point_x = v[0];
            let point_y = v[1];

            let dx = point_x - x;
            let dy = point_y - y;
            let dist2 = dx*dx + dy*dy;
            //log::debug!("dist {} {}", dist2, p.data);

            if dist2 < max_dist*max_dist {  // can remove this extra test
                Some((p.data, dist2))
            } else {
                None
            }
        } else {
            None
        }

    }

} */
