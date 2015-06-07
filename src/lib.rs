#[macro_use]
extern crate log;

pub mod parser;
pub mod map;

use parser::{SettingsValue, Message, parse};

#[derive(Default)]
pub struct Bot {
    settings: Option<Settings>,
    setting_buffer: Vec<parser::SettingsValue>
}

// Using 64 bits because I assume the compiler/platform we are targetting is
// also 64 bit.
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
    pub fn new() -> Bot {
        Default::default()
    }

    pub fn calculate(&self) {

    }

    pub fn read_line(&mut self, line: String) {
        match parse(line) {
            Ok(message) => match message {
                Message::SetupMap(_) => {},
                Message::Settings(setting) => {
                    self.setting_buffer.push(setting)
                },
                Message::UpdateMap(_) => {},
                Message::OpponentMoves(_) => {},
                Message::PickStartingRegion(_, _) => {
                    if !self.setting_buffer.is_empty() {
                        self.process_setting_buffer();
                    }
                },
                Message::GoPlaceArmies(_) => {},
                Message::GoAttackTransfer(_) => {},
            },
            Err(e) => {
                error!("Parser returned: {}", e)
            }
        }
    }

    fn process_setting_buffer(&mut self) {
        let mut settings = Settings{
            timebank: 0,
            time_per_move: 0,
            max_rounds: 0,
            name: "default".to_owned(),
            opponent: "default".to_owned(),
            starting_regions: Vec::new(),
            starting_pick_amount: 0,
            starting_armies: 0
        };
        while let Some(message) = self.setting_buffer.pop() {
            match message {
                SettingsValue::Timebank(time) => settings.timebank = time,
                SettingsValue::TimePerMove(time) => settings.time_per_move = time,
                SettingsValue::MaxRounds(rounds) => settings.max_rounds = rounds,
                SettingsValue::YourBot(name) => settings.name = name,
                SettingsValue::OpponentBot(name) => settings.opponent = name,
                SettingsValue::StartingRegions(region_ids) => {settings.starting_regions = region_ids},
                SettingsValue::StartingPickAmount(value) => settings.starting_pick_amount = value,
                SettingsValue::StartingArmies(value) => settings.starting_armies = value
            };
        }

        self.settings = Some(settings)
    }
}

pub fn run() {
    info!("run()!");
}
