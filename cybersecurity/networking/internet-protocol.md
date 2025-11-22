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

2. Private IP addressing (RFC 1918)

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
