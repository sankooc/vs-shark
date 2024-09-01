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
  constructor(public ctx: WContext){}
  public getFrames(): FrameInfo[] {
    if(!this.frames){
      this.frames = this.ctx.get_frames();
    }
    return this.frames;
  }
  public getConversations() {
    if(!this.conversations) {
      this.conversations = this.ctx.get_conversations();
    }
    return this.conversations;
  }
}

export class MainProto {
  instance: CProto;
}

export { DNSRecord }