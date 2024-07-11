import { Context, Option, AbstractVisitor, Visitor, Packet, Protocol, IPPacket, PVisitor, EtherPacket, InputElement, FileType, TCPStack, TCPConnect, CNode, ARPReply } from "./common";
import { AbstractReaderCreator } from './common/io';
import { PCAPNGVisitor } from './pcapng';
import { DataPacket } from './specs/dataLinkLayer';
import { IPv4, IPv6, ARP, IPPack } from './specs/networkLayer';
import { UDP, TCP, ICMP, IGMP } from './specs/transportLayer';
import { NBNS, DNS, DHCP, HttpPT } from './specs/application';
import { TLS } from './specs/tls';
import { PCAPVisitor } from './pcap';
import { AbstractRootVisitor, Resolver } from './specs';


export const checkFileType = (arr: Uint8Array): FileType => {
  const dataView = new DataView(arr.buffer, 0, 4);
  const code = dataView.getUint32(0, true).toString(16);
  switch (code) {
    case 'a1b2c3d4':
      return FileType.PCAP;
    case 'a0d0d0a':
      return FileType.PCAPNG;
  }
  return null;
};

export const readBuffers = (arr: Uint8Array): AbstractRootVisitor => {
  const ftype = checkFileType(arr);
  const creator = new AbstractReaderCreator();
  const ele = new InputElement(arr, creator);
  let visitor;
  switch (ftype) {
    case FileType.PCAP:
      visitor = new PCAPVisitor();
      break;
    case FileType.PCAPNG:
      visitor = new PCAPNGVisitor();
      break;
    default:
      throw new Error('unexpect data format!');
  }
  visitor.visit(ele);
  return visitor;
}
export { Context, Option, AbstractVisitor, Visitor, Packet, Protocol, IPPacket, Resolver, PVisitor, AbstractRootVisitor, FileType, TCPStack, TCPConnect, CNode, ARPReply };

export { EtherPacket, DataPacket, IPv4, IPv6, ARP, UDP, TCP, ICMP, IGMP, NBNS, DNS, DHCP, IPPack, HttpPT, TLS };

