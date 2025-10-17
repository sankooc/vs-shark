import { Button, Card, DataGrid, DataGridBody, DataGridCell, DataGridHeader, DataGridHeaderCell, DataGridRow, Slot, TableColumnDefinition, TableColumnSizingOptions, useFluent, useScrollbarWidth } from "@fluentui/react-components";
import { IListResult } from "../../share/gen";
import Pagination from './pagination2';

import { JSX, ReactNode, useEffect, useState } from "react";
import { BreadItem } from "./common";
interface GridProps<T> {
    columns: TableColumnDefinition<T>[];
    filterComponent?: JSX.Element;
    onClick?: (item: T) => void;
    load: (page: number, filter: any) => Promise<IListResult<T>>;
    pageSize: number;
    columnSizingOptions?: TableColumnSizingOptions,
    breads?: { icon?: Slot<'span'>, name: string, path?: string }[],
    size?: "small" | "medium" | "extra-small",
}
interface PageProps {
    children: React.ReactElement<ReactNode>;
    breads?: { icon?: Slot<'span'>, name: string, path?: string }[],
}
export function PageFrame(props: PageProps) {
    return (<div className="flex flex-column page-card">
        {
            props.breads && props.breads.length > 0 && <BreadItem items={props.breads} ></BreadItem>
        }
        <Card className="flex flex-grow-1" style={{ margin: '0', padding: '5px', alignItems: 'stretch', justifyContent: 'space-between', overflow: 'auto' }} orientation="vertical">
            {props.children}
        </Card>
    </div>)
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
    useEffect(mountHook, [page]);
    const columnSizingOptions = { ...props.columnSizingOptions };

    return (<div className="flex flex-column page-card">
        {
            props.breads && props.breads.length > 0 && <BreadItem items={props.breads} ></BreadItem>
        }
        {
            props.filterComponent && (<Card className="w-full" style={{margin: '0 0 5px 0', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }} orientation="horizontal">
                <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                    {props.filterComponent}
                </div>
                <Button style={{ marginLeft: 'auto' }} size="small" onClick={doSearch}>Search</Button>
            </Card>)
        }
        <Card className="flex flex-grow-1" style={{ margin: '0px', padding: 0, alignItems: 'center', justifyContent: 'space-between' }} orientation="vertical">
            <DataGrid items={result.items}
                size={props.size}
                resizableColumns
                columnSizingOptions={columnSizingOptions}
                columns={props.columns} style={{ minWidth: "auto", overflow: 'hidden auto' }} className="h-full w-full" >
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
        </Card>
    </div>)
}

export default Component;