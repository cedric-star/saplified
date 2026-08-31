use std::process::Command;
use std::fs;
use std::collections::HashMap;
use openapi3_parser::open_api::*;
//use std::Collection::HashMap;

struct Curl {
    base_cmd: String,
    args: Vec<String>,
}

struct MyRequest {
    server: String,
    headers: Vec<String>,
    body: String
}

fn build_curl(endpoint: String) {}

fn main() {
    let my_curl = Curl {
        base_cmd: String::from("curl"),
        args: vec!["localhost:8080".to_string()],
    };

    let response = Command::new(&my_curl.base_cmd)
        .args(&my_curl.args)
        .output()
        .expect("curl fehlgeschlagen");
    println!("{:?}\n", response);

    let path = String::from("./api.yml");
    let yml = read_yml(&path);

    let openapi_spec: OpenApiSpec = serde_yaml::from_str(&yml).expect("falsch");

    //println!("{:?}", openapi_spec);

    match &openapi_spec.paths {
        Some(paths_obj) => {
            for (path_url, path_item) in paths_obj {
                //println!("{:#?}", path_item);

                for method in extract_methods(path_item) {
                    let tup = extract_bdoy(method.1);

                    println!("{}: {path_url}", method.0);
                    println!("{}", tup.0);
                    println!("{:?}", tup.1);

                    println!("");
                }
            }
        }
        None => {
            println!("Keine Pfade definiert");
        }
    }
}

pub const HTPP_METHODS: [&str; 9] = ["get", "post", "put", "delete", "head", "patch", "trace", "options", "connect"];

//tupel als return: (header für content type, body (examples))
fn extract_bdoy(method: &Operation) -> (String, Vec<String>) {
    let mut header = String::new();
    let mut body = Vec::new();

    if let Some(r_body) = &method.request_body {
        if let Some(content) = &r_body.content {
            //body = content;
            for (content_type, media_type) in content {
                header = format!("Content-Type: {:?}", content_type);
                //body.push(format!("Body: {:?}", media_type.examples));
                if let Some(example) = &media_type.example {
                    //println!("example: {:#?}", example);
                    body.push(format!("{:#?}", example));
                }
                  //println!("key: {}\nval: {:#?}", k, v);
            }
        }
    }
    (header, body)
}

fn extract_methods(item: &PathItem) -> Vec<(&str, &Operation)> {
    let mut operations = Vec::new();

    if let Some(op) = &item.get { operations.push(("get", op)); }
    if let Some(op) = &item.post { operations.push(("post", op)); }
    if let Some(op) = &item.put { operations.push(("put", op)); }
    if let Some(op) = &item.delete { operations.push(("delete", op)); }
    if let Some(op) = &item.head { operations.push(("head", op)); }
    if let Some(op) = &item.patch { operations.push(("patch", op)); }
    if let Some(op) = &item.trace { operations.push(("trace", op)); }
    if let Some(op) = &item.options { operations.push(("options", op)); }

    operations

}

fn read_yml(path: &String) -> String {
    let contents: String = match fs::read_to_string(path) {
        Ok(content) => {
            content
        },
        Err(e) => {
            println!("error while reading file path {path}, msg: {e}");
            "".to_string()
        },
    };

    contents
}
