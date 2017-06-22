/* iptool_core
 Rust program to do IP related tasks.

 Single run script; uses parameters to run
 the tasks.

 Author: Tim Monfette
 Version: 1.0.0
*/

extern crate iptool_core;

use std::env::args;
use std::ascii::AsciiExt;

use iptool_core::ipv4;
use iptool_core::ipv6;

fn main() {
    let args: Vec<_> = args().collect();
    let mut ans = String::new();

    if args.len() !=3 {
        print_usage();
    } else {
        match Some(&*args[1].to_string().to_ascii_lowercase()) {
            Some("valid_ipv4")  => ans = ipv4::validate_ipv4(args[2].to_string()),
            Some("ipv4_ipv6")   => ans = ipv4::ipv4_to_ipv6(args[2].to_string()),
            Some("ipv4_binary") => ans = ipv4::ipv4_binary(args[2].to_string()),
            Some("valid_ipv6")  => ans = ipv6::validate_ipv6(args[2].to_string()),
            Some("ipv6_ipv4")   => ans = ipv6::ipv6_to_ipv4(args[2].to_string()),
            Some("ipv6_binary") => ans = ipv6::ipv6_binary(args[2].to_string()),
            _                   => print_usage(),
        }
    }

    if !ans.is_empty() {
        println!("{}", ans);
    }
}

// Print usage for the tool
fn print_usage() {
    println!("\nIP Tool Usage\n");
    println!("Refer to the following guidelines for running the tool:");
    println!("  -- Validate IPv4: ./target/release/iptool valid_ipv4 <address>");
    println!("  -- IPv4 to IPv6: ./target/release/iptool ipv4_ipv6 <address>");
    println!("  -- IPv4 to binary: ./target/release/iptool ipv4_binary <address>");
    println!("  -- Validate IPv6: ./target/release/iptool valid_ipv6 <address>");
    println!("  -- IPv6 to IPv4: ./target/release/iptool ipv6_ipv4 <address>");
    println!("  -- IPv6 to binary: ./target/release/iptool ipv6_binary <address>\n");
}
