### Routing in Linux

**IPV4** addresses are limited to approximately 2^32 = 4,294,967,296 as IPV4 is 32 bit number, and some of the IP addresses are reserved:

- 10.0.0.0/24 ( private, internal networks like LAN )
- 192.168.0.0/16 ( private, home networks like Routers )
- 169.254.0.0/16 ( link-local auto-config, which can gives devices on LAN ability to reach one anoter in case router is down, or it is not configured. PC to printer )
- 127.0.0.0/8 ( loopback address )
- 203.0.113.0/24 ( Documentation IP for educational purposes, which is not routable )
- 224.0.0.0/4 ( multicast like video streaming )

Considering IoT and smart devices, IPV4 would be already depleted. Two solutions were came up with - IPV6 and NAT. **IPV6** has 2^128 ip addresses, which is a lot.

## NAT

Network Adress Translation or NAT is a routing technique used in **LAN** (Local area network) to route traffic among devices on the same local network. By doing this, we have only one central point, which is router or switch (for one device) that has access to the internet, and all devices are assigned their unique local IP address.

- Dynamic host configuration protocol or **DHCP** server is responsible for private IP's districbution to devices on the LAN by sending device IP, subnet mask and default gateway address.

- The **subnet mask** tells which part of the IP address is the host and which is the network. It divides IP addresses into sub-networks for a better management and control over the traffic. For example, subnet mask 255.255.255.0 has /24 notation and can have 256 different IP addresses (2^8, where 8 is what is left for the network and is 8 bits), while /16 can have 65,534 and is designed for larger networks.

- Media Access Control or **MAC** address is a unique hardware identifier for a network interface card (NIC). It is layer 2 (data link) in OSI and used by WI-Fi or Ethernet networks. To be able to build an Ethernet frame we need a destination and source MAC addresses. In order for my PC to know the router MAC address the address resolution protocal is or ARP is used. It basically asks “Who has 192.168.1.1?”. After first destinaton MAC address discovery, it is typically cached on the machine that has requested it.
