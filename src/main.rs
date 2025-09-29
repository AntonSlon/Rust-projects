use reqwest::{header::{self, HeaderValue}, StatusCode};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, join, sync::Mutex, task, time::{self, error::Elapsed, interval, sleep, timeout, Duration}};
use std::{collections::LinkedList, fs::File, io::{Read, Write}, option, sync::Arc, thread::{JoinHandle, Thread}, time::Instant};
use serde_json::Result;
use serde::{Deserialize, Serialize};

extern crate reqwest;

#[derive(Serialize)]
struct ResponseData{
    status: u16,
    date: String,
    sever: String,
    resoponse_time: Duration
}

#[derive(Deserialize)]
struct Sites{
    urls: Vec<String>
}

fn parse_json() -> Vec<String>{
    let json = File::open("C:/Users/mrkar/OneDrive/Рабочий стол/Rust-projects/src/sites.json").expect("Open file error");
    let parsed_json: Sites = serde_json::from_reader(json).expect("deserialization error");
    return parsed_json.urls;
}

fn header_to_string(data: Option<&HeaderValue>) -> String{
    return data
    .and_then(|f|f
    .to_str().ok())
    .unwrap().to_string();
}

async fn parse_site(site: String){
    let time_start= Instant::now();
    let response = reqwest::get(site).await.unwrap();
    let time_end = time_start.elapsed();
    let header = response.headers();

    let response_data = ResponseData{
        status: response.status().as_u16(),
        date: header_to_string(header.get(header::DATE)),
        sever: header_to_string(header.get(header::SERVER)),
        resoponse_time: time_end
    };

    let json_response_data = serde_json::to_string(&response_data).expect("Serialization error");

    let mut file = File::open("C:/Users/mrkar/OneDrive/Рабочий стол/Rust-projects/responseData.json").unwrap();
    file.write_all(json_response_data.as_bytes()).unwrap();

    println!("{:?}", json_response_data);
    println!("{:?}", time_end);
    println!("{:?}", std::thread::current().id());
}

#[tokio::main]
async fn main(){
    let start = Instant::now();
    let mut task_vec: Vec<tokio::task::JoinHandle<()>> = Vec::new();
    let site = parse_json();
    
    for i in 0..parse_json().len(){
        let task = tokio::spawn(parse_site(site[i].clone()));
        task_vec.push(task);
    }

    for task in task_vec{
        task.await.unwrap();
    }

    println!("{:?}", start.elapsed());
}