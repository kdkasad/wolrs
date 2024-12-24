/*
 * Copyright (C) 2024 Kian Kasad <kian@kasad.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the “Software”), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

use std::net::{Ipv4Addr, UdpSocket};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct CliOptions {
    #[arg(value_parser = mac_from_str)]
    mac_address: u64,

    #[arg(short, long, default_value_t = String::from("255.255.255.255"))]
    ip_address: String,

    #[arg(short, long, default_value_t = 9, value_parser = clap::value_parser!(u16).range(1..))]
    port: u16
}

fn main() {
    let opts = CliOptions::parse();

    // Magic packet is 6 0xff bytes followed by the MAC address repeated 16 times.
    let macbytes = opts.mac_address.to_be_bytes();
    let mut packet = [0xff; 102];
    for i in 0..16 {
        let start = 6 + i * 6;
        packet[start..(start + 6)].copy_from_slice(&macbytes[2..]);
    }

    // Send packet
    let sock = match UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)) {
        Ok(sock) => sock,
        Err(e) => {
            eprintln!("Error: Failed to create UDP socket: {}.", e);
            return;
        }
    };
    if let Err(e) = sock.set_broadcast(true) {
        eprintln!("Error: Request for broadcast send permission failed: {}", e);
        return;
    }
    if let Err(e) = sock.send_to(&packet, (opts.ip_address.as_str(), opts.port)) {
        eprintln!("Error: Failed to send packet: {}", e);
        return;
    }
}

/// Converts a string into a MAC address.
/// Returns `Err(msg)` if the string is not a valid MAC address.
/// `msg` is a string describing the error.
fn mac_from_str(s: &str) -> Result<u64, String> {
    if s.len() == 17
        && s.chars().enumerate().all(|(i, c)| {
            if ((i + 1) % 3) == 0 {
                c == ':'
            } else {
                c.is_ascii_hexdigit()
            }
        })
    {
        Ok(u64::from_str_radix(&s.replace(':', ""), 16).unwrap())
    } else {
        Err("Argument is not a valid MAC address".to_string())
    }
}
