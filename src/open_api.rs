use regex::Regex;

#[derive(Debug)]
pub struct OpenApi {
    pub openapi: String,
    pub info: OpenApiInfo,
    pub servers: Vec<OpenApiServer>,
    pub paths: Vec<OpenApiPath>,
}
#[derive(Debug)]
pub struct OpenApiInfo {
    title: String,
    description: String,
    version: String
}
#[derive(Debug)]
pub struct OpenApiServer {
    url: String,
    description: String,
}
#[derive(Debug)]
pub struct OpenApiPath {
    path: String,
    methods: Vec<OpenApiMethod>,
}
#[derive(Debug)]
pub struct OpenApiMethod {
    method: String,
    summary: String,
    description: String,
    responses: Vec<OpenApiResponse>,
}
#[derive(Debug)]
pub struct OpenApiResponse {
    code: String,
    description: String,
    content: String,
}

pub const HTPP_METHODS: [&str; 9] = ["get", "post", "put", "delete", "head", "patch", "trace", "options", "connect"];

impl OpenApi {
    fn new(openapi: String, info: OpenApiInfo, servers: Vec<OpenApiServer>, paths: Vec<OpenApiPath>) -> OpenApi {
        OpenApi {
            openapi,
            info,
            servers,
            paths,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "openapi: {}\n{} \nservers:\n{} \npaths:\n{}",
            self.openapi,
            self.info.to_string(),
            self.servers.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("\n"),
            self.paths.iter().map(|p| p.to_string()).collect::<Vec<String>>().join("\n")
        )
    }

    fn default() -> OpenApi {
        OpenApi {
            openapi: String::new(),
            info: OpenApiInfo::default(),
            servers: Vec::new(),
            paths: Vec::new(),
        }
    }

    pub fn from_yml_string(yml: &String) -> OpenApi {
        let mut openapi_str = String::new();
        let mut info_str = String::new();
        let mut servers_str = String::new();
        let mut paths_str = String::new();

        let mut iter_info = false;
        let mut iter_servers = false;
        let mut iter_paths = false;

        for (i, line) in yml.lines().enumerate() {
            // define current state
            if line.starts_with("openapi: ") { openapi_str.push_str(rm_yml_key(line.to_string()).as_str()); }
            else if line.starts_with("info") { iter_info = true; }
            else if line.starts_with("servers") { iter_servers = true; iter_info = false;}
            else if line.starts_with("paths:") { iter_paths = true; iter_servers = false; }

            if iter_info { info_str.push_str(line); info_str.push('\n'); }
            else if iter_servers { servers_str.push_str(line); servers_str.push('\n'); }
            else if iter_paths { paths_str.push_str(line); paths_str.push('\n'); }

        }

        // prepare structs for filling with strings
        let mut info = OpenApiInfo::from_yml_string(info_str);
        let mut servers = yml_list_2_vec(servers_str)
            .into_iter()
            .map(|server| OpenApiServer::from_yml_string(server))
            .collect();
        let mut paths = yml_list_2_vec_by_separator(paths_str, "/")
            .into_iter()
            .map(|path| OpenApiPath::from_yml_string(path))
            .collect();

        OpenApi {
            openapi: openapi_str,
            info: info,
            servers: servers,
            paths: paths,
        }

    }
}

impl OpenApiInfo {
    fn new(title: String, description: String, version: String) -> OpenApiInfo {
        OpenApiInfo {
            title,
            description,
            version,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "info:\n  title: {}\n  description: {}\n  version: {}",
            self.title, self.description, self.version
        )
    }

    fn default() -> OpenApiInfo {
        OpenApiInfo {
            title: String::new(),
            description: String::new(),
            version: String::new(),
        }
    }

    fn from_yml_string(yml: String) -> OpenApiInfo {
        let mut info = OpenApiInfo::default();

        for line in yml.lines() {
            let line = rm_starting_spaces(line.to_string());
            if line.starts_with("title:") { info.title = rm_yml_key(line); }
            else if line.starts_with("description:") { info.description = rm_yml_key(line); }
            else if line.starts_with("version:") {info.version = rm_yml_key(line); }
        }

        info
    }
}

impl OpenApiServer {
    fn new(url: String, description: String) -> OpenApiServer {
        OpenApiServer {
            url,
            description,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "  - url: {}\n    description: {}",
            self.url, self.description
        )
    }

    fn default() -> OpenApiServer {
        OpenApiServer {
            url: String::new(),
            description: String::new(),
        }
    }

    fn from_yml_string(yml: String) -> OpenApiServer {

        let mut server = OpenApiServer::default();

        for line in yml.lines() {
            let l = rm_starting_spaces(line.to_string());
            if l.starts_with("url:") { server.url = rm_yml_key(l); }
            else if l.starts_with("description") { server.description = rm_yml_key(l); }
        }
        server
    }
}

impl OpenApiPath {
    fn new(path: String, methods: Vec<OpenApiMethod>) -> OpenApiPath {
        OpenApiPath {
            path,
            methods,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "  /{}:\n{}",
            self.path,
            self.methods.iter().map(|m| m.to_string()).collect::<Vec<String>>().join("\n")

        )
    }

    fn default() -> OpenApiPath {
        OpenApiPath {
            path: String::new(),
            methods: Vec::new(),
        }
    }

