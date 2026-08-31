pub struct MyRequests {
    server: String,
    headers: Vec<String>,
    body: String
}

impl MyRequests {
    pub fn exec() {
        let args = vec!["localhost:8080".to_string()];

        let response = Command::new("curl")
            .args(args.into_iter())
            .output()
            .expect("curl fehlgeschlagen");
        println!("{:?}\n", response);
    }
}
