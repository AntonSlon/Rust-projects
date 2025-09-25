use reqwest::{header::{self, HeaderValue}, StatusCode};
use tokio::{io::unix::AsyncFd, join, time::{self, error::Elapsed, interval, sleep, timeout, Duration}, sync::Mutex};
use std::{collections::LinkedList, fs::File, io::Write, sync::Arc, thread::Thread, time::Instant, io::Read, option};
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
    let json = File::open("sites.json").expect("Open file error");
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

    let mut file = File::create("responseData.json").unwrap();
    file.write_all(json_response_data.as_bytes()).unwrap();

    println!("{:?}", time_end);
    println!("{:?}", std::thread::current().id());
    println!("Статус: {}", response.status());
    println!("Заголовки: {:?}", response.headers());
}

#[tokio::main]
async fn main(){
    for i in 0..parse_json().len(){
        let site = parse_json();
        println!("{:?}", std::thread::current().id());
        let task = tokio::spawn(parse_site(site[i].clone()));
        task.await;
    }
    //parse_site("https://doc.rust-lang.ru".to_string()).await;
}