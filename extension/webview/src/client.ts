import { PCAPClient, ComMessage, ComLog, Panel, Frame, CTreeItem, TCPCol, Grap, Category, GrapNode, GrapLink, MainProps, OverviewSource, HexV } from "./common";
import { WContext, FrameInfo } from 'rshark';
import { DNSRecord } from 'rshark';

export class Statc {
  size: number = 0;
  count: number = 0;
  start!: number;
  end!: number;
  stc: Map<string, number> = new Map();
  public addLable(label: string, packet: FrameInfo): void {
    const count = this.stc.get(label) || 0;
    const size = packet.len || 0;
    this.stc.set(label, count + size);
  }
  public static create(ts: number, per: number) {
    const item = new Statc();
    item.start = ts;
    item.end = ts + per;
    return item;
  }
}

const parseTime = (time: number): string => {
  const date = new Date(time);
  const [hour, minutes, seconds, ms] = [
    date.getHours(),
    date.getMinutes(),
    date.getSeconds(),
    date.getMilliseconds()
  ];
  return `${minutes}:${seconds} ${ms}`;
}
