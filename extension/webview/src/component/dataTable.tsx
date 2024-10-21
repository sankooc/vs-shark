import React, { ReactElement, useState } from "react";
import { DataTable, DataTableSelectionSingleChangeEvent } from 'primereact/datatable';
import { Paginator } from 'primereact/paginator';
import { Column, ColumnProps } from 'primereact/column';
import { IconField } from 'primereact/iconfield';
import { MultiSelect } from 'primereact/multiselect';
import { IResult } from "../common";


export class Props {
  cols: ColumnProps[] = [];
  result?: IResult;
  header?: ReactElement;
  footer?: ReactElement;
  onSelect?: (item: any) => void;
  getStyle?: (item: any) => string;
  scrollHeight?: number;
  size?: 'small' | 'normal' | 'large';
  className?: string;
  multi?: boolean;
  request?: (event:any) => void;
  filter?: any;
}

const DTable = (props: Props) => {
  const [select, setSelect] = useState<number>(0);
  const [selects, setSelects] = useState<number[]>([1]);
  const [loading, setLoading] = useState<boolean>(false);
  const onSelectionChange = (e: DataTableSelectionSingleChangeEvent<any[]>) => {
    const { index } = e.value;
    
    if(index !== undefined) {
      setSelect(index);
      if(props.onSelect) {
        props.onSelect(e.value);
      }
    }
  }
  const scrollHeight = props.scrollHeight || 99;
  const result = props.result || { total: 0, size : 0, items: [], page: 1};
  const { total, size, items, page } = result;
  const hasPaging = total > size;
  let tableHeight = `${scrollHeight}vh`;
  let inSight = `calc(${scrollHeight}vh - 1px)`;
  const space = 55;
  tableHeight = `calc(${scrollHeight}vh - ${space}px)`
  const rowClassName = (data: any) => {
    if (data.index !== undefined && data.index == select){
      return 'active';
    }
    if(props.getStyle) {
      return props.getStyle(data);
    }
    return ''
  };
  const first = (page - 1) * size;
  const opt = {
    optionLabel: "name",
    placeholder: "Select Protocols",
    maxSelectedLabels: 10,
    ...props.filter
  };

  const tableProps = {
    loading,
    style: {height: inSight},
    value: items,
    showHeaders: true,
    rowClassName,
    virtualScrollerOptions: { itemSize: 20 },
    scrollHeight: tableHeight,
    onSelectionChange,
    showGridlines: true,
    scrollable: true,
    // size: 'small',
    className: 'pcap-table w-full',
  };
  if (props.className) {
    tableProps.className = props.className;
  }
  return (<>
    <DataTable {...tableProps}
      size="small"
      selectionMode="single"
      header={props.header}
      footer={props.footer}
      >
      {props.cols.map((c: ColumnProps, inx: number): ReactElement => {
        return (<Column {...c} key={'col' + inx}></Column>)
      })}
    </DataTable>
  </>
  );
};


export default DTable;