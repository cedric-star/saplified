use std::process::Command;
use std::fs;

mod open_api;

struct Curl {
    base_cmd: String,
    args: Vec<String>,
}

fn main() {
    println!("Hello, world!");

    let my_curl = Curl {
        base_cmd: String::from("curl"),
        args: vec!["localhost:8080".to_string()],

    };

    let response = Command::new(&my_curl.base_cmd)
        .args(&my_curl.args)
        .output()
        .expect("curl fehlgeschlagen");

    let output = str::from_utf8(&response.stdout).expect("converting to utf8 failed");

    println!("out: {output}");

    let path = String::from("./api.yml");
    let yml = read_yml(&path);

    let parsed_api = open_api::OpenApi::from_yml_string(&yml);
    println!("\n\napi parsed: \n{}", parsed_api.to_string());
}

fn read_yml(path: &String) -> String {
    let contents: String = match fs::read_to_string(path) {
        Ok(content) => {
            //println!("{path}:\n{content}");
            content
        },
        Err(e) => {
            println!("error while reading file path {path}, msg: {e}");
            "".to_string()
        },
    };

    contents
}
