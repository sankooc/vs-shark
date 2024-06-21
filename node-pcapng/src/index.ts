import { Option, AbstractVisitor, Visitor, Packet, Protocol, IPPacket, Resolver, PVisitor, BasicElement, AbstractRootVisitor } from "./common";
import { Uint8ArrayReader, AbstractReaderCreator } from './io';
import { RootVisitor } from './pcapng';
import { DataPacket } from './dataLinkLayer';
import { IPv4, IPv6, ARP } from './networkLayer';
import { UDP, TCP, ICMP, IGMP } from './transportLayer';
import { NBNS, DNS, DHCP } from './application';

export const readBuffers = (arr: Uint8Array, batchSize: number = 10): RootVisitor => {
  const creator = new AbstractReaderCreator();
  const visitor = new RootVisitor();
  visitor.batchSize = batchSize;
  const ele = new BasicElement('root', creator, arr.length, arr);
  setTimeout(() => {
    visitor.visit(ele);
  }, 0);
  return visitor;
}
export { Option, AbstractVisitor, Visitor, Packet, Protocol, IPPacket, Resolver, PVisitor, BasicElement, AbstractRootVisitor };
export { DataPacket, IPv4, IPv6, ARP, UDP, TCP, ICMP, IGMP, NBNS, DNS, DHCP };
