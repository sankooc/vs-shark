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
import { frameColor } from '../../colors';

const columns: TableColumnDefinition<IFrameInfo>[] = [
    createTableColumn<IFrameInfo>({
        columnId: 'index',
        renderHeaderCell: () => 'Index',
        renderCell: (item: IFrameInfo) => {
            // let cn = indexCss.cell;
            // const protocol = item.protocol;
            // if (indexCss[protocol]) {
            //     cn += ' ' + indexCss[protocol];
            // }
            return (
                <TableCellLayout className={indexCss.cell}>
                    {item.index + 1}
                </TableCellLayout>
            );
        },
    }),
    createTableColumn<IFrameInfo>({
        columnId: 'source',
        renderHeaderCell: () => 'Source',
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
        renderHeaderCell: () => <>Target</>,
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
        renderHeaderCell: () => 'Protocol',
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
        renderHeaderCell: () => 'Size',
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
        renderHeaderCell: () => 'Info',
        renderCell: (item: IFrameInfo) => {
            return (
                <TableCellLayout className={indexCss.cell} >
                    {item.info}
                </TableCellLayout>
            );
        },
    }),
];

interface Props<T> {
    bodyHeight: number;
    items: T[];
    // onSelect: (item: T) => void;
    select: number[];
    onSelect: (index: number[]) => void;
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
    },
    ...frameColor
});

export const VirtualizedDataGrid = (props: Props<any>) => {
    const { targetDocument } = useFluent();
    const scrollbarWidth = useScrollbarWidth({ targetDocument });
    const styles = useStyles();
    const renderRow: RowRenderer<any> = ({ item, rowId }, style) => {
        let claz = indexCss.cellfont;
        // eslint-disable-next-line no-prototype-builtins
        if (styles.hasOwnProperty(item.protocol)) {
            claz += ' ' +(styles as any)[item.protocol];
        }
        // if (select !== undefined && select >= 0) {
        //     const selectedItem = props.items[select];
        //     if (claz && selectedItem && selectedItem.index === item.index) {
        //         claz += claz + ' acdc'
        //     }
        // }
        return <DataGridRow key={rowId} style={{ ...style }} className={claz}>
            {({ renderCell }) => (
                <DataGridCell focusMode="group">{renderCell(item)}</DataGridCell>
            )}
        </DataGridRow>;
    };
    const onSelectionChange = (_event: any, data: OnSelectionChangeData) => {
        if (data.selectedItems.size > 0) {
            const selected = data.selectedItems.values().next().value as number;
            props.onSelect([selected]);
        }
    }
    return (
        <DataGrid
            size="small"
            items={props.items}
            columns={columns}
            columnSizingOptions={columnSizingOptions}
            selectionMode="single"
            selectedItems={props.select}
            selectionAppearance="none"
            resizableColumns
            // focusMode="row_unstable"
            onSelectionChange={onSelectionChange}
        >
            <DataGridHeader style={{ paddingRight: scrollbarWidth }}>
                <DataGridRow>
                    {({ renderHeaderCell }) => (
                        <DataGridHeaderCell>{renderHeaderCell()}</DataGridHeaderCell>
                    )}
                </DataGridRow>
            </DataGridHeader>
            <DataGridBody<any> itemSize={30} height={props.bodyHeight} style={{ overflowX: 'hidden' }}>
                {renderRow}
            </DataGridBody>
        </DataGrid>
    );
};