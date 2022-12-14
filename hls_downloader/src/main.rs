/** 
 * Test URL
 *  - https://devstreaming-cdn.apple.com/videos/streaming/examples/img_bipbop_adv_example_fmp4/master.m3u8
 *  - https://demo.unified-streaming.com/k8s/features/stable/video/tears-of-steel/tears-of-steel.ism/.m3u8
 */


extern crate hyper;
extern crate futures;
extern crate tokio;
extern crate hyper_rustls;
extern crate clap;
extern crate webpki_roots;

mod hls;

use clap::Parser;
use hls::m3u8::master::Playlist;




#[derive(Parser)]
struct Args {
    
    #[arg(short = 'H')]
    hostname: String,
}

const TEST_HOST_URL: &'static str = "https://demo.unified-streaming.com/k8s/features/stable/video/tears-of-steel/tears-of-steel.ism/.m3u8";

fn main() {

    let args = Args::parse();
    let playlist = Playlist::try_from(args.hostname.as_str()).unwrap();
    for stream in playlist.get_variants() {
        println!("{:?}", stream);
    }

    
    
    println!("{:?}", playlist);

}
