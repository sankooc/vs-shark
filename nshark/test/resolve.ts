import { IPPacket, Protocol, readBuffers, EtherPacket, Context } from '../src/index';
import { readLoc } from './misc';

if (process.argv.length > 2) {
  const filepath = process.argv[2];
  const tmpfile = './temp'
  const e = new EventTarget();
  readLoc(filepath, (arr: Uint8Array) => {
    const ctx: Context = readBuffers(arr);
    console.log('read complete');
    console.log('frames', ctx.getFrames().length);
    console.log('pcap/pcapng file info', ctx.getFileInfo());
    // console.log('arp replies', ctx.getARPReplies());
    // console.log('tcp connections', ctx.getTCPConnections().length);
    
    const frame = ctx.getFrames()[0];
    console.log('frame protocol', frame.protocol); // enum
    console.log('frame size', frame.getPacketSize(), 'bytes');
    console.log('frame protocol size', frame.getProtocolSize(), 'bytes');
    console.log('frame protocol pyload size', frame.getPayloadSize(), 'bytes');

    // const rs = ctx.getDNSRecord();
    // console.log(rs);
  })
}