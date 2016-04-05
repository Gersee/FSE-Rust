#[macro_use] extern crate nickel;

use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult};
use std::collections::HashMap;
use std::io::Read;

fn root_page <'a> (_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::<&str, &str>::new();

    //Fill placeholder
    data.insert("page_title", "Root site");

    //Choose template and add data
    res.render("app/views/root_page.tpl", &data)
}

fn show_result <'a> (req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut form_data = String::new();
    req.origin.read_to_string(&mut form_data).unwrap();
    println!("Got parameters: '{}'", form_data);

    //Fill placeholders
    let mut data = HashMap::new();
    data.insert("page_title", "Response site");
    data.insert("name", &form_data);

    res.render("app/views/response.tpl", &data)
}

fn main() {
    let mut server = Nickel::new();

    //Logging each request
    server.utilize(middleware! { |request|
        //Log requested URL
        println!("Requested url: {}", request.origin.uri);
    });

    let mut router = Nickel::router();
    router.get("/", root_page);
    router.post("/sendData", show_result);

    server.utilize(router);
    server.listen("127.0.0.1:8080");
}
