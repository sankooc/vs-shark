import React, { ReactElement, useState } from "react";
import { DataTable, DataTableSelectionSingleChangeEvent } from 'primereact/datatable';
import { Column, ColumnProps } from 'primereact/column';
import { ColumnItem } from "../common";


class Props {
  cols: ColumnProps[] = [];
  items: ColumnItem[] = [];
  onSelect: (item: ColumnItem) => void;
  scrollHeight?: string;
  size?: 'small' | 'normal' | 'large';
  className?: string;
}
const DTable = (props: Props) => {
  const [select, setSelect] = useState<number>(0);
  const onSelectionChange = (e: DataTableSelectionSingleChangeEvent<ColumnItem[]>) => {
    const { no } = e.value;
    setSelect(no);
    props.onSelect(e.value);
  }
  return (<>
    <DataTable value={props.items} selectionMode="single" rowClassName={(data: ColumnItem) => {
      if (select === data.no) {
        return 'active';
      }
      return data.style;
    }} onSelectionChange={onSelectionChange}
      virtualScrollerOptions={{ itemSize: 20 }} scrollable showGridlines
      style={{ tableLayout: 'fixed' }}
      scrollHeight={props.scrollHeight || "67vh"} size={props.size || "small"} className={"pcap-table flex-grow-1 w-full"}
      currentPageReportTemplate="{first} to {last} of {totalRecords}">
      {props.cols.map((c: ColumnProps, inx: number): ReactElement => {
        return (<Column {...c} key={'col' + inx}></Column>)
      })}
    </DataTable>
  </>
  );
};


export default DTable;