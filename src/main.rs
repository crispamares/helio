extern crate canvas;

use canvas::{Canvas, WebCanvas};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

fn main() {
    let mut ctx = WebCanvas::start(SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 8888));

    ctx.fillRect(10.0, 10.0, 20.0, 20.0);
    
    let mut events = ctx.events().iter();
    while let Some(_ev) = events.next() {}

    println!("____Stop____");
}