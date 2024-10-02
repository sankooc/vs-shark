import React, { useState } from "react";
import { IHttp } from "../../common";
import DTable, { Props } from '../dataTable';
import Viewer from './viewer';

const SubComponnet = (props: Props) => {
  const [selection, setSelect] = useState<IHttp>(null);
  const onSelect = setSelect;
  const scrollHeight = 70;
  return (<>
    <DTable {...props} onSelect={onSelect} scrollHeight={scrollHeight}></DTable>
    <Viewer key={Date.now()} item = {selection}/>
  </>
  );
};

export default SubComponnet;
