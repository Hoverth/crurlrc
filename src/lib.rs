use native_tls::TlsConnector;
use std::{collections::HashMap, io::prelude::*, net::TcpStream};
use text_io::scan;

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub url: String,
    pub body: Option<String>,
    pub headers: HashMap<String, String>,
}

#[derive(Debug)]
pub struct URLResult {
    pub status: u16,
    pub url: String,
    pub redirect_to: String,
}

pub fn is_redirect(resp: &Response) -> bool {
    let s: i16 = resp.status as i16 - 300;
    (0 < s) && (s < 99)
}

pub fn get_redirect_url(resp: Response) -> String {
    resp.headers.get("Location").unwrap().to_owned()
}

pub fn check_url(url: &str) -> Option<URLResult> {
    let mut result = URLResult {
        status: 0,
        url: url.to_string(),
        redirect_to: String::new(),
    };
    match get(url) {
        Ok(c) => {
            if is_redirect(&c) {
                result.status = c.status;
                result.redirect_to = get_redirect_url(c);
                return Some(result);
            } else if c.url != url {
                result.status = c.status;
                result.redirect_to = String::from(c.url);
                return Some(result);
            }
        }
        Err(_) => {}
    }
    None
}

fn get(url: &str) -> std::io::Result<Response> {
    let original_url = String::from(url);
    let (port, using_ssl) = if url.contains("https") {
        (443, true)
    } else {
        (80, false)
    };
    let url = if url.contains("://") {
        url.split("://").into_iter().nth(1).unwrap()
    } else {
        url
    };

    let server = url.split('/').into_iter().next().unwrap_or(url);
    let path = url.strip_prefix(&server).unwrap_or("/");
    let server = if !server.contains(":") {
        String::from(server) + ":" + &port.to_string()
    } else {
        String::from(server)
    };

    let request = format!("HEAD {} HTTP/1.0\r\nUser-Agent: crurlrc\r\n\r\n", path,);
    let request = &request.into_bytes();
    let res = if using_ssl {
        let connector = TlsConnector::new().unwrap();
        let stream = TcpStream::connect(&server)?;
        let server = server.strip_suffix(":443").unwrap();
        let mut stream = connector.connect(&server, stream).unwrap();
        let _ = stream.write_all(request);
        let mut res = vec![];
        stream.read_to_end(&mut res).unwrap();
        res
    } else {
        let mut stream = TcpStream::connect(server)?;
        let _ = stream.write(request);
        let mut res = vec![];
        stream.read_to_end(&mut res).unwrap();
        res
    };

    let response = String::from_utf8(res.clone()).unwrap();
    let (http_ver, status_code, code_name): (f32, u16, String);
    scan!(res.into_iter() => "HTTP/{} {} {}\r\n", http_ver, status_code, code_name);
    let headers_body = response.replace(
        format!("HTTP/{http_ver:.1} {status_code} {code_name}\r\n").as_str(),
        "",
    );

    let (headers, body): (String, String) = {
        let v = headers_body.split("\r\n\r\n").collect::<Vec<&str>>();
        (String::from(v[0]), String::from(v[1]))
    };

    let mut processed_headers: HashMap<String, String> = HashMap::new();
    for line in headers.split("\r\n") {
        let (key, val) = {
            let v = line.split_whitespace().collect::<Vec<&str>>();
            let key = v[0].strip_suffix(":").unwrap();
            let value = v[1..].join(" ");
            (String::from(key), String::from(value))
        };
        processed_headers.insert(key, val);
    }

    let r = Response {
        status: status_code,
        url: original_url,
        headers: processed_headers,
        body: {
            if !body.is_empty() {
                Some(body)
            } else {
                None
            }
        },
    };
    Ok(r)
}
