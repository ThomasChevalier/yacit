#!/bin/sh

# For client :
sysctl -w net.ipv4.ip_forward=1
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


# For server:
sysctl -w net.ipv4.ip_forward=1
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
