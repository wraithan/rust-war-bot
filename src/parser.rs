use std::str;

#[derive(Debug)]
pub enum Message {
    Settings(SettingsValue),
    SetupMap{name: &'static str, value: Vec<u64>},
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

pub fn parse(line: &'static str) -> Result<Message, String> {
    let mut words = line.split(' ');
    match words.next().unwrap() {
        "settings" => parse_settings(words),
        "update_map" => {
            Ok(Message::UpdateMap)
        }
        _ => Err("unknown command".to_string())
    }
}

fn parse_settings(mut parts: str::Split<char>) -> Result<Message, String> {
    let command = try!(parts.next().ok_or("no command given to setting"));

    match command {
        "timebank" => Ok(Message::Settings(SettingsValue::Timebank(try!(parts_to_u64(parts, command.to_owned()))))),
        "time_per_move" => Ok(Message::Settings(SettingsValue::TimePerMove(try!(parts_to_u64(parts, command.to_owned()))))),
        "max_rounds" => Ok(Message::Settings(SettingsValue::MaxRounds(try!(parts_to_u64(parts, command.to_owned()))))),
        "starting_pick_amount" => Ok(Message::Settings(SettingsValue::StartingPickAmount(try!(parts_to_u64(parts, command.to_owned()))))),
        "starting_armies" => Ok(Message::Settings(SettingsValue::StartingArmies(try!(parts_to_u64(parts, command.to_owned()))))),
        "starting_regions" => {
            let mut peeker = parts.peekable();
            try!(peeker.peek().ok_or("got starting_regions setting with no value"));
            let mut value = Vec::new();
            for word in peeker {
                value.push(try!(u64::from_str_radix(word, 10).map_err(|e| e.to_string())));
            }
            Ok(Message::Settings(SettingsValue::StartingRegions(value)))
        }
        "your_bot" => {
            let raw_value = try!(parts.next().ok_or("got your_bot setting with no value"));
            Ok(Message::Settings(SettingsValue::YourBot(raw_value.to_owned())))
        }
        "opponent_bot" => {
            let raw_value = try!(parts.next().ok_or("got opponent_bot setting with no value"));
            Ok(Message::Settings(SettingsValue::OpponentBot(raw_value.to_owned())))
        }
        _ => Err(format!("unknown setting {}", command).to_string())
    }
}

fn parts_to_u64(mut parts: str::Split<char>, command: String) -> Result<u64, String> {
    let raw_value = try!(parts.next().ok_or(format!("got {} setting with no value", command)));
    Ok(try!(u64::from_str_radix(raw_value, 10).map_err(|e| e.to_string())))
}

#[test]
fn blank() {
    match parse("") {
        Ok(_) => panic!("should return error because it is malformed"),
        Err(e) => assert_eq!(e, "unknown command".to_string())
    }
}

#[test]
fn setting_blank() {
    match parse("settings") {
        Ok(_) => panic!("should return error because it is malformed"),
        Err(e) => assert_eq!(e, "no command given to setting".to_string())
    }
}

#[test]
fn setting_timebank_blank() {
    match parse("settings timebank") {
        Ok(_) => panic!("got a setting back when expecting an error"),
        Err(e) => assert_eq!(e, "got timebank setting with no value".to_string())
    }
}

#[test]
fn setting_timebank_bad_value() {
    match parse("settings timebank five") {
        Ok(_) => panic!("got a setting back when expecting an error"),
        Err(e) => assert_eq!(e, "invalid digit found in string".to_string())
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
fn setting_time_per_move_blank() {
    match parse("settings time_per_move") {
        Ok(_) => panic!("got a setting back when expecting an error"),
        Err(e) => assert_eq!(e, "got time_per_move setting with no value".to_string())
    }
}

#[test]
fn setting_time_per_move_bad_value() {
    match parse("settings time_per_move five") {
        Ok(_) => panic!("got a setting back when expecting an error"),
        Err(e) => assert_eq!(e, "invalid digit found in string".to_string())
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
    match parse("settings max_rounds") {
        Ok(_) => panic!("got a setting back when expecting an error"),
        Err(e) => assert_eq!(e, "got max_rounds setting with no value".to_string())
    }
}

#[test]
fn setting_max_rounds_bad_value() {
    match parse("settings max_rounds five") {
        Ok(_) => panic!("got a setting back when expecting an error"),
        Err(e) => assert_eq!(e, "invalid digit found in string".to_string())
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
    match parse("settings starting_pick_amount") {
        Ok(_) => panic!("got a setting back when expecting an error"),
        Err(e) => assert_eq!(e, "got starting_pick_amount setting with no value".to_string())
    }
}

#[test]
fn setting_starting_pick_amount_bad_value() {
    match parse("settings starting_pick_amount five") {
        Ok(_) => panic!("got a setting back when expecting an error"),
        Err(e) => assert_eq!(e, "invalid digit found in string".to_string())
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
    match parse("settings starting_armies") {
        Ok(_) => panic!("got a setting back when expecting an error"),
        Err(e) => assert_eq!(e, "got starting_armies setting with no value".to_string())
    }
}

#[test]
fn setting_starting_armies_bad_value() {
    match parse("settings starting_armies five") {
        Ok(_) => panic!("got a setting back when expecting an error"),
        Err(e) => assert_eq!(e, "invalid digit found in string".to_string())
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
    match parse("settings starting_regions") {
        Ok(_) => panic!("got a setting back when expecting an error"),
        Err(e) => assert_eq!(e, "got starting_regions setting with no value".to_string())
    }
}

#[test]
fn setting_starting_regions_bad_value() {
    match parse("settings starting_regions five 50") {
        Ok(a) => panic!("got a setting back when expecting an error, {:?}", a),
        Err(e) => assert_eq!(e, "invalid digit found in string".to_string())
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
    match parse("settings your_bot") {
        Ok(a) => panic!("got a setting back when expecting an error, {:?}", a),
        Err(e) => assert_eq!(e, "got your_bot setting with no value".to_string())
    }
}

#[test]
fn setting_your_bot_proper() {
    match parse("settings your_bot fig").unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::YourBot(value) => assert_eq!(value, "fig".to_string()),
            _ => panic!("got a setting that wasn't a your_bot")
        },
        _ => panic!("didn't get a settings object")
    }
}

#[test]
fn setting_opponent_bot_blank() {
    match parse("settings opponent_bot") {
        Ok(a) => panic!("got a setting back when expecting an error, {:?}", a),
        Err(e) => assert_eq!(e, "got opponent_bot setting with no value".to_string())
    }
}

#[test]
fn setting_opponent_bot_proper() {
    match parse("settings opponent_bot fig").unwrap() {
        Message::Settings(setting) => match setting {
            SettingsValue::OpponentBot(value) => assert_eq!(value, "fig".to_string()),
            _ => panic!("got a setting that wasn't a opponent_bot")
        },
        _ => panic!("didn't get a settings object")
    }
}
