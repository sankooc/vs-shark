import { Button, Card, DataGrid, DataGridBody, DataGridCell, DataGridHeader, DataGridHeaderCell, DataGridRow, Slot, TableColumnDefinition, TableColumnSizingOptions, useFluent, useScrollbarWidth} from "@fluentui/react-components";

// import { UseTableFeaturesOptions } from "@fluentui/react-table";

import { IListResult } from "../../share/gen";
import Pagination from './pagination2';

import { JSX, ReactNode, useEffect, useState } from "react";
import { BreadItem } from "./common";
import Empty from "./http/content/empty";
import React from "react";

// import {
//   useTableFeatures,
//   UseTableFeaturesOptions,
//   TableState,
//   TableSortState,
// } from "@fluentui/react-table";

type SortDirection = "ascending" | "descending";

export interface SortState {
  sortColumn?: string;
  sortDirection?: SortDirection;
}
interface GridProps<T> {
    columns: TableColumnDefinition<T>[];
    header?: JSX.Element;
    filterComponent?: JSX.Element;
    onClick?: (item: T) => void;
    load: (page: number, filter: any) => Promise<IListResult<T>>;
    pageSize: number;
    columnSizingOptions?: TableColumnSizingOptions,
    breads?: { icon?: Slot<'span'>, name: string, path?: string }[],
    // size?: "small" | "medium" | "extra-small",
    size?: string,
    sortState?: SortState,
    onSortChange?:  (e: Event, sortState: SortState) => void;
}
interface PageProps {
    children: React.ReactElement<ReactNode>;
    breads?: { icon?: Slot<'span'>, name: string, path?: string }[],
}
export function PageFrame(props: PageProps) {
    return (<>
        {
            props.breads && props.breads.length > 0 && <BreadItem items={props.breads} ></BreadItem>
        }
        <Card className="flex flex-1 justify-content-between align-items-stretch page-card-item" style={{ margin: '0', padding: '5px', overflow: 'auto' }} orientation="vertical">
            {props.children}
        </Card>
    </>)
}

function Component<T>(props: GridProps<T>) {
    const { targetDocument } = useFluent();
    const scrollbarWidth = useScrollbarWidth({ targetDocument });
    const [page, setPage] = useState<number>(1);
    const [result, setResult] = useState<IListResult<T>>({
        start: 0,
        total: 0,
        items: [],
    });
    const mountHook = () => {
        if (page >= 1) {
            props.load(page, null).then(setResult)
        }
    };
    const doSearch = () => {
        setPage(1);
        props.load(1, null).then(setResult)

    }
    let _state = page + '';
    if(props.sortState){
        _state = _state + ' ' + props.sortState.sortDirection;
    }
    useEffect(mountHook, [_state]);
    const columnSizingOptions = { ...props.columnSizingOptions };

    let main = <Empty/>;
    if(result.items && result.items.length > 0){

        const _props: any = {
            resizableColumns: true,
            columnSizingOptions,
            items: result.items,
            columns: props.columns,
            sortable: false,
            sortState: undefined,
        }
        if(props.sortState){
            _props.sortable = true;
            _props.sortState = props.sortState;
            _props.onSortChange = props.onSortChange;
        }
        main = <>
        <DataGrid
            size='small'
            style={{ minWidth: "auto", overflow: 'hidden auto' }}
            // resizableColumnsOptions={{autoFitColumns: true}}
            // onSortChange={}
            {..._props}
            className="h-full w-full" >
            <DataGridHeader style={{ paddingRight: scrollbarWidth, backgroundColor: '#458588' }}>
                <DataGridRow>
                    {({ renderHeaderCell }) => (
                        <DataGridHeaderCell>{renderHeaderCell()}</DataGridHeaderCell>
                    )}
                </DataGridRow>
            </DataGridHeader>
            <DataGridBody<T>>
                {({ item, rowId }) => (
                    <DataGridRow key={rowId} onClick={() => {
                        // eslint-disable-next-line @typescript-eslint/no-unused-expressions
                        props.onClick && props.onClick(item);
                    }} >
                        {({ renderCell }) => (
                            <DataGridCell>{renderCell(item)}</DataGridCell>
                        )}
                    </DataGridRow>
                )}
            </DataGridBody>
        </DataGrid>
        <Pagination page={page} total={result.total} pageSize={props.pageSize} onPageChange={setPage} />
    </>;
    }

    return (<>
        {
            props.breads && props.breads.length > 0 && <BreadItem items={props.breads} ></BreadItem>
        }
        {
            props.header
        }
        {
            props.filterComponent && (<Card className="w-full flex justify-content-between align-items-center page-card-item" style={{margin: '0 0 5px 0'}} orientation="horizontal">
                <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                    {props.filterComponent}
                </div>
                <Button style={{ marginLeft: 'auto' }} size="small" onClick={doSearch}>Search</Button>
            </Card>)
        }
        <Card className="flex flex-grow-1 justify-content-between align-items-center page-card-item" style={{ margin: '0px', padding: 0 }} orientation="vertical">
            {main}
        </Card>
    </>)
}

export default Component;