use std::collections::HashMap;

#[derive(Debug)]
pub struct GameMap {
    super_regions: HashMap<u64, SuperRegion>,
    regions: HashMap<u64, Region>,
}

#[derive(Debug)]
pub struct SuperRegion {
    pub id: u64,
    pub value: u64,
    pub region_ids: Vec<u64>
}

#[derive(Debug)]
pub struct Region {
    pub id: u64,
    pub super_region_id: u64,
    pub neighbor_ids: Vec<u64>,
    pub armies: u64,
    pub owner: OwnerValue
}

#[derive(Debug, PartialEq)]
pub enum OwnerValue{
    Ally,
    Enemy,
    Neutral
}

impl GameMap {
    pub fn new() -> GameMap {
        GameMap {
            super_regions: HashMap::with_capacity(30),
            regions: HashMap::with_capacity(120),
        }
    }

    pub fn add_super_region(&mut self, id: u64, value: u64) {
        self.super_regions.insert(id, SuperRegion::new(id, value));
    }

    pub fn add_region(&mut self, id: u64, super_region_id: u64) {
        self.regions.insert(id, Region::new(id, super_region_id));
        let mut super_region = self.super_regions.get_mut(&super_region_id).unwrap();
        super_region.region_ids.push(id);
    }

    pub fn add_region_neighbors(&mut self, id: u64, neighbor_ids: Vec<u64>) {
        for new_neighbor in neighbor_ids.iter() {
            let mut neighbor = self.regions.get_mut(&new_neighbor).unwrap();
            neighbor.neighbor_ids.push(id);
        }

        let mut region = self.regions.get_mut(&id).unwrap();

        for new_neighbor in neighbor_ids.iter() {
            region.neighbor_ids.push(*new_neighbor);
        }
    }

    pub fn upgrade_to_wasteland(&mut self, id: u64) {
        let region = self.regions.get_mut(&id).unwrap();
        region.armies = 6;
        region.owner = OwnerValue::Neutral
    }

    pub fn mark_as_enemy(&mut self, id: u64) {
        let region = self.regions.get_mut(&id).unwrap();
        region.owner = OwnerValue::Enemy
    }

    pub fn update_map(&mut self, id: u64, owner: OwnerValue, armies: u64) {
        let region = self.regions.get_mut(&id).unwrap();
        region.owner = owner;
        region.armies = armies;
    }

    pub fn update_fog(&mut self, obscured: Vec<u64>) {
        for (id, region) in self.regions.iter_mut() {
            if !obscured.contains(&id) && region.owner == OwnerValue::Ally {
                region.owner = OwnerValue::Enemy;
            }
        }
    }

    pub fn allies(&self) -> Vec<&Region> {
        self.regions.iter()
            .filter_map(|(_, region)| if region.owner == OwnerValue::Ally {Some(region)} else {None})
            .collect()
    }

    pub fn starting_pick_value(&self, region_id: &u64) -> f64 {
        let super_region = self.super_regions.get(&self.regions.get(region_id).unwrap().super_region_id).unwrap();

        let mut armies = 0;
        for (id, region) in self.regions.iter() {
            if super_region.region_ids.contains(&id) {
                armies += region.armies;
            }
        }

        super_region.value as f64 / (armies) as f64
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
            neighbor_ids: Vec::with_capacity(10),
            armies: 2,
            owner: OwnerValue::Neutral
        }
    }
}
