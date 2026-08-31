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
    body: OpenApiRequestBody,
    header: Vec<OpenApiHeader>,
    responses: Vec<OpenApiResponse>,
}
#[derive(Debug)]
pub struct OpenApiRequestBody {
    required: bool,
    content_type: String,
}
#[derive(Debug)]
pub struct OpenApiHeader {
    name: String,
    description: String,
    header_type: String,
    example: String,
}
#[derive(Debug)]
pub struct OpenApiResponse {
    code: String,
    description: String,
    content_type: String,
}

pub const HTPP_METHODS: [&str; 9] = ["get", "post", "put", "delete", "head", "patch", "trace", "options", "connect"];

impl OpenApi {
    pub fn to_string(&self) -> String {
        format!(
            "openapi: {}\n{}\nservers:\n{}\npaths:\n{}",
            self.openapi,
            self.info.to_string(),
            self.servers.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("\n"),
            self.paths.iter().map(|p| p.to_string()).collect::<Vec<String>>().join("\n")
        )
    }

    pub fn from_yml_string(yml: &String) -> OpenApi {
        let mut openapi_str = String::new();
        let mut info_str = String::new();
        let mut servers_str = String::new();
        let mut paths_str = String::new();

        let mut iter_info = false;
        let mut iter_servers = false;
        let mut iter_paths = false;

        for line in yml.lines() {
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
        let info = OpenApiInfo::from_yml_string(info_str);
        let servers = yml_list_2_vec(servers_str)
            .into_iter()
            .map(|server| OpenApiServer::from_yml_string(server))
            .collect();
        let paths = yml_list_2_vec_by_separator(paths_str, "/")
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
                if l.starts_with(&format!("{}:", http_method)) {
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
        let mut path = OpenApiPath::default();
        path.path = path_str;
        path.methods = method_list
            .into_iter()
            .map(|s| OpenApiMethod::from_yml_string(s))
            .collect::<Vec<OpenApiMethod>>();

        path
    }
}

impl OpenApiMethod {
    fn to_string(&self) -> String {
        let mut parts: Vec<String> = Vec::new();

        parts.push(format!("    {}:", self.method));

        if !self.summary.is_empty() {
            parts.push(format!("      summary: {}", self.summary));
        }

        if !self.description.is_empty() {
            parts.push(format!("      description: {}", self.description));
        }

        if !self.header.is_empty() {
            parts.push("      headers:".to_string());
            parts.push(
                self.header
                    .iter()
                    .map(|h| h.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
        }

        if !self.body.is_empty() {
            parts.push(self.body.to_string());
        }

        if !self.responses.is_empty() {
            parts.push("      responses:".to_string());
            parts.push(
                self.responses
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
        }

        parts.join("\n")
    }

    fn default() -> OpenApiMethod {
        OpenApiMethod {
            method: String::new(),
            summary: String::new(),
            description: String::new(),
            body: OpenApiRequestBody::default(),
            header: Vec::new(),
            responses: Vec::new(),
        }
    }

    fn from_yml_string(yml: String) -> OpenApiMethod {
        let mut method = OpenApiMethod::default();

        let mut res_list: Vec<String> = Vec::new();
        let mut res_str = String::new();
        let mut in_responses = false;

        let mut reading_body = false;
        let mut body_str = String::new();

        let mut reading_headers = false;
        let mut header_list: Vec<String> = Vec::new();
        let mut header_block = String::new();

        let re = Regex::new(r#"^"[0-9]{3}""#).unwrap();

        for l in yml.lines() {

            if method.method.is_empty() {
                'inner: for http_method in HTPP_METHODS {
                    if l.starts_with(&format!("{}:", http_method)) {
                        method.method = http_method.to_string();
                        break 'inner;
                    }
                }
            }

            // section switches - decide BEFORE deciding where this line belongs
            if l.starts_with("headers:") {
                reading_headers = true;
                reading_body = false;
            } else if l.starts_with("requestBody") {
                reading_body = true;
                if reading_headers { flush_header_block(&mut header_list, &mut header_block); }
                reading_headers = false;
            }
            if l.starts_with("responses:") {
                in_responses = true;
                reading_body = false;
                if reading_headers { flush_header_block(&mut header_list, &mut header_block); }
                reading_headers = false;
            }

            if reading_body {
                body_str.push_str(l);
                body_str.push('\n');
            }

            if reading_headers && !l.starts_with("headers:") {
                let trimmed = l.trim();
                if !trimmed.is_empty() {
                    // eine neue Headerzeile (z.B. "X-Response-ID:") erkennt man daran,
                    // dass sie mit ':' endet und keine der bekannten Eigenschaften ist
                    let is_new_header = trimmed.ends_with(':')
                        && !trimmed.starts_with("description:")
                        && !trimmed.starts_with("type:")
                        && !trimmed.starts_with("example:");

                    if is_new_header {
                        flush_header_block(&mut header_list, &mut header_block);
                    }

                    header_block.push_str(l);
                    header_block.push('\n');
                }
            }

            if !in_responses && !reading_headers {
                if l.starts_with("summary:") { method.summary = rm_yml_key(l.to_string()); }
                if l.starts_with("description") { method.description = rm_yml_key(l.to_string()); }
            }

            let is_new_res = re.is_match(l);

            if is_new_res && !res_str.is_empty() {
                res_list.push(res_str.clone());
                res_str.clear();
            }
            if !reading_body && !reading_headers {
                res_str.push_str(l);
                res_str.push('\n');
            }
        }

        if !res_str.is_empty() {
            res_list.push(res_str);
        }
        flush_header_block(&mut header_list, &mut header_block);

        method.body = OpenApiRequestBody::from_yml_string(body_str);
        method.header = header_list
            .into_iter()
            .map(|h| OpenApiHeader::from_yml_string(h))
            .collect();

        method.responses = res_list
            .into_iter()
            .skip(1) // erster Block ist Vorspann (get:/summary:/description:/responses:), kein echter Response
            .map(|r| OpenApiResponse::from_yml_string(r))
            .collect::<Vec<OpenApiResponse>>();

        method
    }
}

impl OpenApiRequestBody {
    fn to_string(&self) -> String {
        let mut parts: Vec<String> = Vec::new();

        parts.push("      requestBody:".to_string());

        if self.required {
            parts.push("        required: true".to_string());
        }

        if !self.content_type.is_empty() {
            parts.push("        content:".to_string());
            parts.push(format!("          {}: {{}}", self.content_type));
        }

        parts.join("\n")
    }

    fn default() -> OpenApiRequestBody {
        OpenApiRequestBody {
            required: false,
            content_type: String::new(),
        }
    }

    fn is_empty(&self) -> bool {
        !self.required && self.content_type.is_empty()
    }

    fn from_yml_string(yml: String) -> OpenApiRequestBody {
        let mut body = OpenApiRequestBody::default();

        for l in yml.lines() {
            if l.starts_with("required:") {
                let value = rm_yml_key(l.to_string());
                body.required = value.trim() == "true";
            }
        }

        let content_part = return_yml_after_key(yml, "content:");
        body.content_type = first_yml_key(content_part);

        body
    }
}

impl OpenApiHeader {
    fn to_string(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        parts.push(format!("        {}:", self.name));

        if !self.description.is_empty() {
            parts.push(format!("          description: {}", self.description));
        }
        if !self.header_type.is_empty() {
            parts.push(format!("          type: {}", self.header_type));
        }
        if !self.example.is_empty() {
            parts.push(format!("          example: {}", self.example));
        }

        parts.join("\n")
    }

    fn default() -> OpenApiHeader {
        OpenApiHeader {
            name: String::new(),
            description: String::new(),
            header_type: String::new(),
            example: String::new(),
        }
    }

    fn from_yml_string(yml: String) -> OpenApiHeader {
        let mut header = OpenApiHeader::default();

        for line in yml.lines() {
            let l = line.trim();
            if l.is_empty() { continue; }

            if l.starts_with("description:") { header.description = rm_yml_key(l.to_string()); }
            else if l.starts_with("type:") { header.header_type = rm_yml_key(l.to_string()); }
            else if l.starts_with("example:") { header.example = rm_yml_key(l.to_string()); }
            else if l.ends_with(':') && header.name.is_empty() {
                header.name = l.trim_end_matches(':').to_string();
            }
        }

        header
    }
}

impl OpenApiResponse {
    fn to_string(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        parts.push(format!("        \"{}\":", self.code));

        if !self.description.is_empty() {
            parts.push(format!("          description: {}", self.description));
        }
        if !self.content_type.is_empty() {
            parts.push("          content:".to_string());
            parts.push(format!("            {}: {{}}", self.content_type));
        }

        parts.join("\n")
    }
    fn default() -> OpenApiResponse {
        OpenApiResponse {
            code: String::new(),
            description: String::new(),
            content_type: String::new(),
        }
    }

    fn from_yml_string(yml: String) -> OpenApiResponse {
        let mut response = OpenApiResponse::default();

        let re = Regex::new(r#"^"([0-9]{3})""#).unwrap();

        let mut reading_content = false;
        let mut content_str = String::new();

        for l in yml.lines() {
            if reading_content {
                content_str.push_str(l);
                content_str.push('\n');
            }

            if let Some(caps) = re.captures(l) {
                response.code = caps[1].to_string();
            } else if l.starts_with("description:") {
                response.description = rm_yml_key(l.to_string());
            } else if l.starts_with("content:") {
                reading_content = true;
            }
        }

        response.content_type = first_yml_key(content_str);
        response
    }
}

// schließt den aktuell gesammelten Header-Block ab und legt ihn in die Liste,
// falls er nicht leer ist
fn flush_header_block(header_list: &mut Vec<String>, header_block: &mut String) {
    if !header_block.is_empty() {
        header_list.push(header_block.clone());
        header_block.clear();
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
    let new_line = if line.starts_with(" ") { rm_starting_spaces(line) } else { line };
    let mut finished_line = String::new();
    for c in new_line.chars() {
        if copying { finished_line.push(c); }
        if !copying && c == ' ' { copying = true; }
    }

    finished_line
}

fn starts_with_without_spaces(line: &str, key: &str) -> bool {

    let my_line = rm_starting_spaces(line.to_string());

    my_line.starts_with(key)
}

fn return_yml_after_key(yml: String, key: &str) -> String{
    let mut to_return = String::new();
    let mut building = false;

    for l in yml.lines() {
        if building {
            to_return.push_str(l);
            to_return.push('\n');
        }
        if starts_with_without_spaces(l, key) {building = true; }

    }

    to_return
}

// liefert den Key der ersten nicht-leeren Zeile eines yml-Blocks, z.B. "application/json:" -> "application/json"
fn first_yml_key(yml: String) -> String {
    for line in yml.lines() {
        let l = rm_starting_spaces(line.to_string());
        if l.is_empty() { continue; }

        return match l.find(':') {
            Some(idx) => l[..idx].to_string(),
            None => l,
        };
    }

    String::new()
}

fn yml_list_2_vec_by_separator(yml: String, separator: &str) -> Vec<String> {
    /*let new_yml = yml
        .lines()
        .into_iter()
        .map(|l| rm_starting_spaces(l.to_string()))
        .collect::<Vec<String>>()
        .join("\n");
    */
    let new_yml = yml;

    let mut yml_list: Vec<String> = Vec::new();
    let mut to_append = String::new();
    for line in new_yml.lines() {
        let trimmed = rm_starting_spaces(line.to_string());

        if trimmed.starts_with(separator) {
            if !to_append.is_empty() {
                yml_list.push(to_append.clone());
                to_append.clear();
            }

            to_append.push_str(&trimmed.chars().skip(1).collect::<String>());
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
