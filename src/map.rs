use std::collections::HashMap;

pub struct GameMap {
    super_regions: HashMap<u64, SuperRegion>,
    regions: HashMap<u64, Region>,
}

pub struct SuperRegion {
    id: u64,
    value: u64,
    region_ids: Vec<u64>
}

pub struct Region {
    id: u64,
    super_region_id: u64,
    neighbor_ids: Vec<u64>
}

impl GameMap {
    fn new() -> GameMap {
        GameMap {
            super_regions: HashMap::with_capacity(30),
            regions: HashMap::with_capacity(120),
        }
    }
}

impl SuperRegion {
    fn new(id: u64, value: u64) -> SuperRegion {
        SuperRegion {
            id: id,
            value: value,
            region_ids: Vec::with_capacity(10)
        }
    }
}

impl Region {
    fn new(id: u64, super_region_id: u64) -> Region {
        Region {
            id: id,
            super_region_id: super_region_id,
            neighbor_ids: Vec::with_capacity(10)
        }
    }
}
