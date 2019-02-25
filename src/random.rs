use std::str;
use rand::prelude::*;
use atoi::atoi;

pub static MAX_BLOCK_SIZE: u32 = 1024;
pub static ENABLE_DEBUG: bool = false;
pub static MAX_RETRY_COUNT: u8 = 10;


fn getRawData(length: u32) -> String {
    let mut url: String = String::from("https://qrng.anu.edu.au/API/jsonI.php?length={}&type=uint8");
    url = url.replace("{}", length.to_string().as_str());

    if ENABLE_DEBUG {
        println!("Url: {}", url);
    }

    let data = reqwest::get(url.as_str());
    String::from(data.unwrap().text().unwrap().as_str())
}

fn substring(arr: String, start: &str, end: &str) -> String {
    let startIdx = arr.find(start).unwrap();
    let endIdx = arr.find(end).unwrap();
    let ret: String = arr.chars().skip(startIdx + 1).take(endIdx - startIdx - 1).collect();
    ret
}


pub fn next_u8s(len: u32) -> Box<Vec<u8>> {
    if len <= 1024 {
        return getBytes(len);
    }

    let mut rBytes: Vec<u8> = Vec::new();
    let mut amtLeft = len;
    while amtLeft > MAX_BLOCK_SIZE {
        let nextVals: Vec<u8> = *getBytes(MAX_BLOCK_SIZE);
        for v in nextVals.iter() {
            rBytes.push(*v);
        }
        amtLeft -= MAX_BLOCK_SIZE;
    }
    let nextVals: Vec<u8> = *getBytes(amtLeft);
    for v in nextVals.iter() {
        rBytes.push(*v);
    }
    Box::new(rBytes)
}


pub fn next_u32s(len: u32) -> Box<Vec<u32>>{
    let mut rng = rand::thread_rng();
    let mut arr: Vec<u32> = Vec::new();
    let refArr: Vec<u16> = *next_u16s(len*2);

    let mut idx: usize = 0;
    while idx < refArr.len() {
        let y: u32 = rng.gen();
        let vector_combined = format!("{:02x}{:02x}", refArr[idx], refArr[idx+1]);
        let v = u32::from_str_radix(&vector_combined, 16).unwrap();
        arr.push(y ^ v);
        idx += 2;
    }

    Box::new(arr)
}

pub fn next_u64s(len: u32) -> Box<Vec<u64>>{
    let mut rng = rand::thread_rng();
    let mut arr: Vec<u64> = Vec::new();
    let refArr: Vec<u32> = *next_u32s(len*2);

    let mut idx: usize = 0;
    while idx < refArr.len() {
        let y: u64 = rng.gen();
        let vector_combined = format!("{:02x}{:02x}", refArr[idx], refArr[idx+1]);
        let v = u64::from_str_radix(&vector_combined, 16).unwrap();
        arr.push(y ^ v);
        idx += 2;
    }

    Box::new(arr)
}

pub fn next_u128s(len: u32) -> Box<Vec<u128>>{
    let mut rng = rand::thread_rng();
    let mut arr: Vec<u128> = Vec::new();
    let refArr: Vec<u64> = *next_u64s(len*2);

    let mut idx: usize = 0;
    while idx < refArr.len() {
        let y: u128 = rng.gen();
        let vector_combined = format!("{:02x}{:02x}", refArr[idx], refArr[idx+1]);
        let v = u128::from_str_radix(&vector_combined, 16).unwrap();
        arr.push(y ^ v);
        idx += 2;
    }

    Box::new(arr)
}

pub fn next_u16s(len: u32) -> Box<Vec<u16>>{
    let mut rng = rand::thread_rng();
    let mut arr: Vec<u16> = Vec::new();
    let refArr: Vec<u8> = *next_u8s(len*2);

    let mut idx: usize = 0;
    while idx < refArr.len() {
        let y: u16 = rng.gen();
        let vector_combined = format!("{:02x}{:02x}", refArr[idx], refArr[idx+1]);
        let v = u16::from_str_radix(&vector_combined, 16).unwrap();
        arr.push(y ^ v);
        idx += 2;
    }

    Box::new(arr)
}

fn getBytes(len: u32) -> Box<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut store : Vec<u8> = Vec::new();
    let mut rawData = getRawData(len);
    let mut retries: u8 = 0;
    //TODO: implement try system here
    while !rawData.contains("\"success\":true") {
        retries += 1;
        if retries >= MAX_RETRY_COUNT {
            return Box::new(store);
        }
        rawData = getRawData(len);
    }

    if ENABLE_DEBUG{
        println!("Success code received! {} ", rawData);
    }

    rawData = substring(rawData, "[", "]");

    if ENABLE_DEBUG {
        println!("Substring: {}", rawData);
    }

    let parts: Vec<&str> = rawData.split(",").collect();
    if parts.len() != len as usize {
        eprintln!("Error: bytes expected not equal to the input length");
        return Box::new(store);
    }
    for byte in parts.iter() {
        let y: u8 = rng.gen();
        store.push(atoi::<u8>(byte.as_bytes()).unwrap() ^ y);
    }


    Box::new(store)
}