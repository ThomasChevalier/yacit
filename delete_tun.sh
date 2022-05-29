#!/bin/sh

ip link set dev tun0 down
ip tuntap del mode tun dev tun0