extern crate hyper;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

fn hello(_: Request, res: Response<Fresh>) {
    res.send(b"Hello World!").unwrap();
}

fn main() {
    Server::http("0.0.0.0:3000").unwrap().handle(hello).unwrap();
}
