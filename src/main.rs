use chrono::NaiveTime;
use quote::{parse_packet, print_quote};
use core::str;
use pcap::Capture;
use structopt::StructOpt;

mod quote;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Input pcap file
    #[structopt(parse(from_os_str))]
    file: std::path::PathBuf,

    /// Re-order by quote accept time
    #[structopt(short = "r")]
    reorder: bool,
}



fn main() {
    let opt = Opt::from_args();
    let mut capture = Capture::from_file(opt.file).expect("Failed to open pcap file");

    let mut quotes = Vec::new();

    while let Ok(packet) = capture.next_packet() {
        if let Some(quote) = parse_packet(&packet.data) {
            quotes.push(quote);
        }
    }

    if opt.reorder {
        quotes.sort_by_key(|q| q.accept_time);
    }

    for quote in quotes {
        print_quote(&quote);
    }
}
