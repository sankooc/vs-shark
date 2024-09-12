import React, { ReactElement, useState } from "react";
import { DataTable, DataTableSelectionSingleChangeEvent } from 'primereact/datatable';
import { Paginator } from 'primereact/paginator';
import { Column, ColumnProps } from 'primereact/column';
import { IconField } from 'primereact/iconfield';
import { MultiSelect } from 'primereact/multiselect';
import { IResult } from "../common";


class Props {
  cols: ColumnProps[] = [];
  result?: IResult;
  onSelect?: (item: any) => void;
  getStyle?: (item: any) => string;
  scrollHeight?: number;
  size?: 'small' | 'normal' | 'large';
  className?: string;
  request?: (event:any) => void;
  filter?: any;
}
const DTable = (props: Props) => {
  const [select, setSelect] = useState<number>(0);
  const [loading, setLoading] = useState<boolean>(false);
  const onSelectionChange = (e: DataTableSelectionSingleChangeEvent<any[]>) => {
    const { index } = e.value;
    setSelect(index);
    if(props.onSelect) {
      props.onSelect(e.value);
    }
  }
  const scrollHeight = props.scrollHeight || 99;
  const result = props.result || { total: 0, size : 0, items: [], page: 1};
  const { total, size, items, page } = result;
  const hasPaging = total > size;
  let tableHeight = `${scrollHeight}vh`;
  let inSight = `calc(${scrollHeight}vh - 1px)`;
  const space = 40;
  const hasFootbar = hasPaging || !!props.filter;
  if(hasFootbar){
    tableHeight = `calc(${scrollHeight}vh - ${space}px)`
    inSight = `calc(${scrollHeight}vh - ${space + 1}px)`;
  }
  const onPageChange = (event) => {
    if(props.request) {
      setLoading(true);
      props.request(event);
    }
  }
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
  return (<>
    <DataTable loading={loading} style={{height: inSight, overflow: "auto"}} value={items} showHeaders selectionMode="single" rowClassName={rowClassName}
    virtualScrollerOptions={{ itemSize: 20 }}
      scrollHeight={inSight} 
      onSelectionChange={onSelectionChange} showGridlines scrollable
      size={"small"} className={"pcap-table flex-grow-1 w-full"}>
      {props.cols.map((c: ColumnProps, inx: number): ReactElement => {
        return (<Column {...c} key={'col' + inx}></Column>)
      })}
    </DataTable>
    {hasFootbar && <div className="flex justify-content-between" style={{height: `${space}px`}}>
      {!!props.filter && <IconField className="filter w-3 flex">
          <MultiSelect {...opt} className="p-inputtext-sm" />
        </IconField>}
        {hasPaging && <Paginator pageLinkSize={16} first={first} onPageChange={onPageChange}  className="paging" rows={size} totalRecords={total} />}
      </div>}
  </>
  );
};


export default DTable;