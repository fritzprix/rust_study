extern crate hyper;
extern crate futures;
extern crate tokio;
extern crate hyper_rustls;
extern crate clap;
extern crate webpki_roots;

mod hls;

use std::{sync::Arc, io::Read};
use clap::Parser;
use hyper::{Client, StatusCode};
use hyper::body::{HttpBody as _, Buf};
use hls::m3u8::{self, Playlist};



#[derive(Parser)]
struct Args {
    
    #[arg(short = 'H')]
    hostname: String,
}

const TEST_HOST_URL: &'static str = "https://demo.unified-streaming.com/k8s/features/stable/video/tears-of-steel/tears-of-steel.ism/.m3u8";

fn main() {

    let args = Args::parse();
    let m3u8_playlst = Playlist::from(args.hostname);

    
    
    println!("{:?}", m3u8_playlst);

}
