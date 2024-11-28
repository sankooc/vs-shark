<p align="center">
  <img src="assets/icon.png" width="200px" alt="VS-Shark Logo">
</p>
<p align="center">
  <a href="/">
    <img src="https://img.shields.io/github/license/sankooc/vs-shark" alt="License">
  </a>
  <a href="https://marketplace.visualstudio.com/items?itemName=sankooc.pcapviewer">
    <img src="https://img.shields.io/visual-studio-marketplace/i/sankooc.pcapviewer" alt="Install Count">
  </a>
  <a href="https://marketplace.visualstudio.com/items?itemName=sankooc.pcapviewer">
    <img src="https://img.shields.io/visual-studio-marketplace/d/sankooc.pcapviewer" alt="Download Count">
  </a>
  <a href="https://marketplace.visualstudio.com/items?itemName=sankooc.pcapviewer">
    <img src="https://img.shields.io/visual-studio-marketplace/last-updated/sankooc.pcapviewer" alt="Last Updated">
  </a>
  <a href="/">
    <img src="https://img.shields.io/github/languages/count/sankooc/vs-shark" alt="Language Count">
  </a>
  <a href="/">
    <img src="https://img.shields.io/visual-studio-marketplace/stars/sankooc.pcapviewer" alt="Marketplace Stars">
  </a>
  <a href="/">
    <img src="https://img.shields.io/npm/unpacked-size/nshark" alt="Unpacked Size">
  </a>
  <a>
    <img src="https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fshark%2Fhit-counter&count_bg=%2379C83D&title_bg=%23555555&icon=rust.svg&icon_color=%23E7E7E7&title=visitor&edge_flat=false)" alt="Visitor Badge">
  </a>
</p>

## Overview

**PcapViewer** is an open-source packet analysis tool designed to efficiently handle PCAP/PCAPNG (Packet Capture) files. It enables users to dive into network traffic and inspect various protocol layers, from the physical Ethernet layer to the application layer, providing a streamlined alternative to full-scale tools like Wireshark. With its lightweight nature, it’s ideal for developers, network administrators, and engineers who need quick insights into network packets.


### Key Features

- **Protocol Support**:
  - **File Formats**: PCAP, PCAPNG
  - **Link Layer**: Ethernet II, PPPoE, IEEE 1905.1a, SSL
  - **Network Layer**: IPv4, IPv6, ARP, RARP, ICMP, IGMP
  - **Transport Layer**: TCP, UDP
  - **Application Layer**: TLS, HTTP, DNS, DHCP, SSDP

### Online Demo

Try out [PcapViewer](https://sankooc.github.io/vs-shark/app/) in your browser.


### Features in Action

#### Demo

<img src="https://i.ibb.co/x6NNTBp/cap2.gif" width="80%" alt="PcapViewer Demo"/>  




<details>
  <summary>IP Packet Overview</summary>
  <img src="assets/overview.png" width="80%" alt="IP Packet Overview"/>
</details>


<details>
  <summary>IP Packet List</summary>
  <img src="assets/frame.png" width="80%" alt="IP Packet List"/>
</details>


<details>
  <summary>TCP Conversations</summary>
  <img src="assets/conv.png" width="80%" alt="TCP Conversations"/>
</details>


<details>
  <summary>HTTP Connection</summary>
  <img src="assets/http.png" width="80%" alt="HTTP Connection"/>
</details>


<details>
  <summary>DNS Response</summary>
  <img src="assets/dns.png" width="80%" alt="DNS Response"/>
</details>



### VS Code Extension

Install the **PcapViewer** extension from the [VSCode Marketplace](https://marketplace.visualstudio.com/items?itemName=sankooc.pcapviewer) for seamless integration into your Visual Studio Code environment.



### ZED Extension

> Coming soon...


### TUI 

![](https://i.ibb.co/WVPTzP1/tui3.gif)

#### Install

`cargo install --git https://github.com/sankooc/vs-shark --branch master --bin pcaps`

#### Usage

`pcaps -f <filepath>`

`pcaps --help`


### Contribution

We welcome contributions! If you'd like to contribute, please check out the guidelines (coming soon) and open a pull request or issue.



### Contact

For any inquiries or suggestions, feel free to reach out to us (details coming soon).