    fn from_yml_string(yml: String) -> OpenApiPath {
        let mut path = OpenApiPath::default();
        let mut path_str = String::new();
        for c in yml.chars() {
            if c == ':' { break; }
            path_str.push(c);
        }

        //remove path name from rest
        let methods_str: String = yml
            .lines()
            .into_iter()
            .skip(1)
            .map(|s| String::from(s))
            .collect::<Vec<String>>()
            .join("\n");

        let mut method_list: Vec<String> = Vec::new();
        let mut method_str = String::new();
        for l in methods_str.lines() {
            let mut is_new_method = false;
            for http_method in HTPP_METHODS {
                if l.starts_with(http_method) {
                    is_new_method = true;
                    break;
                }
            }

            if is_new_method && !method_str.is_empty() {
                method_list.push(method_str.clone());
                method_str.clear();
            }

            method_str.push_str(l);
            method_str.push('\n');
        }

        if !method_str.is_empty() {
            method_list.push(method_str);
        }
        OpenApiPath::new(
            path_str,
            method_list
                .into_iter()
                .map(|s| OpenApiMethod::from_yml_string(s))
                .collect::<Vec<OpenApiMethod>>(),
        )
    }
}

impl OpenApiMethod {
    fn new(method: String, summary: String, description: String, responses: Vec<OpenApiResponse>) -> OpenApiMethod {
        OpenApiMethod {
            method,
            summary,
            description,
            responses,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "    {}:\n      summary: {}\n      description: {}\n      responses: {}",
            self.method,
            self.summary,
            self.description,
            self.responses.iter().map(|r| r.to_string()).collect::<Vec<String>>().join("\n")
        )
    }

    fn default() -> OpenApiMethod {
        OpenApiMethod {
            method: String::new(),
            summary: String::new(),
            description: String::new(),
            responses: Vec::new(),
        }
    }

    fn from_yml_string(yml: String) -> OpenApiMethod {
        let mut method = OpenApiMethod::default();

        let mut res_list: Vec<String> = Vec::new();
        let mut res_str = String::new();
        let mut in_responses = false;

        for l in yml.lines() {
            let mut is_new_res = false;

            if method.method.is_empty() {
                'inner: for http_method in HTPP_METHODS {
                    if l.starts_with(http_method) {
                        method.method = http_method.to_string();
                        break 'inner;
                    }
                }
            }

            if l.starts_with("responses:") { in_responses = true; }

            if !in_responses {
                if l.starts_with("summary:") { method.summary = rm_yml_key(l.to_string()); }
                if l.starts_with("description") { method.description = rm_yml_key(l.to_string()); }
            }

            let re = Regex::new(r#"^"[0-9]{3}""#).unwrap();
            if re.is_match(l) {
                is_new_res = true;
            }

            if is_new_res && !res_str.is_empty() {
                res_list.push(res_str.clone());
                res_str.clear();
            }
            res_str.push_str(l);
            res_str.push('\n');
        }

        if !res_str.is_empty() {
            res_list.push(res_str);
        }

        method.responses = res_list
            .into_iter()
            .skip(1) // erster Block ist Vorspann (get:/summary:/description:/responses:), kein echter Response
            .map(|r| OpenApiResponse::from_yml_string(r))
            .collect::<Vec<OpenApiResponse>>();

        method
    }
}

impl OpenApiResponse {
    fn new(code: String, description: String, content: String) -> OpenApiResponse {
        OpenApiResponse {
            code,
            description,
            content,
        }
    }

    fn to_string(&self) -> String {
        format!("        \"{}\"\n          description: {}\n          content: {}",
            self.code,
            self.description,
            self.content,
        )

    }
    fn default() -> OpenApiResponse {
        OpenApiResponse {
            code: String::new(),
            description: String::new(),
            content: String::new(),
        }
    }

    fn from_yml_string(yml: String) -> OpenApiResponse {
        let mut response = OpenApiResponse::default();

        let re = Regex::new(r#"^"([0-9]{3})""#).unwrap();

        for l in yml.lines() {
            if let Some(caps) = re.captures(l) {
                response.code = caps[1].to_string();
            } else if l.starts_with("description:") {
                response.description = rm_yml_key(l.to_string());
            } else if l.starts_with("content:") {
                response.content = rm_yml_key(l.to_string());
            }
        }

        response
    }
}

fn rm_starting_spaces(line: String) -> String {
    let mut new_line = String::new();
    let mut copying = false;
    for c in line.chars() {
        if c != ' ' { copying = true; }
        if copying { new_line.push(c); }

    }

    new_line
}

fn rm_yml_key(line: String) -> String {
    let mut copying = false;
    let mut new_line = if line.starts_with(" ") { rm_starting_spaces(line) } else { line };
    let mut finished_line = String::new();
    for c in new_line.chars() {
        if copying { finished_line.push(c); }
        if !copying && c == ' ' { copying = true; }
    }

    finished_line
}
fn yml_list_2_vec_by_separator(yml: String, separator: &str) -> Vec<String> {
    let mut new_yml = yml
        .lines()
        .into_iter()
        .map(|l| rm_starting_spaces(l.to_string()))
        .collect::<Vec<String>>()
        .join("\n");

    let mut yml_list: Vec<String> = Vec::new();
    let mut to_append = String::new();
    for line in new_yml.lines() {
        if line.starts_with(separator) {
            if !to_append.is_empty() {
                yml_list.push(to_append.clone());
                to_append.clear();
            }

            to_append.push_str(&line.chars().skip(1).collect::<String>());
            //to_append.push_str(rm_starting_spaces(line.to_string()).as_str());
            to_append.push('\n');

        } else {
            to_append.push_str(rm_starting_spaces(line.to_string()).as_str());
            to_append.push('\n');
        }
    }

    if !to_append.is_empty() {
        yml_list.push(to_append);
    }

    if !yml_list.is_empty() { yml_list.remove(0); }

    yml_list
}
fn yml_list_2_vec(yml: String) -> Vec<String> {
    yml_list_2_vec_by_separator(yml, "-")
}
