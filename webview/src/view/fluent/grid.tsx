import { DataGrid, DataGridBody, DataGridCell, DataGridHeader, DataGridHeaderCell, DataGridRow, TableColumnDefinition, TableColumnSizingOptions, useFluent, useScrollbarWidth } from "@fluentui/react-components";
import { IListResult } from "../../share/gen";
import Pagination from './pagination';
import { useEffect, useState } from "react";

interface GridProps<T> {
    columns: TableColumnDefinition<T>[];
    onClick: (item: T) => void;
    load: (page: number) => Promise<IListResult<T>>;
    pageSize: number;
    columnSizingOptions?: TableColumnSizingOptions,
}

function Component<T>(props: GridProps<T>) {
    const { targetDocument } = useFluent();
    const scrollbarWidth = useScrollbarWidth({ targetDocument });
    const [page, setPage] = useState<number>(1);
    // const [loading, setLoading] = useState<boolean>(false);
    const [result, setResult] = useState<IListResult<T>>({
        start: 0,
        total: 0,
        items: [],
    });
    const mountHook = () => {
        // setLoading(true);
        if (page >= 1) {
            props.load(page).then((rs: IListResult<T>) => {
                setResult(rs);
                // setLoading(false);
            })
        }
    };
    useEffect(mountHook, [page]);
    const columnSizingOptions = { ...props.columnSizingOptions };
    return <div className="h-full flex flex-column">
        <DataGrid items={result.items}
            size="small"
            resizableColumns
            columnSizingOptions={columnSizingOptions}
            columns={props.columns} style={{ overflowY: 'auto' }} className="h-full" >
            <DataGridHeader style={{ paddingRight: scrollbarWidth }}>
                <DataGridRow>
                    {({ renderHeaderCell }) => (
                        <DataGridHeaderCell>{renderHeaderCell()}</DataGridHeaderCell>
                    )}
                </DataGridRow>
            </DataGridHeader>
            <DataGridBody<T>>
                {({ item, rowId }) => (
                    <DataGridRow<any> key={rowId} onClick={() => {
                        props.onClick(item);
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