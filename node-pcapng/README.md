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
  const frame = ctx.getFrames()[0];
  console.log('frame protocol', frame.protocol); // enum
  console.log('frame size', (frame as EtherPacket).captured);

```


## Change Log

  - 0.1.0
    * support file format PCAP/PCAPNG
    * support protocol IPV4/IPV6/ARP/TCP/UDP/ICMP/ICMPv6/DNS/NBNS/HTTP/DHCP

## Acknowledgement

  - [pcapng](https://www.ietf.org/archive/id/draft-tuexen-opsawg-pcapng-05.html)
  - [pcap](https://www.ietf.org/archive/id/draft-gharris-opsawg-pcap-00.html)
  - [IPV4](https://en.wikipedia.org/wiki/Internet_Protocol_version_4)
  - [ARP](https://en.wikipedia.org/wiki/Address_Resolution_Protocol)
  - [DNS](https://datatracker.ietf.org/doc/html/rfc1035)
  - [NBNS](https://www.ietf.org/rfc/rfc1002.txt)
  - [ICMP](https://en.wikipedia.org/wiki/Internet_Control_Message_Protocol)
  - [ICMPv6](https://en.wikipedia.org/wiki/Internet_Control_Message_Protocol)
  - [IGMP](https://en.wikipedia.org/wiki/Internet_Group_Management_Protocol)
  - [UDP](https://en.wikipedia.org/wiki/Transmission_Control_Protocol)
  - [TCP](https://en.wikipedia.org/wiki/User_Datagram_Protocol)
  
