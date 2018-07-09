use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::prelude::*;
use http::*;

mod http;
mod router;
fn main() {
    let mut server = Server::new();
    server.add_route(Method::GET, "/", root);
    server.listen("127.0.0.1:8000");
}

fn root(_req: Request) -> Response {
    let mut res = Response::new();
    res.send_message(" <!DOCTYPE html>
<html>
<head>
<title>Page Title</title>
</head>
<body>

<h1>This is a Heading</h1>
<p>This is a paragraph.</p>

</body>
</html> ");
    res
}


use router::Paths;

#[allow(dead_code)]
pub struct Server {
    get: Paths,
    head: Paths,
    post: Paths,
    put: Paths,
    delete: Paths,
    trace: Paths,
    options: Paths,
    connect: Paths,
    patch: Paths
}

#[allow(dead_code)]
impl Server {
    fn new() -> Server {
        Server {
            get: Paths::new_root(),
            head: Paths::new_root(),
            post: Paths::new_root(),
            put: Paths::new_root(),
            delete: Paths::new_root(),
            trace: Paths::new_root(),
            options: Paths::new_root(),
            connect: Paths::new_root(),
            patch: Paths::new_root()
        }
    }

    fn add_route(&mut self, method: Method, route: &str, function: fn (Request) -> Response) {
        self.method_match_mut(method).new_route(route, function);
    }

    fn get_route(&self, method: Method, route: &str) -> Option<fn (Request) -> Response> {
        self.method_match(method).router(route)
    }

    fn listen(&self, address: &str) {
        let listener = TcpListener::bind(address).unwrap();
        let mut count = 0;
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_stream(stream);
            count += 1;
            if count > 0 {
                break;
            }
        }
    }

    fn method_match(&self, method: Method) -> &Paths {
        match method {
            Method::GET => &self.get,
            Method::HEAD => &self.head,
            Method::POST => &self.post,
            Method::PUT => &self.put,
            Method::DELETE => &self.delete,
            Method::TRACE => &self.trace,
            Method::OPTIONS => &self.options,
            Method::CONNECT=> &self.connect,
            Method::PATCH => &self.patch
        }
    }

    fn method_match_mut(&mut self, method: Method) -> &mut Paths {
        match method {
            Method::GET => &mut self.get,
            Method::HEAD => &mut self.head,
            Method::POST => &mut self.post,
            Method::PUT => &mut self.put,
            Method::DELETE => &mut self.delete,
            Method::TRACE => &mut self.trace,
            Method::OPTIONS => &mut self.options,
            Method::CONNECT => &mut self.connect,
            Method::PATCH => &mut self.patch
        }
    }

    fn handle_stream(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let req = Request::new(&mut buffer);
        let res;// = Response::new();
        if let Some(val) = req {
            let handle = self.get_route(val.get_method(),
                                        val.get_route());

            res = match handle {
                Some(func) => func(val),
                None => {
                    let mut tmp = Response::new();
                    tmp.set_status(404);
                    tmp
                }
            }
        }else {
            let mut tmp = Response::new();
            tmp.set_status(404);
            res = tmp;
        }

        stream.write(&res.to_bytes()[..]).unwrap();
        stream.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_routes() {
        let mut server = Server::new();
        server.add_route(Method::GET, "/some", test);
        match server.get_route(Method::GET, "/some") {
            Some(_x) => {},
            None => panic!("Server routing error")
        }
    }

    fn test (_req: Request) -> Response {
        Response::new()
    }
}