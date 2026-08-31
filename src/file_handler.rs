use std::fs;

pub fn read_yml(path: &String) -> String {
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
