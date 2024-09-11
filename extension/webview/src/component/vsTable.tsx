import React, { ReactElement, useState } from "react";
import { ColumnItem } from "../common";
import {
  VSCodeDataGrid,
  VSCodeDataGridCell,
  VSCodeDataGridRow,
} from "@vscode/webview-ui-toolkit/react";


class Props {
  cols: any[] = [];
  items: any[] = [];
  onSelect: (item: ColumnItem) => void;
  scrollHeight?: string;
  size?: 'small' | 'normal' | 'large';
  className?: string;
}
const DTable = (props: Props) => {
  const headers = props.cols.map((col, inx): ReactElement => {
    const no = inx +1;
    return (<VSCodeDataGridCell cell-type="columnheader" grid-column={no} key={"head"+no}>
      {col.header}
  </VSCodeDataGridCell>);
  });
  const _colRender = (item, col, inx: number): ReactElement => {
    const no = inx + 1;
    const _text = item[col.field] || '';
    return <VSCodeDataGridCell grid-column={no} key={'item'+no}>{_text}</VSCodeDataGridCell>
  }
  const colReader = (item, inx): ReactElement => {
    return (<VSCodeDataGridRow key={'row'+inx}>
      {props.cols.map((f, inx) => {return _colRender(item, f, inx);})}
    </VSCodeDataGridRow>)
  }
  return (<>
    <VSCodeDataGrid aria-label="Default">
      <VSCodeDataGridRow row-type="header">
        {headers}
      </VSCodeDataGridRow>
      {props.items.map(colReader)}
    </VSCodeDataGrid>
  </>
  );
};


export default DTable;