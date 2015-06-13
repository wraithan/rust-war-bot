#[macro_use]
extern crate log;
extern crate rand;

pub mod parser;
pub mod map;

use std::sync::mpsc::{Receiver, Sender, channel};
use parser::{Message, SettingsValue, SetupMapValue, OpponentMoveValue, parse};
use rand::{thread_rng, sample};

pub struct Bot {
    settings: Option<Settings>,
    setting_buffer: Vec<parser::SettingsValue>,
    map: map::GameMap,
    output: Sender<String>,
    output_buffer: String
}

// Using 64 bits because I assume the compiler/platform we are targetting is
// also 64 bit.
#[derive(Debug)]
struct Settings {
    timebank: u64,
    time_per_move: u64,
    max_rounds: u64,
    name: String,
    opponent: String,
    starting_regions: Vec<u64>,
    starting_pick_amount: u64,
    starting_armies: u64
}

impl Bot {
    pub fn spawn() -> (Sender<String>, Receiver<String>) {
        let (input_tx, input_rx) = channel();
        let (output_tx, output_rx) = channel();
        std::thread::spawn(move || {
            let mut bot = Bot::new(output_tx);
            loop {
                match input_rx.try_recv() {
                    Ok(line) => bot.read_line(line),
                    Err(_) => bot.calculate()
                }
            }
        });
        (input_tx, output_rx)
    }

    fn new(output: Sender<String>) -> Bot {
        Bot {
            settings: None,
            setting_buffer: Vec::with_capacity(10),
            map: map::GameMap::new(),
            output: output,
            output_buffer: String::new(),
        }
    }

    fn calculate(&self) {
        debug!("calculating")
    }

    fn read_line(&mut self, line: String) {
        info!("read_line: '{}'", line);
        match parse(line) {
            Ok(message) => match message {
                Message::SetupMap(map_message) => self.process_map_message(map_message),
                Message::Settings(setting) => {
                    if self.settings.is_none() {
                        self.setting_buffer.push(setting);
                    } else {
                        self.process_settings(setting);
                    }
                },
                Message::UpdateMap(regions) => {
                    let mut found = Vec::new();

                    for (id, raw_owner, armies) in regions {
                        found.push(id);
                        let owner = self.name_to_owner_value(raw_owner).unwrap();
                        self.map.update_map(id, owner, armies);
                    }

                    self.map.update_fog(found);
                },
                Message::OpponentMoves(moves) => for movement in moves {
                    match movement {
                        OpponentMoveValue::PlaceArmies(_, id, _) => {
                            self.map.mark_as_enemy(id);
                        },
                        OpponentMoveValue::AttackTransfer(_, source_id, _, _) => {
                            self.map.mark_as_enemy(source_id);
                        },
                    }
                },
                Message::PickStartingRegion(_, regions) => {
                    if !self.setting_buffer.is_empty() {
                        self.process_setting_buffer();
                    }
                    let mut rng = thread_rng();
                    let choices = sample(&mut rng, regions.iter(), 1);
                    let response = format!("{}", choices.get(0).unwrap());
                    self.output_buffer = self.queue(response);
                },
                Message::GoPlaceArmies(_) => {
                    let regions = self.map.allies();
                    let settings = self.settings.as_ref().unwrap();
                    let mut rng = thread_rng();
                    let choices = sample(&mut rng, regions.iter(), settings.starting_armies as usize);
                    for region in choices {
                        let response = format!("{} place_armies {} {}", settings.name, region.id, 1);
                        self.output_buffer = self.queue(response);
                    }
                },
                Message::GoAttackTransfer(_) => {
                    let regions = self.map.allies();
                    let settings = self.settings.as_ref().unwrap();
                    let mut rng = thread_rng();
                    for region in regions {
                        if region.armies >= 4 {
                            let choices = sample(&mut rng, region.neighbor_ids.iter(), 1);
                            if let Some(target) = choices.get(0) {
                                let response = format!("{} attack/transfer {} {} {}", settings.name, region.id, target, 3);
                                self.output_buffer = self.queue(response);
                            }
                        }
                    }
                    if self.output_buffer.is_empty() {
                        self.output_buffer = "No moves".to_owned();
                    }
                },
            },
            Err(e) => {
                error!("Parser returned: {}", e)
            }
        }
        self.send();
    }

    fn queue(&self, message: String) -> String {
        if self.output_buffer.is_empty() {
            message
        } else {
            format!("{} {}", self.output_buffer, message)
        }
    }

    fn send(&mut self) {
        info!("send: '{}'", self.output_buffer);
        if self.output_buffer.len() > 0 {
            let response = self.output_buffer.clone();
            self.output_buffer = String::new();
            self.output.send(response).unwrap();
        }
    }

    fn process_map_message(&mut self, message: SetupMapValue) {
        match message {
            SetupMapValue::SuperRegions(super_regions) => {
                for (id, value) in super_regions {
                    self.map.add_super_region(id, value);
                }
            },
            SetupMapValue::Regions(regions) => {
                for (id, super_region) in regions {
                    self.map.add_region(id, super_region);
                }
            },
            SetupMapValue::Neighbors(new_neighbors) => {
                for (id, neighbors) in new_neighbors {
                    self.map.add_region_neighbors(id, neighbors);
                }
            },
            SetupMapValue::Wastelands(wastelands) => {
                for id in wastelands {
                    self.map.upgrade_to_wasteland(id);
                }
            },
            SetupMapValue::OpponentStartingRegions(enemies) => {
                for id in enemies {
                    self.map.mark_as_enemy(id);
                }
            }
        }
    }

    fn process_setting_buffer(&mut self) {
        let settings = Settings{
            timebank: 0,
            time_per_move: 0,
            max_rounds: 0,
            name: "default".to_owned(),
            opponent: "default".to_owned(),
            starting_regions: Vec::new(),
            starting_pick_amount: 0,
            starting_armies: 0
        };

        self.settings = Some(settings);

        while let Some(message) = self.setting_buffer.pop() {
            self.process_settings(message);
        }

    }

    fn process_settings(&mut self, message: SettingsValue) {
        let settings = self.settings.as_mut().unwrap();

        match message {
            SettingsValue::Timebank(time) => settings.timebank = time,
            SettingsValue::TimePerMove(time) => settings.time_per_move = time,
            SettingsValue::MaxRounds(rounds) => settings.max_rounds = rounds,
            SettingsValue::YourBot(name) => settings.name = name,
            SettingsValue::OpponentBot(name) => settings.opponent = name,
            SettingsValue::StartingRegions(region_ids) => settings.starting_regions = region_ids,
            SettingsValue::StartingPickAmount(value) => settings.starting_pick_amount = value,
            SettingsValue::StartingArmies(value) => settings.starting_armies = value
        };
    }

    fn name_to_owner_value(&self, name: String) -> Result<map::OwnerValue, &'static str> {
        let settings = self.settings.as_ref().unwrap();
        if name == settings.name {
            Ok(map::OwnerValue::Ally)
        } else if name == settings.opponent {
            Ok(map::OwnerValue::Enemy)
        } else if name == "neutral" {
            Ok(map::OwnerValue::Neutral)
        } else {
            error!("Got unknown name {}", name);
            Err("Unknown name")
        }
    }
}

pub fn run() {
    info!("run()!");
}
