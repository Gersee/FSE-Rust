#[macro_use] extern crate nickel;

use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult};
use std::collections::HashMap;

fn root_page <'a> (_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::<&str, &str>::new();

    //Fill placeholder
    data.insert("page_title", "Root site");

    //Choose template and add data
    res.render("app/views/root_page.tpl", &data)
}

fn show_result <'a> (req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::<&str, &str>::new();

    let name: &str = req.param("sendname").unwrap();

    //Fill placeholders
    data.insert("name", name);
    data.insert("page_title", "Response site");

    //Choose template and add data
    res.render("app/views/response.tpl", &data)
}

fn main() {
    let mut server = Nickel::new();

    //Logging each request
    server.utilize(middleware! { |request|
        println!("Requested url: {:?}", request.origin.uri);
    });

    let mut router = Nickel::router();
    router.get("/", root_page);
    router.post("/sendData", show_result);

    server.utilize(router);
    server.listen("127.0.0.1:8080");
}
