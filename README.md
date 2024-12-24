# wolrs

A tool to trigger a Wake-on-LAN (WoL) wakeup.

(Pronounced like "walrus")

## Installation

Install from [crates.io]:
```
$ cargo install wolrs
```

Install from Git sources:
```
$ git clone https://github.com/kdkasad/wolrs
$ cd wolrs
$ cargo install --path .
```

## Usage

```
$ wolrs <mac-address>
# or
$ wolrs -i <ip_address> -p <port> <mac_address>
```

Specify the MAC address of the computer to wake up.
The [magic packet] will be sent to the broadcast address 255.255.255.255.


[crates.io]: https://crates.io/crates/wolrs
[magic packet]: https://en.wikipedia.org/wiki/Wake-on-LAN#Magic_packet
