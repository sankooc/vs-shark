import React, { useState } from "react";
import { TabView, TabPanel } from 'primereact/tabview';
import fflate from 'fflate';
import { IHttpMessage } from "../../gen";
class Proto {
  message: IHttpMessage
  content?: Uint8Array
}
enum ContentType {
  Raw,
  TXT,
}

const getHeaderValue = (header, key): string => {
  if(!header || !key){
    return '';
  }
  for(const head of header){
    let toks = head.split(':');
    if(toks && toks.length > 1){
      const [hk, vl] = toks;
      if(hk.toLowerCase() == key.toLowerCase()){
        const [v] = vl.split(';');
        return v.trim();
      }
    }
  }
  return '';
};


const image_mime = [
  'image/apng',
'image/avif',
'image/gif',
'image/jpeg',
'image/png',
'image/svg+xml',
'image/webp',
];

const maxByte = 500;
const Content = (props: Proto) => {
  const {message, content} = props;
  if(!message){
    return <div>No Selected Message </div>
  }
  if(!content || !content.length){
    return <div> &lt;Empty&gt; </div>
  }
  let type = ContentType.Raw;
  const [txt, setTxt] = useState<string>('');

  const { headers, len } = message;
  const _type = getHeaderValue(headers, 'content-type');
  const enco = getHeaderValue(headers, 'content-encoding');
  const size = getHeaderValue(headers, 'content-length');
  let plantext = false;
  let image = false;
  if(_type.startsWith('text/')) {
    type = ContentType.TXT;
    plantext = true;
  } else {
    switch(_type){
      case 'application/javascript':
      case 'application/json':
        type = ContentType.TXT;
        plantext = true;
        break;
    }
  }
  let image_url;
  if(image_mime.indexOf(_type) >= 0) {
    image = true;
    image_url = URL.createObjectURL(
      new Blob([content.buffer], { type: _type })
    );
  }
  const convertArray = (data: Uint8Array) => {
    const len = data.length;
    const more = len > maxByte;
    const _data = data.slice(0, Math.min(maxByte, len));
    new Promise(r => {
      const reader = new FileReader()
      reader.onload = () => r(reader.result)
      reader.readAsDataURL(new Blob([_data]))
    }).then((_rs) => {
      const str = _rs as string;
      let _txt = str.slice(str.indexOf(',') + 1);
      if(more){
        _txt = `${_txt} (has more ${len - maxByte} bytes)`;
      }
      setTxt(_txt);
    });
  };


  convertArray(content);
  let complete = parseInt(size) == content.length;
  // console.log(parseInt(size), content.length, complete);
  if(!txt){
    return <div> &lt;Empty&gt; </div>
  }
  const get_to_text = () => {
    try{
      if (enco == 'gzip'){
        const _text = fflate.decompressSync(content);
        return new TextDecoder().decode(_text);
      }
    }catch(e){}
    return new TextDecoder().decode(content);
  }
  return <TabView className="w-full h-full flex flex-column" style={{padding: 0}}>
  <TabPanel header="Raw Base64" style={{padding: 0}}>
      <div className="http-content"><span className="base64-content">{txt}</span></div>
  </TabPanel>
  {plantext &&<TabPanel header="text" style={{padding: 0}}>
    <div className="http-content"><span className="content">{get_to_text()}</span></div>
  </TabPanel>}
  {image &&<TabPanel header="image" style={{padding: 0}}>
    <div className="http-content"><img src={image_url}/></div>
  </TabPanel>}
</TabView>;
};

export default Content;
