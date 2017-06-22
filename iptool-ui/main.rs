/* iptool
 Rust program to do IP related tasks.

 CLI based version with text UI.

 Author: Tim Monfette
 Version: 1.0.0
*/

extern crate iptool_core;

use std::io;
use std::io::Write;

use iptool_core::utilities::helpers::str_to_int;
use iptool_core::ipv4;
use iptool_core::ipv6;

fn main() {   
    println!("\n\t\tIP Tool by Tim Monfette\n");

    let mut ans;
    loop { 
        println!("Main Menu");
        println!("---------");
        println!("1.\tValidate an IPv4 address");
        println!("2.\tIPv4 to IPv6");
        println!("3.\tIPv4 to Binary");
        println!("4.\tValidate an IPv6 address");
        println!("5.\tIPv6 to IPv4");
        println!("6.\tIPv6 to Binary");
        println!("Other.\tQuit");
        print!("\nWhat would you like to do: ");
 
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("failed to read line"); 

        let input = str_to_int(&choice);

        match input {
            1 => ans = ipv4_wrapper("validate".to_owned()),
            2 => ans = ipv4_wrapper("v6".to_owned()),
            3 => ans = ipv4_wrapper("binary".to_owned()),
            4 => ans = ipv6_wrapper("validate".to_owned()),
            5 => ans = ipv6_wrapper("v4".to_owned()),
            6 => ans = ipv6_wrapper("binary".to_owned()),
            _ => break,
        }

        println!("\n{}\n", ans);
    }

    println!("\nExiting IP Tool\n");
}

// Wrapper for calling the IPv4 functions with user input
fn ipv4_wrapper(choice: String) -> String {
    print!("\nPlease enter an IPv4 address: ");
    io::stdout().flush().unwrap();
    let mut v4 = String :: new();

    io::stdin().read_line(&mut v4)
        .expect("failed to read line");

    let ans;
    match Some(&*choice.to_string()) {
        Some("validate") => ans = ipv4::validate_ipv4(v4),
        Some("v6")       => ans = ipv4::ipv4_to_ipv6(v4),
        Some("binary")   => ans = ipv4::ipv4_binary(v4),
        _                => ans = "ERROR: You passed me something I don't recognize".to_owned(),
    }

    ans
}

// Wrapper for calling the IPv6 functions with user input
fn ipv6_wrapper(choice: String) -> String {
    print!("\nPlease enter an IPv6 address: ");
    io::stdout().flush().unwrap();
    let mut v6 = String :: new();

    io::stdin().read_line(&mut v6)
        .expect("failed to read line");

    let ans;
    match Some(&*choice.to_string()) {
        Some("validate") => ans = ipv6::validate_ipv6(v6),
        Some("v4")       => ans = ipv6::ipv6_to_ipv4(v6),
        Some("binary")   => ans = ipv6::ipv6_binary(v6),
        _                => ans = "ERROR: You passed me something I don't recognize".to_owned(),
    }

    ans
}
