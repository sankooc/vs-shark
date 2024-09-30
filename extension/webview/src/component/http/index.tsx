import React, { useEffect, useState } from "react";
import "./index.css";
import { ComMessage, IHttp, IHttpEnity } from "../../common";
import { emitMessage } from "../../connect";
import Mult from '../input/select';
import { filter } from 'lodash';
import SubComponnet from './subview';

class Proto {
  items: IHttp[];
}
const toSim = (entity: IHttpEnity) => {
  return `${entity.host}:${entity.port}`;
}
const __method = (item: IHttp): string => item.method;
const __status = (item: IHttp): string => item.status;
const __source = (item: IHttp): string => toSim(item.req);
const __dest = (item: IHttp): string => toSim(item.res);

const createFilter = (filter: string[], f_getter: (item: IHttp) => string): (item: IHttp) => boolean => {
  if (filter.length === 0) {
    return (_: IHttp) => true;
  }
  let ff = new Set(filter);
  return (item: IHttp) => ff.has(f_getter(item));
};
const HttpComponnet = (props: Proto) => {
  const mountHook = () => {
    emitMessage(new ComMessage('http', null));
  };
  useEffect(mountHook, []);
  const [f_methods, setMethods] = useState<any[]>([]);
  const [f_status, setStatus] = useState<any[]>([]);
  const [f_source, setSource] = useState<any[]>([]);
  const [f_dest, setDest] = useState<any[]>([]);
  const [opts, setOpts] = useState([[], [], [], []]);
  useEffect(() => {
    const _methods = new Set<string>();
    const _status = new Set<string>();
    const _source = new Set<string>();
    const _dest = new Set<string>();
    props.items.forEach((item: IHttp, index: number) => {
      item.index = index + 1;
      _methods.add(__method(item));
      _status.add(__status(item));
      _source.add(__source(item));
      _dest.add(__dest(item));
    });
    const methods = Array.from(_methods);
    const statuses = Array.from(_status);
    const sources = Array.from(_source);
    const dests = Array.from(_dest);
    setOpts([methods, statuses, sources, dests]);
  }, [props.items.length]);
  
  const columes = [
    { field: 'req', body: (data) => <span>{__source(data)}</span>, header: 'source' },
    { field: 'res', body: (data) => <span>{__dest(data)}</span>, header: 'dest' },
    { field: 'method', header: 'method' },
    { field: 'status', header: 'status' },
    { field: 'ttr', header: 'ttr(micro sec)', sortable: true },
    { field: 'req.head', header: 'path',style: { width: '40vw' } },
  ];

  const fetchItems = (): any[] => {
    let filters = [
      createFilter(f_methods, __method),
      createFilter(f_status, __status),
      createFilter(f_source, __source),
      createFilter(f_dest, __dest),
    ];
    return filter(props.items, (item: IHttp) => {
      return filters.reduce<boolean>((prev: boolean, cur: (item: IHttp) => boolean) => (prev && cur(item)), true);
    });
  }

  const items = fetchItems();

  const result = {
    items,
    page: 1,
    size: items.length,
    total: items.length
  };
  const header = <div className="card flex flex-nowrap gap-3 p-fluid">
    <Mult label="Method: " _options={opts[0]} select={setMethods} ></Mult>
    <Mult label="Status: " _options={opts[1]} select={setStatus} ></Mult>
    <Mult label="From: " _options={opts[2]} select={setSource} ></Mult>
    <Mult label="To:" _options={opts[3]} select={setDest} ></Mult>
  </div>;
  const _props = {
    header,
    scrollHeight: 80,
    cols: columes,
    getStyle: (item) => {
      return `status-${item.status}`
    },
    result
  };
  return (<div className="flex flex-column h-full w-full" id="http-page">
    <SubComponnet {..._props}/>
  </div>
  );
};

export default HttpComponnet;
