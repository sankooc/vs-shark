import { DNSRecord, WContext, FrameInfo, TCPConversation } from 'rshark';

// import { MainProps, Frame } from './common';

// const convert(f: FrameInfo): Frame {
//   const rs = new Frame();
//   rs.
// }

// export class WASMProps extends MainProps {
//   constructor(private cxt: WContext){}
//   _frames(): Frame[] {
//     this.cxt.get_frames;
//   }
// }

export class CProto {
  frames?: FrameInfo[];
  conversations?: TCPConversation[];
  dnsRecords?: DNSRecord[];
  constructor(public ctx: WContext){}
  public getFrames(): FrameInfo[] {
    if(!this.frames){
      this.frames = this.ctx.get_frames();
    }
    return this.frames;
  }
  public getConversations(): TCPConversation[]{
    if(!this.conversations) {
      this.conversations = this.ctx.get_conversations();
    }
    return this.conversations;
  }
  public getDNSRecord(): DNSRecord[]{
    if(!this.dnsRecords) {
      this.dnsRecords = this.ctx.get_dns_record();
    }
    return this.dnsRecords;
  }
}

export class MainProto {
  instance: CProto;
}

export { DNSRecord }