import React, { useState } from "react";
import { IHttpEnity } from "../../common";
import { TabView, TabPanel } from 'primereact/tabview';
import fflate from 'fflate';
class Proto {
  message: IHttpEnity
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

const maxByte = 500;
const Content = (props: Proto) => {
  const {message} = props;
  if(!message){
    return <div>No Selected Message </div>
  }
  if(!message.content || !message.content.length){
    return <div> &lt;Empty&gt; </div>
  }
  let type = ContentType.Raw;
  const [txt, setTxt] = useState<string>('');

  const { content, header, content_len } = props.message;
  const _type = getHeaderValue(header, 'content-type');
  const enco = getHeaderValue(header, 'content-encoding');
  const size = getHeaderValue(header, 'content-length');
  let plantext = false;
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
  const convertArray = (data: Uint8Array) => {
    const len = data.length;
    const more = len > maxByte;
    const _data = data.slice(Math.min(maxByte, len));
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
  // console.log(enco);
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
  return <TabView className="w-full detail-tab" style={{padding: 0}}>
  <TabPanel header="Raw Base64" style={{padding: 0}}>
      <div className="http-content"><span className="base64-content">{txt}</span></div>
  </TabPanel>
  {plantext &&<TabPanel header="text" style={{padding: 0}}>
    <div className="http-content"><span className="content">{get_to_text()}</span></div>
  </TabPanel>}
</TabView>;
};

export default Content;
