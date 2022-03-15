use std::{collections::HashMap, error::Error, path::Path, env, fs::File, io::{self, BufRead}};

use std::fs::OpenOptions;
use std::io::Write;


enum ArgTypes {
    FilePath,
    ResultPath
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("开始查询");

    let file_path = get_file_path(ArgTypes::FilePath);
    let stored_path = get_file_path(ArgTypes::ResultPath);

    let client = reqwest::Client::new();
    let lines = read_lines(file_path)?;


    let mut stored_file = OpenOptions::new().append(true).open(stored_path).expect("Should create file before running");

    for line in lines {
        if let Ok(ip) = line {
            let result = get_result(&client, &ip).await?;
            let ll = format!("{}, {:?}\n", ip, result);
            stored_file.write_all(ll.as_bytes()).expect("write failed for");
        }      
    }   
    Ok(())
}

fn get_file_path(arg_type: ArgTypes) -> String {
    let args: Vec<String> = env::args().collect();
    let index = match arg_type {
        ArgTypes::FilePath => 1,
        ArgTypes::ResultPath => 2,
    };
    let file_path = &args[index];
    String::from(file_path)
}

fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>{
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

async fn get_result(client: &reqwest::Client, line: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let response = client.get("https://httpbin.org/ip")
    .send();
    let resp = match response.await {
        Ok(it) => it.json::<HashMap<String, String>>().await?,
        Err(err) => panic!("请求出错，{:?}", err),
    };
    println!("{}, {:?}", line, resp);

    Ok(resp)
}
