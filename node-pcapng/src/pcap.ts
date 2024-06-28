import { Uint8ArrayReader, AbstractReaderCreator } from './io';
import { Option, AbstractVisitor, Visitor, Packet, Protocol, IPPacket, Resolver, PVisitor, InputElement, AbstractRootVisitor, EtherPacket, FileInfo } from "./common";
import { DataLaylerVisitor } from './dataLinkLayer';

export class PCAPVisitor extends AbstractRootVisitor {
  linktype!: number;
  major!: number;
  minor!: number;
  visitor: DataLaylerVisitor = new DataLaylerVisitor();
  getFileInfo(): FileInfo {
    const info = new FileInfo();
    info.linkType = this.linktype;
    info.majorVersion = this.major;
    info.minorVersion = this.minor;
    return info;
  }
  _visit(ele: InputElement): void {
    const { readerCreator, content } = ele;
    const reader = readerCreator.createReader(content, 'root', false);
    const magic = reader.read32Hex();
    const major = reader.read16();
    const minor = reader.read16();
    reader.skip(8);
    const snapLen = reader.read32(false);
    reader.skip(2);
    const linktype = reader.read16();
    this.major = major;
    this.minor = minor;
    this.linktype = linktype;
    do {
      const highTS = reader.read32(true);
      const lowTS = reader.read32(true);
      const ts = highTS * 1000 + Math.floor(lowTS / 1000);
      const captured = reader.read32();
      const origin = reader.read32();
      const _packet = reader.slice(origin);
      
      const data = this.createEtherPacket(readerCreator.createReader(_packet, 'frame', false));
      data.captured = captured;
      data.origin = origin;
      data.ts = ts;
      data.nano = highTS * 1000000 + lowTS;
      // this.addPacket(data.accept(this.visitor));
      this.addPacket(data);
    } while (reader.eof());
  }
}