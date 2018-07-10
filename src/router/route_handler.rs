use super::*;
use http::*;

impl RouteHandler {
    pub fn new() -> RouteHandler {
        RouteHandler {
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

    pub fn add_route(&mut self, method: Method, route: &str, function: fn (Request) -> Response) {
        self.method_match_mut(method).new_route(route, function);
    }

    pub fn get_route(&self, method: Method, route: &str) -> Option<fn (Request) -> Response> {
        self.method_match(method).router(route)
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
}