
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
    YourBot(&'static str),
    OpponentBot(&'static str),
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
        "timebank" => {
            let raw_value = try!(parts.next().ok_or("got timebank setting with no value"));
            let value = try!(u64::from_str_radix(raw_value, 10).map_err(|e| e.to_string()));
            Ok(Message::Settings(SettingsValue::Timebank(value)))
        }
        "starting_regions" => {
            let mut peeker = parts.peekable();
            try!(peeker.peek().ok_or("got starting_regions setting with no value"));
            let mut value = Vec::new();
            for word in peeker {
                value.push(try!(u64::from_str_radix(word, 10).map_err(|e| e.to_string())));
            }
            Ok(Message::Settings(SettingsValue::StartingRegions(value)))
        }
        _ => Err("unknown setting".to_string())
    }
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
