# yacit
Yet Another Crappy ICMP Tunnel

# Installation

```sh
cargo build
```

# Lancement
## Coté server

```sh
sudo ./target/debug/yacit --server --mtu 1500 --internal-ip 10.0.0.1
```

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
## Coté client

```sh
sudo ./target/debug/yacit --remote-ip 192.168.43.238 --mtu 1500 --out-iface-name <NOM INTERFACE PHYSIQUE>
```

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


# Raw socket

https://stackoverflow.com/questions/61350364/anything-better-than-resorting-to-libc-for-arbitrary-protocols-over-raw-sockets
