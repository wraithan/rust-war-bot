#[macro_use]
pub mod errors;

use parser::errors::{ErrorKind, ParseError};
use std::str;


pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum Message {
    SetupMap(SetupMapValue),
    Settings(SettingsValue),
    UpdateMap(Vec<(u64, String, u64)>),
    OpponentMoves(Vec<OpponentMoveValue>),
    PickStartingRegion(u64, Vec<u64>),
    GoPlaceArmies(u64),
    GoAttackTransfer(u64),
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

#[derive(Debug)]
pub enum OpponentMoveValue {
    PlaceArmies(String, u64, u64),
    AttackTransfer(String, u64, u64, u64),
}

pub fn parse(line: String) -> ParseResult<Message> {
    let mut words = line.trim().split(' ');
    match words.next().unwrap() {
        "setup_map" => parse_setup_map(words),
        "settings" => parse_settings(words),
        "update_map" => parse_update_map(words),
        "opponent_moves" => parse_opponent_moves(words),
        "pick_starting_region" => parse_pick_starting_region(words),
        "go" => parse_go(words),
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
            let value = try!(parts_to_u64_vector(parts));
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

    match command {
        "super_regions" => {
            let value = try!(parts_to_pair_vector(parts));
            Ok(Message::SetupMap(SetupMapValue::SuperRegions(value)))
        }
        "regions" => {
            let value = try!(parts_to_pair_vector(parts));
            Ok(Message::SetupMap(SetupMapValue::Regions(value)))
        }
        "neighbors" => {
            let args: Vec<_> = parts.collect();

            if args.len() == 0 {
                fail!((
                    ErrorKind::MalformedCommand,
                    "Got setup_map neighbors without any arguments"
                ))
            }

            let mut value = Vec::new();
            for pair in args.chunks(2) {
                if pair.len() < 2 {
                    fail!((
                        ErrorKind::MalformedCommand,
                        "Got setup_map neighbors with an odd number of args, expecting and even amount"
                    ))
                }
                value.push((
                    try!(u64::from_str_radix(pair.get(0).unwrap(), 10)),
                    try!(parts_to_u64_vector(pair.get(1).unwrap().split(',')))
                ));
            }
            Ok(Message::SetupMap(SetupMapValue::Neighbors(value)))
        }
        "wastelands" => {
            let value = try!(parts_to_u64_vector(parts));
            Ok(Message::SetupMap(SetupMapValue::Wastelands(value)))
        }
        "opponent_starting_regions" => {
            let value = try!(parts_to_u64_vector(parts));
            Ok(Message::SetupMap(SetupMapValue::OpponentStartingRegions(value)))
        }
        _ => fail!((ErrorKind::UnknownCommand, "got an unknown setup_map type", command.to_owned()))
    }
}

fn parse_update_map(parts: str::Split<char>) -> ParseResult<Message> {
    let args: Vec<_> = parts.collect();

    if args.len() == 0 {
        fail!((
            ErrorKind::MalformedCommand,
            "Got update_map without any arguments"
        ))
    }

    let mut value = Vec::new();

    for triplet in args.chunks(3) {
        if triplet.len() < 3 {
            fail!((
                ErrorKind::MalformedCommand,
                "Got update_map without all three parts"
            ))
        }
        value.push((
            try!(u64::from_str_radix(triplet.get(0).unwrap(), 10)),
            triplet.get(1).unwrap().to_string(),
            try!(u64::from_str_radix(triplet.get(2).unwrap(), 10))
        ));
    }
    Ok(Message::UpdateMap(value))
}

fn parse_opponent_moves(mut parts: str::Split<char>) -> ParseResult<Message> {
    let mut value = Vec::new();

    loop {
        match parts.next() {
            Some(name) => {
                let command = try!(parts.next().ok_or((ErrorKind::MalformedCommand, "opponent_moves missing type")));
                match command {
                    "attack/transfer" => {
                        value.push(try!(parts_to_attack_transfer(name.to_string(), &mut parts)))
                    }
                    "place_armies" => {
                        value.push(try!(parts_to_place_armies(name.to_string(), &mut parts)))
                    }
                    _ => fail!((ErrorKind::MalformedCommand, "opponent_moves unknown type"))
                }
            }
            None => break
        }
    }

    Ok(Message::OpponentMoves(value))
}

fn parse_pick_starting_region(mut parts: str::Split<char>) -> ParseResult<Message> {
    let raw_time = try!(parts.next().ok_or((ErrorKind::MalformedCommand, "Got go without type")));
    let timebank = try!(u64::from_str_radix(raw_time, 10));
    let value = try!(parts_to_u64_vector(parts));
    Ok(Message::PickStartingRegion(timebank, value))
}

fn parse_go(mut parts: str::Split<char>) -> ParseResult<Message> {
    let command = try!(parts.next().ok_or((ErrorKind::MalformedCommand, "Got setup_map without type")));

    match command {
        "place_armies" => {
            let value = try!(parts_to_u64(parts, "go place_armies".to_owned()));
            Ok(Message::GoPlaceArmies(value))
        }
        "attack/transfer" => {
            let value = try!(parts_to_u64(parts, "go attack/transfer".to_owned()));
            Ok(Message::GoAttackTransfer(value))
        }
        _ => fail!((ErrorKind::UnknownCommand, "got an unknown go type", command.to_owned()))
    }
}

fn parts_to_u64(mut parts: str::Split<char>, command: String) -> ParseResult<u64> {
    let raw_value = try!(parts.next().ok_or((
        ErrorKind::MalformedCommand,
        "Missing numeric argument",
        command
    )));
    Ok(try!(u64::from_str_radix(raw_value, 10)))
}

fn parts_to_u64_vector(parts: str::Split<char>)  -> ParseResult<Vec<u64>> {
    let mut peeker = parts.peekable();
    try!(peeker.peek().ok_or((ErrorKind::MalformedCommand, "Got command without any arguments")));
    let mut value = Vec::new();
    for word in peeker {
        value.push(try!(u64::from_str_radix(word, 10)));
    }
    Ok(value)
}


fn parts_to_pair_vector(parts: str::Split<char>)  -> ParseResult<Vec<(u64, u64)>> {
    let args: Vec<_> = parts.collect();

    if args.len() == 0 {
        fail!((
            ErrorKind::MalformedCommand,
            "Got setup_map subcommand without any arguments"
        ))
    };

    let mut value = Vec::new();
    for pair in args.chunks(2) {
        if pair.len() < 2 {
            fail!((
                ErrorKind::MalformedCommand,
                "odd number of arguments to setup_map subcommand expecting an even amount"
            ))
        }
        value.push((
            try!(u64::from_str_radix(pair.get(0).unwrap(), 10)),
            try!(u64::from_str_radix(pair.get(1).unwrap(), 10))
        ));
    }
    Ok(value)
}

fn parts_to_attack_transfer(name: String, parts: &mut str::Split<char>) -> ParseResult<OpponentMoveValue> {
    let raw_source = try!(parts.next().ok_or((ErrorKind::MalformedCommand, "opponent_moves source")));
    let raw_target = try!(parts.next().ok_or((ErrorKind::MalformedCommand, "opponent_moves target")));
    let raw_armies = try!(parts.next().ok_or((ErrorKind::MalformedCommand, "opponent_moves armies")));
    Ok(OpponentMoveValue::AttackTransfer(
        name,
        try!(u64::from_str_radix(raw_source, 10)),
        try!(u64::from_str_radix(raw_target, 10)),
        try!(u64::from_str_radix(raw_armies, 10)),
    ))
}

fn parts_to_place_armies(name: String, parts: &mut str::Split<char>) -> ParseResult<OpponentMoveValue> {
    let raw_target = try!(parts.next().ok_or((ErrorKind::MalformedCommand, "opponent_moves target")));
    let raw_armies = try!(parts.next().ok_or((ErrorKind::MalformedCommand, "opponent_moves armies")));
    Ok(OpponentMoveValue::PlaceArmies(
        name,
        try!(u64::from_str_radix(raw_target, 10)),
        try!(u64::from_str_radix(raw_armies, 10)),
    ))
}

#[test]
fn blank() {
    match parse("".to_owned()).unwrap_err().kind() {
        ErrorKind::UnknownCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_blank() {
    match parse("settings".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_timebank_blank() {
    match parse("settings timebank".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_timebank_bad_value() {
    match parse("settings timebank five".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_timebank_proper() {
    match parse("settings timebank 10000".to_owned()).unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::Timebank(value) => assert_eq!(value, 10000),
            _ => panic!("got a setting that wasn't a timebank")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_timebank_proper_with_leading_space() {
    match parse(" settings timebank 10000".to_owned()).unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::Timebank(value) => assert_eq!(value, 10000),
            _ => panic!("got a setting that wasn't a timebank")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_time_per_move_blank() {
    match parse("settings time_per_move".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_time_per_move_bad_value() {
    match parse("settings time_per_move five".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_time_per_move_proper() {
    match parse("settings time_per_move 10000".to_owned()).unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::TimePerMove(value) => assert_eq!(value, 10000),
            _ => panic!("got a setting that wasn't a time_per_move")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_max_rounds_blank() {
    match parse("settings max_rounds".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_max_rounds_bad_value() {
    match parse("settings max_rounds five".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_max_rounds_proper() {
    match parse("settings max_rounds 10000".to_owned()).unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::MaxRounds(value) => assert_eq!(value, 10000),
            _ => panic!("got a setting that wasn't a max_rounds")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_starting_pick_amount_blank() {
    match parse("settings starting_pick_amount".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_starting_pick_amount_bad_value() {
    match parse("settings starting_pick_amount five".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_starting_pick_amount_proper() {
    match parse("settings starting_pick_amount 10000".to_owned()).unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::StartingPickAmount(value) => assert_eq!(value, 10000),
            _ => panic!("got a setting that wasn't a starting_pick_amount")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_starting_armies_blank() {
    match parse("settings starting_armies".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_starting_armies_bad_value() {
    match parse("settings starting_armies five".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_starting_armies_proper() {
    match parse("settings starting_armies 10000".to_owned()).unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::StartingArmies(value) => assert_eq!(value, 10000),
            _ => panic!("got a setting that wasn't a starting_armies")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_starting_regions_blank() {
    match parse("settings starting_regions".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_starting_regions_bad_value() {
    match parse("settings starting_regions five 50".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_starting_regions_proper() {
    match parse("settings starting_regions 4 7 11".to_owned()).unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::StartingRegions(value) => assert_eq!(value, vec![4, 7, 11]),
            _ => panic!("got a setting that wasn't a starting_regions")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_your_bot_blank() {
    match parse("settings your_bot".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_your_bot_proper() {
    match parse("settings your_bot fig".to_owned()).unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::YourBot(value) => assert_eq!(value, "fig".to_owned()),
            _ => panic!("got a setting that wasn't a your_bot")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_opponent_bot_blank() {
    match parse("settings opponent_bot".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setting_opponent_bot_proper() {
    match parse("settings opponent_bot fig".to_owned()).unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::OpponentBot(value) => assert_eq!(value, "fig".to_owned()),
            _ => panic!("got a setting that wasn't a opponent_bot")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setup_map_blank() {
    match parse("setup_map".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_super_regions_blank() {
    match parse("setup_map super_regions".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_super_regions_nonnumeric_id() {
    match parse("setup_map super_regions 3 fred".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_super_regions_missing_value() {
    match parse("setup_map super_regions 1".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_super_regions_nonnumeric_value() {
    match parse("setup_map super_regions 3 fred".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_super_regions_proper() {
    match parse("setup_map super_regions 1 2".to_owned()).unwrap() {
        Message::SetupMap(setting) => match setting {
            SetupMapValue::SuperRegions(value) => assert_eq!(value, vec![(1, 2)]),
            _ => panic!("got a setup_map value that wasn't a super region")
        },
        _ => panic!("didn't get a setup_map object")
    }
}

#[test]
fn setup_map_super_regions_multiple_proper() {
    match parse("setup_map super_regions 1 2 2 4".to_owned()).unwrap() {
        Message::SetupMap(setting) => match setting {
            SetupMapValue::SuperRegions(value) => assert_eq!(value, vec![(1, 2), (2, 4)]),
            _ => panic!("got a setup_map value that wasn't a super region")
        },
        _ => panic!("didn't get a setup_map object")
    }
}

#[test]
fn setup_map_regions_blank() {
    match parse("setup_map regions".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_regions_nonnumeric_id() {
    match parse("setup_map regions 3 fred".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_regions_missing_value() {
    match parse("setup_map regions 1".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_regions_nonnumeric_value() {
    match parse("setup_map regions 3 fred".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_regions_proper() {
    match parse("setup_map regions 1 2".to_owned()).unwrap() {
        Message::SetupMap(setting) => match setting {
            SetupMapValue::Regions(value) => assert_eq!(value, vec![(1, 2)]),
            _ => panic!("got a setup_map value that wasn't a region")
        },
        _ => panic!("didn't get a setup_map object")
    }
}

#[test]
fn setup_map_regions_multiple_proper() {
    match parse("setup_map regions 1 1 2 1".to_owned()).unwrap() {
        Message::SetupMap(setting) => match setting {
            SetupMapValue::Regions(value) => assert_eq!(value, vec![(1, 1), (2, 1)]),
            _ => panic!("got a setup_map value that wasn't a region")
        },
        _ => panic!("didn't get a setup_map object")
    }
}

#[test]
fn setup_map_wastelands_blank() {
    match parse("setup_map wastelands".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_wastelands_nonnumeric() {
    match parse("setup_map wastelands 3 fred".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_wastelands_proper() {
    match parse("setup_map wastelands 1 2".to_owned()).unwrap() {
        Message::SetupMap(setting) => match setting {
            SetupMapValue::Wastelands(value) => assert_eq!(value, vec![1, 2]),
            _ => panic!("got a setup_map value that wasn't a wastelands")
        },
        _ => panic!("didn't get a setup_map object")
    }
}

#[test]
fn setup_map_opponent_starting_regions_blank() {
    match parse("setup_map opponent_starting_regions".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_opponent_starting_regions_nonnumeric() {
    match parse("setup_map opponent_starting_regions 3 fred".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_opponent_starting_regions_proper() {
    match parse("setup_map opponent_starting_regions 1 2".to_owned()).unwrap() {
        Message::SetupMap(setting) => match setting {
            SetupMapValue::OpponentStartingRegions(value) => assert_eq!(value, vec![1, 2]),
            _ => panic!("got a setup_map value that wasn't a opponent_starting_regions")
        },
        _ => panic!("didn't get a setup_map object")
    }
}

#[test]
fn setup_map_neighbors_blank() {
    match parse("setup_map neighbors".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_neighbors_nonnumeric_id() {
    match parse("setup_map neighbors fred 2,3".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_neighbors_comma_in_id() {
    match parse("setup_map neighbors 1,2 2,3".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_neighbors_missing_value() {
    match parse("setup_map neighbors 1".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_neighbors_nonnumeric_value() {
    match parse("setup_map neighbors 1 foxy,3".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn setup_map_neighbors_proper() {
    match parse("setup_map neighbors 1 2,3".to_owned()).unwrap() {
        Message::SetupMap(setting) => match setting {
            SetupMapValue::Neighbors(value) => assert_eq!(value, vec![(1, vec![2, 3])]),
            _ => panic!("got a setup_map value that wasn't a region")
        },
        _ => panic!("didn't get a setup_map object")
    }
}

#[test]
fn setup_map_neighbors_proper_single_neighbor() {
    match parse("setup_map neighbors 1 2".to_owned()).unwrap() {
        Message::SetupMap(setting) => match setting {
            SetupMapValue::Neighbors(value) => assert_eq!(value, vec![(1, vec![2])]),
            _ => panic!("got a setup_map value that wasn't a region")
        },
        _ => panic!("didn't get a setup_map object")
    }
}

#[test]
fn setup_map_neighbors_multiple_proper() {
    match parse("setup_map neighbors 1 2,3 2 4,5".to_owned()).unwrap() {
        Message::SetupMap(setting) => match setting {
            SetupMapValue::Neighbors(value) => assert_eq!(value, vec![(1, vec![2, 3]), (2, vec![4, 5])]),
            _ => panic!("got a setup_map value that wasn't a region")
        },
        _ => panic!("didn't get a setup_map object")
    }
}

#[test]
fn go_blank() {
    match parse("go".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn go_place_armies_blank() {
    match parse("go place_armies".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn go_place_armies_nonnumeric() {
    match parse("go place_armies samwise".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn go_place_armies_proper() {
    match parse("go place_armies 100".to_owned()).unwrap() {
        Message::GoPlaceArmies(timebank) => assert_eq!(timebank, 100),
        _ => panic!("didn't get a setup_map object")
    }
}

#[test]
fn go_attack_transfer_blank() {
    match parse("go attack/transfer".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn go_attack_transfer_nonnumeric() {
    match parse("go attack/transfer frodo".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn go_attack_transfer_proper() {
    match parse("go attack/transfer 100".to_owned()).unwrap() {
        Message::GoAttackTransfer(timebank) => assert_eq!(timebank, 100),
        _ => panic!("didn't get a setup_map object")
    }
}

#[test]
fn update_map_blank() {
    match parse("update_map".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn update_map_missing_name_and_amount() {
    match parse("update_map 1".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn update_map_missing_amount() {
    match parse("update_map 1 foobar".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn update_map_nonnumeric_amount() {
    match parse("update_map 1 foobar lol".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn update_map_nonnumeric_id() {
    match parse("update_map so foobar 3".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn update_map_proper() {
    match parse("update_map 1 truck 4".to_owned()).unwrap() {
        Message::UpdateMap(value) => assert_eq!(value, vec!((1, "truck".to_owned(), 4))),
        _ => panic!("didn't get an update_map object")
    }
}

#[test]
fn update_map_proper_multiple() {
    match parse("update_map 1 truck 4 2 train 8".to_owned()).unwrap() {
        Message::UpdateMap(value) => assert_eq!(
            value,
            vec!((1, "truck".to_owned(), 4), (2, "train".to_owned(), 8))
        ),
        _ => panic!("didn't get an update_map object")
    }
}

#[test]
fn opponent_moves_blank() {
    match parse("opponent_moves".to_owned()).unwrap() {
        Message::OpponentMoves(value) => {
            assert_eq!(value.len(), 0)
        },
        _ => panic!("didn't get an opponent_moves object")
    }
}

#[test]
fn opponent_moves_missing_type() {
    match parse("opponent_moves name".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn opponent_moves_unknown_type() {
    match parse("opponent_moves name merf".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn opponent_moves_place_armies_missing_region() {
    match parse("opponent_moves name place_armies".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn opponent_moves_place_armies_nonnumeric_region() {
    match parse("opponent_moves name place_armies wilma".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn opponent_moves_place_armies_missing_value() {
    match parse("opponent_moves name place_armies".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn opponent_moves_place_armies_nonnumeric_value() {
    match parse("opponent_moves name place_armies 1 wilma".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn opponent_moves_place_armies_proper() {
    match parse("opponent_moves name place_armies 4 5".to_owned()).unwrap() {
        Message::OpponentMoves(value) => {
            let mut iter = value.into_iter();
            let first_move = iter.next().unwrap();
            match first_move {
                OpponentMoveValue::PlaceArmies(player, region, value) => {
                    assert_eq!(player, "name".to_owned());
                    assert_eq!(region, 4);
                    assert_eq!(value, 5);
                }
                _ => panic!("didn't get the expected opponent_moves type")
            }
        }
        _ => panic!("didn't get an opponent_moves object")
    }
}

#[test]
fn opponent_moves_place_armies_proper_multiple() {
    match parse("opponent_moves player2 place_armies 4 5 player2 place_armies 7 9".to_owned()).unwrap() {
        Message::OpponentMoves(value) => {
            let mut iter = value.into_iter();
            match iter.next().unwrap() {
                OpponentMoveValue::PlaceArmies(player, region, value) => {
                    assert_eq!(player, "player2".to_owned());
                    assert_eq!(region, 4);
                    assert_eq!(value, 5);
                }
                _ => panic!("didn't get the expected opponent_moves type")
            }
            match iter.next().unwrap() {
                OpponentMoveValue::PlaceArmies(player, region, value) => {
                    assert_eq!(player, "player2".to_owned());
                    assert_eq!(region, 7);
                    assert_eq!(value, 9);
                }
                _ => panic!("didn't get the expected opponent_moves type")
            }
        }
        _ => panic!("didn't get an opponent_moves object")
    }
}

#[test]
fn opponent_moves_attack_transfer_missing_source_region() {
    match parse("opponent_moves name attack/transfer".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn opponent_moves_attack_transfer_nonnumeric_source_region() {
    match parse("opponent_moves name attack/transfer wilma".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn opponent_moves_attack_transfer_missing_target_region() {
    match parse("opponent_moves name attack/transfer 1".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn opponent_moves_attack_transfer_nonnumeric_target_region() {
    match parse("opponent_moves name attack/transfer 2 baz".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn opponent_moves_attack_transfer_missing_value() {
    match parse("opponent_moves name attack/transfer 1 2".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn opponent_moves_attack_transfer_nonnumeric_value() {
    match parse("opponent_moves name attack/transfer 3 4 betty".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn opponent_moves_attack_transfer_proper() {
    match parse("opponent_moves name attack/transfer 4 5 6".to_owned()).unwrap() {
        Message::OpponentMoves(value) => {
            let mut iter = value.into_iter();
            let first_move = iter.next().unwrap();
            match first_move {
                OpponentMoveValue::AttackTransfer(player, source, target, value) => {
                    assert_eq!(player, "name".to_owned());
                    assert_eq!(source, 4);
                    assert_eq!(target, 5);
                    assert_eq!(value, 6);
                }
                _ => panic!("didn't get the expected opponent_moves type")
            }
        }
        _ => panic!("didn't get an opponent_moves object")
    }
}

#[test]
fn opponent_moves_attack_transfer_proper_multiple() {
    match parse("opponent_moves player2 attack/transfer 4 5 6 player2 attack/transfer 7 9 11".to_owned()).unwrap() {
        Message::OpponentMoves(value) => {
            let mut iter = value.into_iter();
            match iter.next().unwrap() {
                OpponentMoveValue::AttackTransfer(player, source, target, value) => {
                    assert_eq!(player, "player2".to_owned());
                    assert_eq!(source, 4);
                    assert_eq!(target, 5);
                    assert_eq!(value, 6);
                }
                _ => panic!("didn't get the expected opponent_moves type")
            }
            match iter.next().unwrap() {
                OpponentMoveValue::AttackTransfer(player, source, target, value) => {
                    assert_eq!(player, "player2".to_owned());
                    assert_eq!(source, 7);
                    assert_eq!(target, 9);
                    assert_eq!(value, 11);
                }
                _ => panic!("didn't get the expected opponent_moves type")
            }
        }
        _ => panic!("didn't get an opponent_moves object")
    }
}

#[test]
fn opponent_moves_attack_transfer_place_armies_proper() {
    match parse("opponent_moves player2 attack/transfer 4 5 6 player2 place_armies 7 9".to_owned()).unwrap() {
        Message::OpponentMoves(value) => {
            let mut iter = value.into_iter();
            match iter.next().unwrap() {
                OpponentMoveValue::AttackTransfer(player, source, target, value) => {
                    assert_eq!(player, "player2".to_owned());
                    assert_eq!(source, 4);
                    assert_eq!(target, 5);
                    assert_eq!(value, 6);
                }
                _ => panic!("didn't get the expected opponent_moves type")
            }
            match iter.next().unwrap() {
                OpponentMoveValue::PlaceArmies(player, region, value) => {
                    assert_eq!(player, "player2".to_owned());
                    assert_eq!(region, 7);
                    assert_eq!(value, 9);
                }
                _ => panic!("didn't get the expected opponent_moves type")
            }
        }
        _ => panic!("didn't get an opponent_moves object")
    }
}

#[test]
fn opponent_moves_place_armies_attack_transfer_proper() {
    match parse("opponent_moves player2 place_armies 7 9 player2 attack/transfer 4 5 6".to_owned()).unwrap() {
        Message::OpponentMoves(value) => {
            let mut iter = value.into_iter();
            match iter.next().unwrap() {
                OpponentMoveValue::PlaceArmies(player, region, value) => {
                    assert_eq!(player, "player2".to_owned());
                    assert_eq!(region, 7);
                    assert_eq!(value, 9);
                }
                _ => panic!("didn't get the expected opponent_moves type")
            }
            match iter.next().unwrap() {
                OpponentMoveValue::AttackTransfer(player, source, target, value) => {
                    assert_eq!(player, "player2".to_owned());
                    assert_eq!(source, 4);
                    assert_eq!(target, 5);
                    assert_eq!(value, 6);
                }
                _ => panic!("didn't get the expected opponent_moves type")
            }
        }
        _ => panic!("didn't get an opponent_moves object")
    }
}

#[test]
fn pick_starting_region_blank() {
    match parse("pick_starting_region".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn pick_starting_region_missing_region() {
    match parse("pick_starting_region 1000".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn pick_starting_region_nonnumeric_time() {
    match parse("pick_starting_region tenseconds".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn pick_starting_region_nonnumeric_region() {
    match parse("pick_starting_region 100 tenseconds".to_owned()).unwrap_err().kind() {
        ErrorKind::MalformedCommand => {},
        _ => panic!("got an error of unexpected kind")
    }
}

#[test]
fn pick_starting_region_proper() {
    match parse("pick_starting_region 100 1".to_owned()).unwrap() {
        Message::PickStartingRegion(time, value) => {
            assert_eq!(time, 100);
            assert_eq!(value, vec!(1));
        }
        _ => panic!("didn't get an opponent_moves object")
    }
}

#[test]
fn pick_starting_region_proper_multiple() {
    match parse("pick_starting_region 283 5 8".to_owned()).unwrap() {
        Message::PickStartingRegion(time, value) => {
            assert_eq!(time, 283);
            assert_eq!(value, vec![5, 8]);
        }
        _ => panic!("didn't get an opponent_moves object")
    }
}
