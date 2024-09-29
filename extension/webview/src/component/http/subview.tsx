import React, { useState } from "react";
import { IHttp } from "../../common";
import DTable, { Props } from '../dataTable';
import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';

const SubComponnet = (props: Props) => {

  const [selection, setSelect] = useState<IHttp>(null);
  const [visible, setVisible] = useState<boolean>(false);

  const disabled = !selection;
  const onSelect = setSelect;
  const footer = <div className="card flex flex-nowrap gap-3 p-fluid">
    <div className="flex align-items-right gap-3">
      <Button disabled={disabled} onClick={() => {setVisible(true)}} label="Detail" icon="pi pi-search" size="small" />
    </div>
  </div>
  return (<>
    <Dialog header="Header" visible={visible} style={{ width: '70vw' }} onHide={() => {if (!visible) return; setVisible(false);}}>
    <p className="m-0">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
        Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
        consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. 
        Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
    </p>
    </Dialog>
    <DTable {...props} onSelect={onSelect} footer={footer}></DTable>
  </>
  );
};

export default SubComponnet;
