

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde_json::{Value};
use chrono::prelude::*;
use chrono::Duration;
use std::collections::HashMap;
use std::time::Instant;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


#[tauri::command]
fn get_bus_json(line_id: &str, line_number: &str) -> Vec<String>{
    let mut res: Vec<String> = vec![];
    let mut linez: String = String::new();
    let response = reqwest::blocking::get(format!("https://api.um.warszawa.pl/api/action/dbtimetable_get/?id=88cd555f-6f31-43ca-9de4-66c479ad5942&busstopId={line_id}&busstopNr={line_number}&apikey=oops!")).unwrap();
    let v: Value = serde_json::from_str(&response.text().unwrap()).expect("Failed to parse");
    if let Some(result) = v["result"].as_array() {
        for r in result {
            let linez = r["values"][0]["value"].to_string().replace("\"", "");
            let response = reqwest::blocking::get(format!("https://api.um.warszawa.pl/api/action/dbtimetable_get/?id=e923fa0e-d96c-43f9-ae6e-60518c9f3238&busstopId={line_id}&busstopNr={line_number}&line={linez}&apikey=oops!")).unwrap();
            let v: Value = serde_json::from_str(&response.text().unwrap()).expect("Failed to parse");
            if let Some(result2) = v["result"].as_array() {
                res = result2.into_iter().map(|i| i.to_string()).collect::<Vec<String>>();
                
            }
        }
    }
    res.push(linez);
    res
}

#[tauri::command]
fn get_bus_times(mut bus_arr: Vec<String>) -> Vec<String>{
    let linez = bus_arr.pop().unwrap();
    let now = Instant::now();
    let mut l1: Vec<i32> = vec![];
    let mut l2: Vec<String> = vec![];
    let result2 = bus_arr.into_iter().map(|i| serde_json::from_str(&i).unwrap()).collect::<Vec<Value>>();
                for r2 in result2 {
                    //println!("testing testing 123");
                    let values2 = &r2["values"];
                    let destination = &values2[3];
                    let hour_str = &values2[5];
                    
                    //println!("{}", hour_str["value"].to_string());
                    let hour_str_parsed = hour_str["value"].to_string().split(":").collect::<Vec<&str>>().into_iter().map(|i| i.replace("\"", "").parse::<i32>().unwrap()).collect::<Vec<i32>>();
                    if hour_str_parsed[0] == 24 {
                        continue;
                    }
                    let hour = Local::now().hour() as i32;
                    let minute = Local::now().minute() as i32;
                    //println!("{} {} vs {} {}", hour, minute, hour_str_parsed[0], hour_str_parsed[1]);
                    let time_difference = ((hour_str_parsed[0]*60) + hour_str_parsed[1]) - ((hour*60)+minute);
                    if hour_str_parsed[0] - hour > 1 || time_difference > 10 ||  hour_str_parsed[0] - hour < 0 || time_difference < 0 {
                        continue;
                    }
                    
                    let mut in_minutes = "";
                    let formatted1 = format!("za {} minuty", time_difference);
                    let formatted2 = format!("za {} minut", time_difference);
                    match time_difference {
                        0 => in_minutes = "teraz",
                        1 => in_minutes = "za minutę",
                        2 | 3 | 4 => in_minutes = &formatted1,
                        _ => in_minutes = &formatted2,
                    }
                    l1.push(time_difference);
                    l2.push(format!("{linez} - {} - {in_minutes}", destination["value"].to_string()));
                    
                }
            
    let mut sorting_map = l1.into_iter().zip(l2.into_iter()).collect::<HashMap<i32, String>>();
    let mut sorted: Vec<_> = sorting_map.into_iter().collect();
    sorted.sort_by_key(|a| a.0);
    let last_array = sorted.into_iter().map(|(key, value)| value.to_string()).collect::<Vec<String>>();
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    last_array
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_bus_json, get_bus_times])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
