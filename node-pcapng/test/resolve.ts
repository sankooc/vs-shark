// import { RootVisitor } from '../src/pcapng';
// import { DebugReaderCreator } from '../src/io';
// import { BasicElement } from "../src/common";
import { IPPacket, Protocol, readBuffers, IPv4 } from '../src/index';
import { readLoc } from './misc';
if (process.argv.length > 2) {
  const filepath = process.argv[2];
  const tmpfile = './temp'
  const e = new EventTarget();
  readLoc(filepath, (arr: Uint8Array) => {
    console.log('start---');
    const root = readBuffers(arr, 500, (packet: IPPacket[]) => {
      console.log('--', packet.length);
    });
    console.log('finish--');
    // root.addEventListener('init', () => {
    //   console.log('start---');
    // })
    // root.addEventListener('finish', () => {
    //   console.log('finsh---');
    // })
  
    // root.addEventListener('frame', (evt: CustomEvent<IPPacket[]>) => {
    //   const items = evt.detail;
    //   console.log('--')
    //   for(const p of items){
    //     const ip = (p.getProtocal(Protocol.IPV4) || p.getProtocal(Protocol.IPV6)) as IPv4;
    //     if(ip){
    //       console.log(p.getIndex(), p.protocol,p.getProtocal(Protocol.ETHER).packet.length, ip.source, ip.target,p.toString());
    //     } else {
    //       console.log(p.getIndex(), p.protocol,p.getProtocal(Protocol.ETHER).packet.length, p.toString());
    //     }
    //   }
    // });
    // const creator = new DebugReaderCreator(tmpfile);
    // const visitor = new RootVisitor();
    // const ele = new BasicElement('root', creator, arr.length, arr);
    // visitor.visit(ele)
  })
}