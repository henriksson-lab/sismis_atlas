use my_web_app::UmapData;
use rstar::{primitives::GeomWithData, RTree};

/// This data structure takes quite a while to build... 
/// do it asynch, after rendering, or provide a simpler algo?


type UmapPoint = GeomWithData<[f32; 2], usize>;

pub struct UmapPointIndex {
    index: RTree<GeomWithData<[f32; 2], usize>>
}
impl UmapPointIndex {

    pub fn new() -> UmapPointIndex {
        let index = RTree::new();

        UmapPointIndex {
            index: index
        }

    }

    pub fn build_point_index(umap: &UmapData) -> UmapPointIndex {

        let mut index = RTree::new();
        for i in 0..umap.num_point {
            let x = umap.data[i*2+0];
            let y: f32 = umap.data[i*2+1];
            index.insert(UmapPoint::new([x, y], i));
        }

        UmapPointIndex {
            index
        }
    }


    pub fn get_closest_point(&self, x:f32, y:f32, max_dist:f32) -> Option<usize> {

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

            if dist2 < max_dist*max_dist {
                Some(p.data)           
            } else {
                None
            }
        } else {
            None
        }

    }

}