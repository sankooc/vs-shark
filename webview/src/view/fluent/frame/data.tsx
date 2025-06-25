import {
    TableColumnDefinition,
    createTableColumn,
    TableCellLayout,
    useScrollbarWidth,
    useFluent,
    makeStyles,
    OnSelectionChangeData,
} from '@fluentui/react-components';
import {
    DataGridBody,
    DataGrid,
    DataGridRow,
    DataGridHeader,
    DataGridCell,
    DataGridHeaderCell,
    RowRenderer,
} from '@fluentui-contrib/react-data-grid-react-window';
import { IFrameInfo } from '../../../share/gen';
import indexCss from './index.module.scss';
import React from 'react';

const columns: TableColumnDefinition<IFrameInfo>[] = [
    createTableColumn<IFrameInfo>({
        columnId: 'index',
        renderHeaderCell: () => {
            return 'Index';
        },
        renderCell: (item: IFrameInfo) => {
            let cn = indexCss.headcell + ' ' + indexCss.cell;
            const protocol = item.protocol;
            if (indexCss[protocol]) {
                cn += ' ' + indexCss[protocol];
            }
            return (
                <TableCellLayout className={cn}>
                    {item.index + 1}
                </TableCellLayout>
            );
        },
    }),
    createTableColumn<IFrameInfo>({
        columnId: 'source',
        renderHeaderCell: () => {
            return 'Source';
        },
        renderCell: (item: IFrameInfo) => {
            return (
                <TableCellLayout className={indexCss.cell}>
                    {item.source}
                </TableCellLayout>
            );
        },
    }),
    createTableColumn<IFrameInfo>({
        columnId: 'target',
        renderHeaderCell: () => {
            return 'Target';
        },
        renderCell: (item: IFrameInfo) => {
            return (
                <TableCellLayout className={indexCss.cell}>
                    {item.dest}
                </TableCellLayout>
            );
        },
    }),
    createTableColumn<IFrameInfo>({
        columnId: 'protocol',
        renderHeaderCell: () => {
            return 'Protocol';
        },
        renderCell: (item: IFrameInfo) => {
            return (
                <TableCellLayout className={indexCss.cell} >
                    {item.protocol}
                </TableCellLayout>
            );
        },
    }),
    createTableColumn<IFrameInfo>({
        columnId: 'length',
        renderHeaderCell: () => {
            return 'Size';
        },
        renderCell: (item: IFrameInfo) => {
            return (
                <TableCellLayout className={indexCss.cell}>
                    {item.len}
                </TableCellLayout>
            );
        },
    }),
    createTableColumn<IFrameInfo>({
        columnId: 'info',
        renderHeaderCell: () => {
            return 'Info';
        },
        renderCell: (item: IFrameInfo) => {
            return (
                <TableCellLayout className={indexCss.cell} >
                    {item.info}
                </TableCellLayout>
            );
        },
    }),
];

const renderRow: RowRenderer<any> = ({ item, rowId }, style) => (
    <DataGridRow<any> key={rowId} style={{...style}}>
        {({ renderCell }) => (
            <DataGridCell focusMode="group">{renderCell(item)}</DataGridCell>
        )}
    </DataGridRow>
);

interface Props<T> {
    bodyHeight: number;
    items: T[];
    onSelect: (item: T) => void;
}
const columnSizingOptions = {
    index: {
        idealWidth: 50,
        minWidth: 50,
        defaultWidth: 50,
    },
};
const useStyles = makeStyles({
    hideSelectionColumn: {
        '& [role="row"] > [role="gridcell"]:first-child': {
            display: 'none'
        }
    }
});

export const VirtualizedDataGrid = (props: Props<any>) => {
    const { targetDocument } = useFluent();
    const [select, setSelect] = React.useState<number | undefined>(undefined);
    const scrollbarWidth = useScrollbarWidth({ targetDocument });
    const styles = useStyles();
    const onSelectionChange = (_event: any, data: OnSelectionChangeData) => {
        if (data.selectedItems.size > 0) {
            const selected = data.selectedItems.values().next().value as number;
            if (selected != select) {
                setSelect(selected);
                const item = props.items[selected];
                props.onSelect(item);
            }
        }
    }
    return (
        <DataGrid
            size="small"
            items={props.items}
            columns={columns}
            columnSizingOptions={columnSizingOptions}
            selectionMode="single"
            resizableColumns
            className={styles.hideSelectionColumn}
            onSelectionChange={onSelectionChange}
        >
            <DataGridHeader style={{ paddingRight: scrollbarWidth }}>
                <DataGridRow>
                    {({ renderHeaderCell }) => (
                        <DataGridHeaderCell>{renderHeaderCell()}</DataGridHeaderCell>
                    )}
                </DataGridRow>
            </DataGridHeader>
            <DataGridBody<any> itemSize={30} height={props.bodyHeight} style={{overflowX: 'hidden'}}>
                {renderRow}
            </DataGridBody>
        </DataGrid>
    );
};