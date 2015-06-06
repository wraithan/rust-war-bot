
#[macro_use]
pub mod errors;

use parser::errors::{ErrorKind, ParseError};
use std::str;


pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum Message {
    Settings(SettingsValue),
    SetupMap(SetupMapValue),
    PickStartingRegion{time_left: u64, ids: Vec<u64>},
    UpdateMap,
    GoPlaceArmies,
    GoAttackTranser,
}

#[derive(Debug)]
pub enum SettingsValue {
    Timebank(u64),
    TimePerMove(u64),
    MaxRounds(u64),
    YourBot(String),
    OpponentBot(String),
    StartingRegions(Vec<u64>),
    StartingPickAmount(u64),
    StartingArmies(u64)
}

#[derive(Debug)]
pub enum SetupMapValue {
    SuperRegions(Vec<(u64, u64)>),
    Regions(Vec<(u64, u64)>),
    Neighbors(Vec<(u64, Vec<u64>)>),
    Wastelands(Vec<u64>),
    OpponentStartingRegions(Vec<u64>)
}

pub fn parse(line: &'static str) -> ParseResult<Message> {
    let mut words = line.trim().split(' ');
    match words.next().unwrap() {
        "settings" => parse_settings(words),
        "setup_map" => parse_setup_map(words),
        _ => fail!((ErrorKind::UnknownCommand, "Got an unknown command", line.to_owned()))
    }
}

fn parse_settings(mut parts: str::Split<char>) -> ParseResult<Message> {
    let command = try!(parts.next().ok_or((ErrorKind::MalformedCommand, "Got setting without type")));

    match command {
        "timebank" => {
            let value = try!(parts_to_u64(parts, command.to_owned()));
            Ok(Message::Settings(SettingsValue::Timebank(value)))
        }
        "time_per_move" => {
            let value = try!(parts_to_u64(parts, command.to_owned()));
            Ok(Message::Settings(SettingsValue::TimePerMove(value)))
        }
        "max_rounds" => {
            let value = try!(parts_to_u64(parts, command.to_owned()));
            Ok(Message::Settings(SettingsValue::MaxRounds(value)))
        }
        "starting_pick_amount" => {
            let value = try!(parts_to_u64(parts, command.to_owned()));
            Ok(Message::Settings(SettingsValue::StartingPickAmount(value)))
        }
        "starting_armies" => {
            let value = try!(parts_to_u64(parts, command.to_owned()));
            Ok(Message::Settings(SettingsValue::StartingArmies(value)))
        }
        "starting_regions" => {
            let mut peeker = parts.peekable();
            try!(peeker.peek().ok_or((ErrorKind::MalformedCommand, "Got starting_regions without any arguments")));
            let mut value = Vec::new();
            for word in peeker {
                value.push(try!(u64::from_str_radix(word, 10)));
            }
            Ok(Message::Settings(SettingsValue::StartingRegions(value)))
        }
        "your_bot" => {
            let raw_value = try!(parts.next().ok_or((
                ErrorKind::MalformedCommand,
                "Got your_bot without an argument"
            )));
            Ok(Message::Settings(SettingsValue::YourBot(raw_value.to_owned())))
        }
        "opponent_bot" => {
            let raw_value = try!(parts.next().ok_or((
                ErrorKind::MalformedCommand,
                "Got opponent_bot without an argument"
            )));
            Ok(Message::Settings(SettingsValue::OpponentBot(raw_value.to_owned())))
        }
        _ => fail!((ErrorKind::UnknownCommand, "got an unknown setting type", command.to_owned()))
    }
}

fn parse_setup_map(mut parts: str::Split<char>) -> ParseResult<Message> {
    let command = try!(parts.next().ok_or((ErrorKind::MalformedCommand, "Got setup_map without type")));
    fail!((ErrorKind::UnknownCommand, "Not implemented"))
    // match command {
    //     "super_regions" => {
    //     }
    // }
}

fn parts_to_u64(mut parts: str::Split<char>, command: String) -> ParseResult<u64> {
    let raw_value = try!(parts.next().ok_or((
        ErrorKind::MalformedCommand,
        "Missing numeric argument",
        command
    )));
    Ok(try!(u64::from_str_radix(raw_value, 10)))
}

#[test]
fn blank() {
    match parse("").unwrap_err().kind() {
        ErrorKind::UnknownCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_blank() {
    match parse("settings").unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_timebank_blank() {
    match parse("settings timebank").unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_timebank_bad_value() {
    match parse("settings timebank five").unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_timebank_proper() {
    match parse("settings timebank 10000").unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::Timebank(value) => assert_eq!(value, 10000),
            _ => panic!("got a setting that wasn't a timebank")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_timebank_proper_with_leading_space() {
    match parse(" settings timebank 10000").unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::Timebank(value) => assert_eq!(value, 10000),
            _ => panic!("got a setting that wasn't a timebank")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_time_per_move_blank() {
    match parse("settings time_per_move").unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_time_per_move_bad_value() {
    match parse("settings time_per_move five").unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_time_per_move_proper() {
    match parse("settings time_per_move 10000").unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::TimePerMove(value) => assert_eq!(value, 10000),
            _ => panic!("got a setting that wasn't a time_per_move")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_max_rounds_blank() {
    match parse("settings max_rounds").unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_max_rounds_bad_value() {
    match parse("settings max_rounds five").unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_max_rounds_proper() {
    match parse("settings max_rounds 10000").unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::MaxRounds(value) => assert_eq!(value, 10000),
            _ => panic!("got a setting that wasn't a max_rounds")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_starting_pick_amount_blank() {
    match parse("settings starting_pick_amount").unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_starting_pick_amount_bad_value() {
    match parse("settings starting_pick_amount five").unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_starting_pick_amount_proper() {
    match parse("settings starting_pick_amount 10000").unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::StartingPickAmount(value) => assert_eq!(value, 10000),
            _ => panic!("got a setting that wasn't a starting_pick_amount")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_starting_armies_blank() {
    match parse("settings starting_armies").unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_starting_armies_bad_value() {
    match parse("settings starting_armies five").unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_starting_armies_proper() {
    match parse("settings starting_armies 10000").unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::StartingArmies(value) => assert_eq!(value, 10000),
            _ => panic!("got a setting that wasn't a starting_armies")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_starting_regions_blank() {
    match parse("settings starting_regions").unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_starting_regions_bad_value() {
    match parse("settings starting_regions five 50").unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_starting_regions_proper() {
    match parse("settings starting_regions 4 7 11").unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::StartingRegions(value) => assert_eq!(value, vec![4, 7, 11]),
            _ => panic!("got a setting that wasn't a starting_regions")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_your_bot_blank() {
    match parse("settings your_bot").unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_your_bot_proper() {
    match parse("settings your_bot fig").unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::YourBot(value) => assert_eq!(value, "fig".to_owned()),
            _ => panic!("got a setting that wasn't a your_bot")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_opponent_bot_blank() {
    match parse("settings opponent_bot").unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_opponent_bot_proper() {
    match parse("settings opponent_bot fig").unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::OpponentBot(value) => assert_eq!(value, "fig".to_owned()),
            _ => panic!("got a setting that wasn't a opponent_bot")
        },
        _ => panic!("didn't get a settings object")
    }
}
