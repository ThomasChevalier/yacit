# yacit
Yet Another Crappy ICMP Tunnel

# Installation

```sh
cargo build
```

# Lancement
## Coté server

```sh
sudo ./target/debug/yacit --server --mtu 1500
```

## Coté client

```sh
sudo ./target/debug/yacit --remote_ip 192.168.43.238 --mtu 1500
```



# Raw socket

https://stackoverflow.com/questions/61350364/anything-better-than-resorting-to-libc-for-arbitrary-protocols-over-raw-sockets
