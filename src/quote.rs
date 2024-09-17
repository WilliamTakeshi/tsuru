use core::str;

use chrono::NaiveTime;

#[derive(Debug, Clone)]
pub struct Quote {
    pub accept_time: NaiveTime,
    issue_code: String,
    bids: Vec<(u64, u64)>,
    asks: Vec<(u64, u64)>,
}


pub fn parse_packet(data: &[u8]) -> Option<Quote> {
    let prefix = b"B6034";

    if let Some((_, packet)) = split_once_subslice(data, prefix) {
        // Check if we have the whole packet
        assert!(&packet[209] == &0xff);

        // Extract accept time, issue code, bids, and asks from the packet
        let accept_time_str =
            str::from_utf8(&packet[201..209]).expect("Failed to parse accept time");
        let accept_time = parse_time(accept_time_str);
        let issue_code = str::from_utf8(&packet[0..12])
            .expect("Failed to parse issue code")
            .to_string();

        let bids: Vec<_> = (0..5)
            .map(|i| {
                let start_price = i * 12 + 24;
                let start_qty = i * 12 + 29;
                let price = str::from_utf8(&packet[start_price..start_price + 5])
                    .expect("Failed to parse bid price")
                    .parse::<u64>()
                    .expect("Failed to parse bid price");
                let qty = str::from_utf8(&packet[start_qty..start_qty + 7])
                    .expect("Failed to parse bid qty")
                    .parse::<u64>()
                    .expect("Failed to parse bid qty");
                (qty, price)
            })
            .collect();

        let asks: Vec<_> = (0..5)
            .map(|i| {
                let start_price = i * 12 + 91;
                let start_qty = i * 12 + 96;
                let price = str::from_utf8(&packet[start_price..start_price + 5])
                    .expect("Failed to parse ask price")
                    .parse::<u64>()
                    .expect("Failed to parse ask price");
                let qty = str::from_utf8(&packet[start_qty..start_qty + 7])
                    .expect("Failed to parse ask qty")
                    .parse::<u64>()
                    .expect("Failed to parse ask qty");
                (qty, price)
            })
            .collect();

        Some(Quote {
            accept_time,
            issue_code,
            bids,
            asks,
        })
    } else {
        None
    }
}

fn parse_time(time_str: &str) -> NaiveTime {
    // Because chrono does not support parsing microseconds directly, we need to parse it manually
    let hhmmss = &time_str[0..6];
    
    let micros_str = &time_str[6..8];
    let micros: i64 = micros_str.parse::<i64>().expect("Invalid microseconds") * 10_000; // Multiply by 10_000 to shift to the correct place

    // Parse the HHMMSS part
    let time_without_micros = NaiveTime::parse_from_str(hhmmss, "%H%M%S")
        .expect("Failed to parse time");
    
    // Add the microseconds to the parsed time
    time_without_micros + chrono::Duration::microseconds(micros)
}

fn split_once_subslice<'a>(data: &'a [u8], pattern: &[u8]) -> Option<(&'a [u8], &'a [u8])> {
    // Find the first occurrence of the pattern in the data
    if let Some(pos) = find_subslice(data, pattern) {
        // Split the data into two parts: before the pattern and after
        let before = &data[..pos];
        let after = &data[pos + pattern.len()..];
        Some((before, after))
    } else {
        // If the pattern is not found, return None
        None
    }
}

fn find_subslice(data: &[u8], pattern: &[u8]) -> Option<usize> {
    data.windows(pattern.len())
        .position(|window| window == pattern)
}

pub fn print_quote(quote: &Quote) {
    print!(
        "{} {}",
        quote.accept_time.format("%H:%M:%S%.6f"), quote.issue_code
    );
    for (qty, price) in quote.bids.iter().rev() {
        print!(" {}@{}", qty, price);
    }
    for (qty, price) in &quote.asks {
        print!(" {}@{}", qty, price);
    }
    println!();
}
