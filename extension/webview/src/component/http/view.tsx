import React, { useState } from "react";
import { IHttpEnity } from "../../common";

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

const maxByte = 30;
const Content = (props: Proto) => {
  // const [type, setType] = useState<ContentType>(ContentType.Raw);
  let type = ContentType.Raw;
  const [txt, setTxt] = useState<string>('');
  const { content, header, content_len } = props.message;
  const _type = getHeaderValue(header, 'content-type');
  const enco = getHeaderValue(header, 'content-encoding');
  if(_type.startsWith('text/')) {
    type = ContentType.TXT;
  } else {
    switch(_type){
      case 'application/javascript':
      case 'application/json':
        type = ContentType.TXT;
        break;
    }
  }
  // console.log(content_len);
  // console.log(_type);
  // console.log(enco);
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


  switch (type) {
    case ContentType.Raw: {
      convertArray(content);
      return <div className="http-content"><span className="base64-content">{txt}</span></div>;
    }
    case ContentType.TXT: {
      const _txt = new TextDecoder().decode(content);
      return <div className="http-content"><span className="content">{_txt}</span></div>;
    }
  }

  return <div />;
};

export default Content;
