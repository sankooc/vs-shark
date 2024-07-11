import {Visitor, Context, TCPConnect, DNSRecord, IPPacket, Metadata, FileInfo, EtherPacket, Protocol, ARPReply, CNode, InputElement, Packet } from '../common';
import { Uint8ArrayReader } from '../common/io';
import { DNS } from './application';
import { ARP } from './networkLayer';
import { TCP } from './transportLayer';

export class Resolver {
    tcpConnections: TCPConnect[] = [];
    tcpCache: Map<string, TCPConnect> = new Map();
    arpMap: Map<string, Set<string>> = new Map();
    dnsRecord: DNSRecord[] = [];
    flush(key: string): void {
      if (!key) {
        this.tcpCache.forEach((value) => {
          this.tcpConnections.push(value);
        })
        this.tcpCache.clear();
        return;
      }
      const connect = this.tcpCache.get(key);
      this.tcpCache.set(key, null);
      if (connect) {
        this.tcpConnections.push(connect)
      }
    }
  }

export abstract class AbstractRootVisitor implements Visitor, Context {
    resolver: Resolver = new Resolver();
    readonly packets: IPPacket[] = []
    index: number = 0;
    readonly metadata: Metadata = new Metadata();
    getFrames(): IPPacket[] {
      return this.packets;
    }
    getFrame(inx: number): IPPacket {
      return this.packets[inx - 1];
    }
    getCurrentIndex(): number {
      return this.index;
    }
    abstract getFileInfo(): FileInfo;
    protected getNextIndex(): number {
      this.index += 1;
      return this.index;
    }
  
    createEtherPacket(reader: Uint8ArrayReader): EtherPacket {
      return new EtherPacket(reader, this, this.getNextIndex());
    }
    protected addPacket(packet: IPPacket): void {
      const epack = packet.getProtocol(Protocol.ETHER) as EtherPacket;
      if (epack) {
        const nano = epack.nano;
        if (this.packets.length == 0) {
          this.metadata.peroid[0] = nano;
        }
        this.metadata.peroid[1] = nano;
      }
      this.packets.push(packet);
    };
    public getContext(): Context {
      return this;
    }
    getMetadata(): Metadata {
      return this.metadata;
    }
    getDNSRecord(): DNSRecord[] {
      return this.resolver.dnsRecord;
    }
    resolveDNS(p: DNS): void {
      if (p.isResponse()) {
        const ip = p.getIpProvider().getSourceIp().getAddress();
        const port = p.getPortProvider().getSourcePort();
        const source = `${ip}:${port}`;
        for (const answer of p.answers) {
          switch (answer.getType()) {
            case 'A':
            case 'CNAME':
              this.resolver.dnsRecord.push(new DNSRecord(source, answer));
              break;
          }
        }
      }
    }
    resolve(p: ARP): void {
      const { oper } = p;
      if (oper === 2) {
        const sourceKey = `${p.senderMac}@${p.senderIp.getAddress()}`;
        let list = this.resolver.arpMap.get(sourceKey);
        if (!list) {
          list = new Set();
          this.resolver.arpMap.set(sourceKey, list);
        }
        list.add(`${p.targetMac}@${p.targetIp.getAddress()}`);
      }
    }
    resolveTCP(p: TCP): TCPConnect {
      if (p.rst) return null;
      const resolver = this.resolver;
      const payloadSize = p.getPayloadSize()
      const noContent = !p.syn && p.ack && !p.psh && payloadSize < 9;
      p.hasContent = !noContent;
      const [arch, ip1, port1, ip2, port2] = p.mess();
      const key = `${ip1}${port1}-${ip2}${port2}`;
      let connect = resolver.tcpCache.get(key);
      if (!connect) {
        if (noContent) return null;
        connect = new TCPConnect(ip1, port1, ip2, port2);
        resolver.tcpCache.set(key, connect);
      }
      const sequence = p.sequence;
      const nextSequence = (p.syn || p.fin) ? p.sequence + 1 : p.sequence + payloadSize
      const stack = connect.getStack(arch);
      const dump = stack.checkDump(sequence, nextSequence);
      p.isDump = dump;
      connect.count += 1;
      connect.total += p.getPacketSize();
      connect.tcpSize += payloadSize;
      if (dump) {
        return;
      }
      if (stack.next > 0 && stack.next != sequence) {
        p.missPre = true;
        stack.clearSegment();
      }
      connect.tcpUse += payloadSize;
      connect.countUse += 1;
      stack.sequence = sequence;
      stack.next = nextSequence;
      const stackRec = connect.getStack(!arch);
      stackRec.ack = p.acknowledge;
      if (noContent) {
        return null;
      }
      return connect;
      // if (p.ack) {
  
      // }
      // if (p.ack && !p.psh) {
      //     if (p.packet.length > 10) {
      //         const len = p.getProtocol(Protocol.ETHER).packet.length;
      //     }
      // }
      // if (p.psh) {
      // }
    }
    getTCPConnections(): TCPConnect[] {
      return this.resolver.tcpConnections;
    }
    getARPReplies(): ARPReply[] {
      const arp = this.resolver.arpMap;
      const hostnames = arp.keys();
      const rs: ARPReply[] = [];
      arp.forEach((values, hostname) => {
        const [mac, ip] = hostname.split('@');
        const reply = new ARPReply(new CNode(ip, mac));
        values.forEach((val) => {
          const [mac, ip] = val.split('@');
          reply.clients.push(new CNode(ip, mac));
        });
        rs.push(reply);
      });
      return rs;
    }
  
    getHTTPConnects(): void { }
    abstract _visit(ele: InputElement): void;
    visit(ele: InputElement): Packet {
      const { readerCreator, content } = ele;
      const start = Date.now();
      this._visit(ele);
      const per = Date.now() - start;
      this.resolver.flush(null);
      return null;
    }
  }