# Open Systems Interconections model

- The **OSI model** is a **conecptual framework** used to understand how computer networks communicate. Created by **ISO** in 1984 and deviced networking into **7 layers**, each with a specific function.

1. **Physical layer** job is to **transmit raw bits (0s and 1s) over a phuysical medium.** It does **not** understand what the bits mean - that is handled by higher layers.

It defines:

- How bits are represented (voltage, light, radio waves)
- Physical connections (cables, connections, wireless channels)
- Transmission speed and timing
- Physical topology of the network

Devices operating:

- Hubs
- Repeaters
- Cables, NIC's

2. **Data link layer** job is to move data between devices on the same local network and ensure error-free frame delivery. It is **"local delivery + MAC addressing + framing"** layer.

It provides:

- Framing (turning raw bits into structured frames)
- MAC addressing
- Error detection
- Access control (who gets to send data on the network)
- Local network communication

!! It does **not** route between networks - that is Layer 3 !!

Protocols used:

- Ethernet
- Wi-Fi
- Switching technologies

3. **Network layer** is responsible for **moving packets across different networks**, choosing the best path from the source to the destination. It uses **IP addresses**, then decides the best path for packets and forwards them to the destination IP. If necessary, the Network Layer splits packet into smaller pieces.

Devices at this layer:

- Routers
- Layer 3 switches

4. **Transport layer** provides **end-to-end communication** between devices and ensures that data is delivered **reliably, correctly and in the right order**. Think of it as the **traffic manager** of the network stack. Transport layer introduces **port numbers**, which let multiple applications communicate over the network at the same time.

Key responsibilities:

- Segmentation & reassembly
- End-to-end reliability
- Flow control
- Error control
- Connection control

5. **Session layer** ensures that two communicating systems can establish a **session**, exchange data reliably, and close the session when done.

Key responsibilities:

- Session establishment (sets up connection and keeps it running)
- Dialog control (who can send data at what time)
- Synchronization (if a connection drops, can resume from the last checkpoint instead of restarting)
- Session recovery (helps to restart the session if an interruption occurs)

Functions that do it:

- RPC
- SQL\*Net (database session management)
- NetBIOS

6. **Presentation layer** ensures that data from the sending device can be properly **interpreted** by the receiving device, even if the systems use different data formats. Examples are:

- TLS/SSL
- MIME
- JPEG
- JSON, XML
- MP3, MP4

7. **Application layer** is an interface between the end-user applications (like browsers and email clients) and the network, enabling them to **send/receive** data using protocols (HTTP, SMTP, FTP) for different tasks. It is not the application itself, but rather the **protocol** that facilitates the communication.
