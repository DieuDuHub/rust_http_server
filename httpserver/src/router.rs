use super::handler::{Handler,PageNotFoundHandler,StaticPageHandler,WebServiceHandler};
use http::{httprequest,httprequest::HttpRequest,httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            httprequest::Method::GET => match &req.resource {
                httprequest::Resource::Path(path) => {
                    let route: Vec<&str> = path.split("/").collect();
                    match route[1] {
                        "api" => {
                            println!("Serving API for : {:?}",&req);
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            println!("Serving static page for : {:?}",&req);
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            println!("Response : {:?}",&resp);
                            let _ = resp.send_response(stream);
                        },
                    }
                }
            },
            _ => {
                println!("Serving Not found method for : {:?}",&req);
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                println!("Response : {:?}",&resp);
                let _ = resp.send_response(stream);
            }
        }
    }
}