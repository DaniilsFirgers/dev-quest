# Routing

**IPV4** addresses are limited to approximately 2^32 = 4,294,967,296 as IPV4 is 32 bit number, and some of the IP addresses are reserved:

> **10.0.0.0/24** ( private, internal networks like LAN )
>
> **192.168.0.0/16** ( private, home networks like Routers )
>
> **169.254.0.0/16** ( link-local auto-config, which can gives devices on LAN ability to reach one anoter in case router is down, or it is not configured. PC to printer )
>
> **127.0.0.0/8** ( loopback address )
>
> **203.0.113.0/24** ( Documentation IP for educational purposes, which is not routable )
>
> **224.0.0.0/4** ( multicast like video streaming )

Considering IoT and smart devices, IPV4 would be already depleted. Two solutions were came up with - IPV6 and NAT. **IPV6** has 2^128 ip addresses, which is a lot.

## NAT

Network Adress Translation or **NAT** is a routing technique used in **LAN** (Local area network) to route traffic among devices on the same local network. By doing this, we have only one central point, which is router or switch (for one device) that has access to the internet, and all devices are assigned their unique local IP address.

- Dynamic host configuration protocol or **DHCP** server is responsible for private IP's distribution to devices on the LAN by sending device IP, subnet mask and default gateway address.

- The IP address given by a router is **leased**, not given forever. Before the lease expires, the device will try to renew the lease, if lease is not renewed, the IP goes back into the pool. On Linux you can check it out via:

```
sudo cat /var/lib/dhcp/dhclient.leases
```

- The **subnet mask** tells which part of the IP address is the host and which is the network. It divides IP addresses into sub-networks for a better management and control over the traffic. For example, subnet mask 255.255.255.0 has /24 notation and can have 254 different IP addresses (2^8 - 2, where 8 is what is left for the network and is 8 bits while two for broadcast and network), while /16 can have 65,534 and is designed for larger networks.

- Media Access Control or **MAC** address is a unique hardware identifier for a network interface card (NIC). It is layer 2 (data link) in OSI and used by WI-Fi or Ethernet networks. To be able to build an Ethernet frame we need a destination and source MAC addresses. In order for my PC to know the router MAC address the address resolution protocal is or ARP is used. It basically asks “Who has 192.168.1.1?”. After first destinaton MAC address discovery, it is typically cached on the machine that has requested it.

- When a request is routed from the router to the ISP it both assigns a private port (56001) to a private IP (192.168.1.104) for this specific request and opens a public port (43022) on the router (85.254.32.100), and writes it down to the **NAT table**. Then the request source replies to 85.254.32.100:43022 and router looks up the NAT table and knows whom to rewrite the destination. Same logic applies to ISP.

| Public IP     | Public Port | Private IP    | Private Port |
| ------------- | ----------- | ------------- | ------------ |
| 85.254.32.100 | 43022       | 192.168.1.104 | 56001        |

- **0.0.0.0** means that server accepts connections on all available IPv4 network interfaces. Meaning, from localhost and from LAN IP 192.168.x.x.

## Routing commands

The following command will show a routing table with numeric output:

```
route -n
```

| Destination | Gateway       | Genmask       | Flags | Metric | Ref | Use | Iface   |
| ----------- | ------------- | ------------- | ----- | ------ | --- | --- | ------- |
| 0.0.0.0     | 192.168.1.254 | 0.0.0.0       | UG    | 600    | 0   | 0   | wlp2s0  |
| 169.254.0.0 | 0.0.0.0       | 255.255.0.0   | U     | 1000   | 0   | 0   | wlp2s0  |
| 172.17.0.0  | 0.0.0.0       | 255.255.0.0   | U     | 0      | 0   | 0   | docker0 |
| 192.168.1.0 | 0.0.0.0       | 255.255.255.0 | U     | 600    | 0   | 0   | wlp2s0  |

1. First is the default route for **any traffic not matching another route**. Used for accessing the internet of any IP **outside your local networks**.
2. Failover route for link-local range (Automatic Private IP Addressing or **APIPA**) that communicates **without a DCHP sever**.
3. Docker Internal Network.
4. Local LAN for **direct communication** with other devices over Wi-Fi (no gateway needed).

The following command would give routing table as well, just in a different format:

```
ip route
```

> default via 192.168.1.254 dev wlp2s0 proto dhcp metric 600
>
> 169.254.0.0/16 dev wlp2s0 scope link metric 1000
>
> 172.17.0.0/16 dev docker0 proto kernel scope link src 172.17.0.1 linkdown
>
> 192.168.1.0/24 dev wlp2s0 proto kernel scope link src 192.168.1.104 metric 600

Some of the references:

- **wlpX** is a signle wireless interface, while **ethX** is the Ethernet card and **lo** is loopback, and **docker0** is the docker interface.
- **proto dhcp** means that this route was configured via DCHP.
- **link** means that this route is used for local communication

This command will show all network interfaces along with their IP addresses and other network-related information.

```
ip -a
```

```
2: wlp2s0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default qlen 1000
    link/ether 60:57:18:0b:dd:3a brd ff:ff:ff:ff:ff:ff
    inet 192.168.1.104/24 brd 192.168.1.255 scope global dynamic noprefixroute wlp2s0
       valid_lft 83387sec preferred_lft 83387sec
    inet6 fe80::7b6b:1ac5:2e40:744d/64 scope link noprefixroute
       valid_lft forever preferred_lft forever
```
