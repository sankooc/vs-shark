import { ComLog, ComMessage, HexV, Panel } from '../common';
const ele = document.getElementById("files");
const main = document.getElementById('main') as HTMLIFrameElement;

ele.addEventListener('change', function () {
  var reader = new FileReader();
  reader.onload = function () {
    const arrayBuffer: ArrayBuffer = this.result as ArrayBuffer;
    const array = new Uint8Array(arrayBuffer);
    main.contentWindow.postMessage(new ComMessage<Uint8Array>('raw-data', array), '*');
  };
  const _this = this as HTMLInputElement;
  reader.readAsArrayBuffer(_this.files[0]);
}, false);

window.addEventListener('message', function (msg: any) {
  if(msg.data.type){
    const _msg = msg.data as ComMessage<any>;
    
    const { type, body } = _msg
    try {
        switch(type){
            case 'ready':
                try{
                  console.log('ready');
                } catch(e) {
                    console.error(e);
                    // this.printLog(new ComLog('error', 'failed to open file'));
                }
                break;
            case 'log':
                console.log(body);
                break;
            default:
                console.log('unknown type', msg.type);
        }
    }catch(e){
        console.error(e);
    }
  }
});
