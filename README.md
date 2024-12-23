# wolrs

A tool to trigger a Wake-on-LAN (WoL) wakeup.

(Pronounced like "walrus")

## Usage

```
$ wolrs <mac-address>
```

Specify the MAC address of the computer to wake up.
The [magic packet] will be sent to the broadcast address 255.255.255.255.

[magic packet]: https://en.wikipedia.org/wiki/Wake-on-LAN#Magic_packet
