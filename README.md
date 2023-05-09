# wglogd
Logging daemon for WireGuard peer handshakes

## Whats is this?

This is a **Debian systemd service** that is meant to **log WireGuard VPN handshakes**.
It polls the peer information from WireGuard at a set interval and logs new handshakes
if it detects a new handshake has been made since the last poll.


## Why do I need this?

If you want to **monitor your VPN connections**, you can do so with iptables, nftables
and their respective logs. However, this will only tell you what external IPs are
transmitting via WireGuard and the VPN-internal traffic, but not the connection from
external IP to internal IP.

Therefore, in order to properly **accociate VPN IPs with their public IPs (and thus: geo location)**,
you need a service like this, that can intercept the peer handshakes and log them separately.
