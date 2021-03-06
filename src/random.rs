#![allow(non_snake_case)]

use atoi::atoi;
use rand::prelude::*;
use std::str;

pub static MAX_BLOCK_SIZE: u32 = 1024; //Please do not exceed this value, or the webserver will deny your request
pub static ENABLE_DEBUG: bool = false; //Enable temporarily if you're encountering problems with fetching the data
pub static MAX_RETRY_COUNT: u8 = 10; //In the case the https stream is interrupted (I've had this happen quite frequenctly), increase this value. 10 should be more than enough for a stable connection

fn get_raw_data(length: u32) -> String {
    let mut url: String =
        String::from("https://qrng.anu.edu.au/API/jsonI.php?length={}&type=uint8");
    url = url.replace("{}", length.to_string().as_str());

    if ENABLE_DEBUG {
        println!("Url: {}", url);
    }

    let data = reqwest::get(url.as_str());
    String::from(data.unwrap().text().unwrap().as_str())
}

fn substring(arr: String, start: &str, end: &str) -> String {
    let start_idx = arr.find(start).unwrap();
    let end_idx = arr.find(end).unwrap();
    let ret: String = arr
        .chars()
        .skip(start_idx + 1)
        .take(end_idx - start_idx - 1)
        .collect();
    ret
}

pub fn next_u8s(len: u32) -> Option<Vec<u8>> {
    if len <= 1024 {
        return get_bytes(len);
    }

    let mut r_bytes: Vec<u8> = Vec::new();
    let mut amt_left = len;
    while amt_left > MAX_BLOCK_SIZE {
        let next_vals: Vec<u8> = get_bytes(MAX_BLOCK_SIZE)?;
        for v in next_vals.iter() {
            r_bytes.push(*v);
        }
        amt_left -= MAX_BLOCK_SIZE;
    }
    let next_vals: Vec<u8> = get_bytes(amt_left)?;
    for v in next_vals.iter() {
        r_bytes.push(*v);
    }
    Some(r_bytes.to_owned())
}

pub fn next_u16s(len: u32) -> Option<Vec<u16>> {
    let mut rng = rand::thread_rng();
    let mut arr: Vec<u16> = Vec::new();
    let ref_arr: Vec<u8> = next_u8s(len * 2)?;

    let mut idx: usize = 0;
    while idx < ref_arr.len() {
        let y: u16 = rng.gen();
        let vector_combined = format!("{:02x}{:02x}", ref_arr[idx], ref_arr[idx + 1]);
        let v = u16::from_str_radix(&vector_combined, 16).unwrap();
        arr.push(y ^ v);
        idx += 2;
    }

    Some(arr.to_owned())
}

pub fn next_u32s(len: u32) -> Option<Vec<u32>> {
    let mut rng = rand::thread_rng();
    let mut arr: Vec<u32> = Vec::new();
    let ref_arr: Vec<u16> = next_u16s(len * 2)?;

    let mut idx: usize = 0;
    while idx < ref_arr.len() {
        let y: u32 = rng.gen();
        let vector_combined = format!("{:02x}{:02x}", ref_arr[idx], ref_arr[idx + 1]);
        let v = u32::from_str_radix(&vector_combined, 16).unwrap();
        arr.push(y ^ v);
        idx += 2;
    }

    Some(arr.to_owned())
}

pub fn next_u64s(len: u32) -> Option<Vec<u64>> {
    let mut rng = rand::thread_rng();
    let mut arr: Vec<u64> = Vec::new();
    let ref_arr: Vec<u32> = next_u32s(len * 2)?;

    let mut idx: usize = 0;
    while idx < ref_arr.len() {
        let y: u64 = rng.gen();
        let vector_combined = format!("{:02x}{:02x}", ref_arr[idx], ref_arr[idx + 1]);
        let v = u64::from_str_radix(&vector_combined, 16).unwrap();
        arr.push(y ^ v);
        idx += 2;
    }

    Some(arr.to_owned())
}

pub fn next_u128s(len: u32) -> Option<Vec<u128>> {
    let mut rng = rand::thread_rng();
    let mut arr: Vec<u128> = Vec::new();
    let ref_arr: Vec<u64> = next_u64s(len * 2)?;

    let mut idx: usize = 0;
    while idx < ref_arr.len() {
        let y: u128 = rng.gen();
        let vector_combined = format!("{:02x}{:02x}", ref_arr[idx], ref_arr[idx + 1]);
        let v = u128::from_str_radix(&vector_combined, 16).unwrap();
        arr.push(y ^ v);
        idx += 2;
    }
    Some(arr.to_owned())
}

fn get_bytes(len: u32) -> Option<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut store: Vec<u8> = Vec::new();
    let mut raw_data = get_raw_data(len);
    let mut retries: u8 = 0;

    while !raw_data.contains("\"success\":true") {
        retries += 1;
        if retries >= MAX_RETRY_COUNT {
            return None;
        }
        raw_data = get_raw_data(len);
    }

    if ENABLE_DEBUG {
        println!("Success code received! {} ", raw_data);
    }

    raw_data = substring(raw_data, "[", "]");

    if ENABLE_DEBUG {
        println!("Substring: {}", raw_data);
    }

    let parts: Vec<&str> = raw_data.split(",").collect();
    if parts.len() != len as usize {
        eprintln!("Error: bytes expected not equal to the input length");
        return None;
    }
    for byte in parts.iter() {
        let y: u8 = rng.gen();
        store.push(atoi::<u8>(byte.as_bytes()).unwrap() ^ y);
    }

    Some(store.to_owned())
}
