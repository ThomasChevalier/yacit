# yacit - Yet Another Crappy ICMP Tunnel

yacit is a simple icmp tunnel that forwards all the traffic of the client to the server via ICMP Echo Request (aka ping).

Goals of this project:
* Gain experience with Rust
* Try to configure everything via system calls, without the help of existing programs (such as iproute2)
* Create a working prototype in a time-constrained context

The logic for the VPN is largely inspired from [davlxd's excellent demo](https://github.com/davlxd/simple-vpn-demo).

# Installation

```sh
cargo build
```

# Running
## Server side

```sh
nft 'flush ruleset
  table inet nat {
        chain postrouting {
                type nat hook postrouting priority filter; policy accept;
                  ip saddr 10.0.0.0/16 ip daddr != 10.0.0.0/16 masquerade
        }
        chain forward{
           type filter hook forward priority 0; policy accept
        }
  }'
```

```sh
sudo ./target/debug/yacit --mtu 1500 --server --internal-ip 10.0.0.1
```

## Client side

```sh
nft 'flush ruleset
  table inet nat {
        chain postrouting {
                type nat hook postrouting priority filter; policy accept;
                oifname "yacit0" masquerade
        }
        chain forward{
           type filter hook forward priority 0; policy accept
        }
  }'
```

```sh
sudo ./target/debug/yacit --mtu 1500 --remote-ip <SERVER IP> --out-iface-name <OUTPUT INTERFACE NAME>
```
