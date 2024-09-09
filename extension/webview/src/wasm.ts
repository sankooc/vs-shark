import { DNSRecord, WContext, FrameInfo, TCPConversation } from 'rshark';

import { IDNSRecord } from './common';
import { PCAPClient } from './client';

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
export class DNSProps {
  constructor(public ctx: WContext, public dnsRecords: IDNSRecord[]) { }
}
export { DNSRecord }