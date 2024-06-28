import { ComLog, ComMessage, HexV, Panel } from '../common';
import { Client } from '../client';

class DemoClient extends Client {
  ele: HTMLElement;
  main: HTMLIFrameElement;
  tree: HTMLIFrameElement;
  detail: HTMLIFrameElement;
  constructor(ele: HTMLElement, main: HTMLIFrameElement, tree: HTMLIFrameElement, detail: HTMLIFrameElement) {
    super();
    this.ele = ele;
    this.main = main;
    this.tree = tree;
    this.detail = detail;
    const _super = this;
    ele.addEventListener('change', function () {
      var reader = new FileReader();
      reader.onload = function () {
        const arrayBuffer: ArrayBuffer = this.result as ArrayBuffer;
        const array = new Uint8Array(arrayBuffer);
        _super.initData(array);
        _super.init();
      };
      const _this = this as HTMLInputElement;
      reader.readAsArrayBuffer(_this.files[0]);
    }, false);
  }
  emitMessage(panel: Panel, msg: ComMessage<any>) {
    let el: HTMLIFrameElement;
    switch(panel){
      case Panel.MAIN:
        el = this.main;
        break;
      case Panel.TREE:
        el = this.tree;
        break;
      case Panel.DETAIL:
        el = this.detail;
        break;
      default:
        return;
    }
    el.contentWindow.postMessage(msg, '*');
  }
  printLog(log: ComLog) {
    console.log(log.msg);
  }
  renderHexView(data: HexV) {
    this.emitMessage(Panel.DETAIL, new ComMessage('hex-data', data));
  }
  selectFrame(no: number): void {
    const items = this.buildFrameTree(no);
    const data = this.getPacket(no);
    this.emitMessage(Panel.TREE, new ComMessage('frame', {items, data }));
    this.renderHexView(new HexV(data));
  }
}
const ele = document.getElementById("files");
const main = document.getElementById('iframe') as HTMLIFrameElement;
const tree = document.getElementById('tree') as HTMLIFrameElement;
const detail = document.getElementById('hex') as HTMLIFrameElement;


const client = new DemoClient(ele, main, tree, detail);

window.addEventListener('message', function (msg: any) {
  if(msg.data.type){
    const _msg = msg.data as ComMessage<any>;
    client.handle(_msg);
    console.log('--', _msg)
  }
});
