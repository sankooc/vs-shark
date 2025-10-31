import { DataGrid, DataGridBody, DataGridCell, DataGridHeader, DataGridHeaderCell, DataGridRow, Slot, TableColumnDefinition, TableColumnSizingOptions, useFluent, useScrollbarWidth } from "@fluentui/react-components";
import { IListResult } from "../../share/gen";
import Pagination from './pagination2';

import { useEffect, useState } from "react";
import { BreadItem } from "./common";

interface GridProps<T> {
    columns: TableColumnDefinition<T>[];
    onClick?: (item: T) => void;
    load: (page: number) => Promise<IListResult<T>>;
    pageSize: number;
    columnSizingOptions?: TableColumnSizingOptions,
    breads?: { icon?: Slot<'span'>, name: string, path?: string}[],
    size: string 
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
            props.load(page).then(setResult)
        }
    };
    useEffect(mountHook, [page]);
    const columnSizingOptions = { ...props.columnSizingOptions };

    return <div className="flex flex-column">
        {
            props.breads && props.breads.length > 0 && <BreadItem items={props.breads} ></BreadItem>
        }
        <DataGrid items={result.items}
            size="extra-small"
            resizableColumns
            columnSizingOptions={columnSizingOptions}
            columns={props.columns} style={{ minWidth: "auto", overflow: 'hidden auto' }} className="h-full" >
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
    </div>
}

export default Component;