import React, { useEffect, useState } from "react";
import DTable from '../dataTable';
import { ComMessage, ITLS } from "../../common";
import { emitMessage } from "../../connect";
import './index.css';


class Props {
  items: ITLS[]
}

const suggest_ciphers = [
  "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
"TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
"TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
"TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
"TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305",
"TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305",
"TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256",
"TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256",
"TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA",
"TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
"TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA",
"TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
"TLS_RSA_WITH_AES_128_GCM_SHA256",
"TLS_RSA_WITH_AES_256_GCM_SHA384",
"TLS_RSA_WITH_AES_128_CBC_SHA256",
"TLS_RSA_WITH_AES_128_CBC_SHA",
"TLS_RSA_WITH_AES_256_CBC_SHA",
"TLS_RSA_WITH_3DES_EDE_CBC_SHA",
];

const _cipher_map = {};
for(const cp of suggest_ciphers){
  _cipher_map[cp] = 1;
}
const TLSComponent = (props: Props) => {
  const mountHook = () => {
    emitMessage(new ComMessage('tls', null));
  };
  useEffect(mountHook, []);
  const arr_to_string = (arr: string[]): string => {
    if (!arr) {
      return '';
    }
    if (arr.length === 1) {
      return arr[0]
    }
    return '[' + arr.join(', ') + ']';
  }
  const items = props.items || [];
  items.forEach((item, index) => {
    item.index = index + 1;
    const cipher = item.used_cipher;
    if (!cipher) {
      item.status = 'unkown';
    } else if (_cipher_map[cipher]) {
      item.status = 'normal';
    } else {
      item.status = 'warn';
    }
  })

  const result = {
    items,
    page: 1,
    size: items.length,
    total: items.length
  };
  const columes = [
    { body: (data: ITLS) => <span>{data.source + ' -> ' + data.target}</span>, header: 'connect' },
    { body: (data: ITLS) => <span>{arr_to_string(data.server_name)}</span>, header: 'host' },
    { body: (data: ITLS) => <span>{arr_to_string(data.support_version)}</span>, header: 'support_version' },
    { body: (data: ITLS) => <span>{arr_to_string(data.support_negotiation)}</span>, header: 'support_neg', style: { width: '8vw' } },
    { field: 'used_version', header: 'used_version', style: { width: '6vw' } },
    { field: 'used_cipher', header: 'used_cipher' },
    { body: (data: ITLS) => <span>{arr_to_string(data.used_negotiation)}</span>, header: 'used_neg', style: { width: '5vw' } },
  ]
  const _props = {
    cols: columes,
    getStyle: (item) => {
      return `status-${item.status}`
    },
    result
  };
  const onSelect = () => { };
  const scrollHeight = 100;
  return (<>
    <DTable {..._props} className="tls-page flex-grow-1" onSelect={onSelect} scrollHeight={scrollHeight}></DTable>
  </>
  );
};

export default TLSComponent;
