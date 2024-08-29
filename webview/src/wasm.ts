import { DNSRecord, WContext, FrameInfo } from 'rshark';

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
  constructor(public ctx: WContext){}
  public getFrames(): FrameInfo[] {
    if(!this.frames){
      this.frames = this.ctx.get_frames();
    }
    return this.frames;
  }
}

export class MainProto {
  instance: CProto;
}

export { DNSRecord }