import { ReactElement } from "react";
import { DataTable, DataTableProps, DataTableRowToggleEvent, DataTableSelectionSingleChangeEvent, DataTableStateEvent } from 'primereact/datatable';
import { Column, ColumnProps } from 'primereact/column';
import { IResult } from "../../share/common";


export class Props<T> {
  cols: ColumnProps[] = [];
  result!: IResult<T>;
  header?: ReactElement;
  onSelect?: (item: T) => void;
  getStyle?: (item: T) => string;
  scrollHeight?: number;
  size?: 'small' | 'normal' | 'large';
  className?: string;
  multi?: boolean;
  request?: (event: any) => void;
  filter?: any;

  rowExpansionTemplate?: (data: T) => ReactElement;
  expandedRows?: T[];
  showGridlines: boolean = false;
  onRowToggle?: (e: DataTableRowToggleEvent) => void;
  onPage!: (e: DataTableStateEvent) => void;
  loading: boolean = false;
}

const DTable = (props: Props<any>) => {
  // const [select, setSelect] = useState<number>(0);

  const onSelectionChange = (e: DataTableSelectionSingleChangeEvent<any[]>) => {
    const { key } = e.value;
    // console.log(e)

    if (key !== undefined) {
      // setSelect(key);
      if (props.onSelect) {
        props.onSelect(e.value);
      }
    }
  }

  const { total, size, items, page } = props.result;
  const tableProps: DataTableProps<any> = {
    onPage: props.onPage,
    header: props.header,
    loading: props.loading,
    showGridlines: props.showGridlines,
    value: items,
    showHeaders: true,
    onSelectionChange,
    scrollable: true,
    scrollHeight: 'flex',
    className: 'pcap-table w-full',
  };
  if (props.className) {
    tableProps.className = props.className;
  }
  if (props.onRowToggle) {
    tableProps.onRowToggle = props.onRowToggle;
    tableProps.expandedRows = props.expandedRows;
    tableProps.rowExpansionTemplate = props.rowExpansionTemplate;
  }
  return (<>
    <DataTable {...tableProps}
      size="small"
      selectionMode="single"
      lazy 
      paginator
      rows={size}
      first={page} 
      totalRecords={total}
    >
      {props.cols.map((c: ColumnProps, inx: number): ReactElement => {
        return (<Column {...c} key={'col' + inx}></Column>)
      })}
    </DataTable>
  </>
  );
};


export default DTable;