use std::collections::HashMap;

#[derive(Debug,PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug,PartialEq)]
pub enum Method {
    GET,
    POST,
    Uninitialized
}

impl From<&str> for Method {
    fn from(q: &str) -> Method {
        match q {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::Uninitialized
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Unititialized
}

impl From<&str> for Version {
    fn from(q: &str) -> Version {
        match q {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Unititialized
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        // Initiliaze the parsed values
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path(String::from(""));
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
           if line.contains("HTTP") {
            let (method,resource,version) = process_req_lines(line);
            parsed_method = method;
            parsed_resource = resource;
            parsed_version = version;
           }
           else if line.contains(":") {
            let (key,value) = process_header_line(line);
            parsed_headers.insert(key,value);
           }
           else {
            parsed_msg_body = line;
           }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            body: String::from(parsed_msg_body),
        }
    }
}

fn process_req_lines(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (method.into(), Resource::Path(resource.to_string()),version.into())
}

fn process_header_line(s:&str) -> (String,String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }

    if let Some(v) = header_items.next() {
        value = v.trim().to_string();
    }

    (key,value)
}

// Configuration for the test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::GET);
    }
    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /greetings HTTP/1.1\r\nHost: localhost\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");

        let mut headers_expected = HashMap::new();

        headers_expected.insert(String::from("Host"),String::from("localhost"));
        headers_expected.insert(String::from("User-Agent"),String::from("curl/7.64.1"));
        headers_expected.insert(String::from("Accept"),String::from("*/*"));

        let req: HttpRequest = s.into();

        assert_eq!(req.method, Method::GET);
        assert_eq!(req.version, Version::V1_1);
        assert_eq!(req.resource, Resource::Path(String::from("/greetings")));
        assert_eq!(req.headers, headers_expected);
    

    }
}
