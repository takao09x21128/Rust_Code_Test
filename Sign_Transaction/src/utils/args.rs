
use std::collections::HashMap;
use std::env;

use serde_json::Value;

use crate::definition::constants::ARGS_WITHOUT_VALUE;
use crate::definition::constants::ARGS_WITH_VALUE;
use crate::definition::constants::ARGS_WITH_FILENAME;

pub fn parse_args() -> Result<HashMap<String, String>, String> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        return Err("arguments is needed.".to_string())
    }

    let mut map = HashMap::<String, String>::new();

    parse_list(&args, &mut map)?;

    Ok(map)
}

fn parse_list(src: &Vec<String>, dest: &mut HashMap<String, String>) -> Result<(), String> {
    let mut i = 0;

    while i < src.len() {
        if src[i].starts_with("--") {
            let key_slice = &src[i][2..]; //skip --

            // json file
            if key_slice == ARGS_WITH_FILENAME {
                if i + 1 < src.len() && !src[i+1].starts_with("--") {
                    parse_json(&src[i+1], dest)?;
                    i += 2;
                    continue;
                }
            } else if ARGS_WITHOUT_VALUE.contains(&key_slice) {
                dest.insert(key_slice.to_string(), String::from("YES"));

            } else if ARGS_WITH_VALUE.contains(&key_slice) {
                if i + 1 < src.len() && !src[i+1].starts_with("--") {
                    dest.insert(key_slice.to_string(), src[i+1].clone());
                    i += 2;
                    continue;
                }
            }
        }
        i+=1;
    }

    Ok(())
}

fn parse_json(filename: &String, dest: &mut HashMap<String, String>) -> Result<(), String> {
    let content = std::fs::read_to_string(filename)
                    .map_err(|_| format!("cannot read the file {}", filename))?;

    let json: Value = serde_json::from_str(&content)
                    .map_err(|_| "failed to parse JSON".to_string())?;

    let obj = json.as_object()
                    .ok_or("invalid JSON file".to_string())?;

    for key in ARGS_WITH_VALUE.iter() {
        if let Some(Value::String(s)) = obj.get(*key) {
            dest.insert(key.to_string(), s.clone());
        }
    }

    Ok(())
}