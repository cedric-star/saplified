use std::process::Command;  // <-- HIER: Command importieren!
use std::collections::HashMap;
use openapi3_parser::open_api::*;
use serde_yaml;

use crate::file_handler;
use crate::my_requests;
use crate::my_requests::MyRequests;

pub fn create_requests(path: &str) -> Vec<MyRequests> {
    let path = String::from(path);
    let yml = file_handler::read_yml(&path);
    let openapi_spec: OpenApiSpec = serde_yaml::from_str(&yml).expect("falsch");

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
    my_requests::MyRequests::exec();
    Vec::new()
}

pub fn extract_methods(item: &PathItem) -> Vec<(&str, &Operation)> {
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

//tuple return: (header für content type, body (examples))
pub fn extract_bdoy(method: &Operation) -> (String, Vec<String>) {
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
                    //println!("media: {:#?}", media_type);

                if let Some (examples) = &media_type.examples {
                    for example in examples {
                        body.push(format!("{:#?}", example.1));
                    }
                }
            }
        }
    }
    (header, body)
}
