# NODE-Potocols

pcap/pcapng analyzer written by pure Nodejs

## Install

`pnpm install protocols`

## Basic Usage

```typescript

  import { IPPacket, Protocol, readBuffers, IPv4, EtherPacket, Context } from 'protocols';
  const raw: Uint8Array = getData();
  const ctx: Context = readBuffers(raw);
  console.log('read complete');
  console.log('frames', ctx.getFrames().length);
  console.log('pcap/pcapng file info', ctx.getFileInfo());
  console.log('arp replies', ctx.getARPReplies());
  console.log('tcp connections', ctx.getTCPConnections().length);

```

## Support Protocol

- Ethernet II
- IPv4
- IPv6
- ARP
- TCP
- UDP
- ICMP
- ICMPv6
- IGMP
- DNS
- DHCP
- TLS
- HTTP

## Change Log

  - 0.1.0
    * support file format PCAP/PCAPNG
    * support protocol IPV4/IPV6/ARP/TCP/UDP/ICMP/ICMPv6/DNS/NBNS/HTTP/DHCP

## Acknowledgement

  - [pcapng](https://www.ietf.org/archive/id/draft-tuexen-opsawg-pcapng-05.html)
  - [pcap](https://www.ietf.org/archive/id/draft-gharris-opsawg-pcap-00.html)
