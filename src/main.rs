use std::process::Command;
use std::fs;
use std::collections::HashMap;
use openapi3_parser::open_api::*;
use serde_yaml;

mod fill_curl;
mod file_handler;
mod my_requests;

pub const HTPP_METHODS: [&str; 9] = ["get", "post", "put", "delete", "head", "patch", "trace", "options", "connect"];

fn main() {
    let requests = fill_curl::create_requests("api.yml");

}
