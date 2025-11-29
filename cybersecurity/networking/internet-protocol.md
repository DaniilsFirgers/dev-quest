# Adrress space

**Problems**:

When the internet was created in 1980s there was no idea of billion of computers and IoT devices and routers had kilobytes of RAM, therefore, 32 bits IP address made sense (~4.3 billion addresses).

Also, it did not allow every device to have a separate IP address (private IP addresses to the rescue)

**Solutions**:

1. Network Address Translation (NAT)

   It allows **many devices on the private network** to share one **public IP**.

   Types of NAT are: **static**, **dynamic** and **Port Address Translation**. Most popular **PAT** (home setup), which is where IPs share one public IP and are distringuished by **ports**.

   NAT uses translation tables to track translated requests/responses. Opened public ports **cannot be hijacked** because only responses from **that sever** are allowed back. Acts like a **firewall** by a default.

   Looks like this:

   **Port forwarding** allows to manually tell the NAT router to **forward** from **public IP:PORT** to **private IP:PORT**. It bypasses NAT rule of only **outgoing connections can get replies**. But you can **restrict** which IP address can use the opened port!

   Theoretically in the private network of many devices, ports of a **single public IP can be exhausted**. For example, 300 devices \* 200 TCP connections = 60,000 (limit is roughly 64,000). Use **multiple** public IPs!

   **CGNAT** (Carier-Grade NAT) - when one IP address is shared among various customers and each customer is given a unique port.

2. Private IP addressing (RFC 1918)

A **private IP address** is an IP thta is used **inside a private network** and is **not routable on the public internet**.

| Class | Range                         | CIDR notation  |
| ----- | ----------------------------- | -------------- |
| A     | 10.0.0.0 - 10.255.255.255     | 10.0.0.8       |
| B     | 172.16.0.0 - 172.31.255.255   | 172.16.0.0/12  |
| C     | 192.168.0.0 - 192.168.255.255 | 192.168.0.0/16 |

- Different classes let networks pick an appropriate size without wasting IPs.

- **CIDR** (Classless Inter-Domain Routing) is a modern way to represent IP ranges using a **slash** `/` notation. Where `/ prefix_length` is numer of bits in the network portion of the IP. So, `/24` says first 24 bits is for **network** (basically, the network is `192.168.1.x`) and last 8 bits for **hosts** (2^8 = 256 possible addresses (0-255)).

- IPv4 is represented in 4 groups of 8 bits like `11000000 10101000 00000001 00000001` (192.168.1.1). Here each bit represents a power of 2 starting from the right most with 2^0 and going to the left most increasing the power **twofold**.

```
1   1   0   0   0   0   0   0
128 64  32  16  8   4   2   1
```

3. Dynamic Host Configuration Protocol (DHCP)

4. IPv6 introduction (128-bit addresses)

# Inefficient routing

**Problems**:

Classful addressing and its limitations (A, B, C classes)

Routing table growth

**Solutions**:

Classless Inter-Domain Routing (CIDR)

Hierarchical IP allocation

Route aggregation and summarization

# Security limitations

**Problems**:

Lack of inherent security in IPv4

IP spoofing and packet tampering

**Solutions**:

IPsec (Authentication Header, Encapsulating Security Payload)

Firewalls

Virtual Private Networks (VPNs)
