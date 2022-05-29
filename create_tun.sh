#!/bin/sh

ip tuntap add mode tun dev tun0
#ip addr add 10.0.3.0/24 dev tun0
#ip link set dev tun0 up
#ip route get 10.0.3.50
#ping 10.0.3.50
