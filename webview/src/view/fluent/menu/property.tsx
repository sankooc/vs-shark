
import {
    Dialog,
    DialogSurface,
    DialogTitle,
    DialogContent,
    DialogActions,
    DialogTrigger,
    DialogBody,
    Button,
    Divider
} from "@fluentui/react-components";
import {
    TableBody,
    TableCell,
    TableRow,
    Table,
    TableHeader,
    TableHeaderCell,
    TableCellLayout,
} from "@fluentui/react-components";

import { useEffect, useState } from "react";
import { usePcapStore } from "../../context";
import { FileMetadata } from "../../../share/common";


interface InProps {
    items: { name: string, description: string, filter: string, protocol: string, os: string }[]
}
const Interfaces = (props: InProps) => {
    const items = props.items;

    const columns = [
        { columnKey: "name", label: "Interface" },
        { columnKey: "author", label: "Filter" },
        { columnKey: "protocol", label: "Protocol" },
        { columnKey: "os", label: "OS" },
    ];
    return <Table
        size="extra-small"
        style={{ minWidth: "100%" }}
    >
        <TableHeader>
            <TableRow>
                {columns.map((column) => (
                    <TableHeaderCell key={column.columnKey}>
                        {column.label}
                    </TableHeaderCell>
                ))}
            </TableRow>
        </TableHeader>
        <TableBody>
            {items.map((item) => (
                <TableRow key={item.name}>
                    <TableCell>
                        <TableCellLayout className="cell-data">
                            {item.name || 'N/A'}
                        </TableCellLayout>
                    </TableCell>
                    <TableCell>
                        <TableCellLayout>
                            {item.filter || 'N/A'}
                        </TableCellLayout>
                    </TableCell>
                    <TableCell>
                        <TableCellLayout>
                            {item.protocol}
                        </TableCellLayout>
                    </TableCell>
                    <TableCell>
                        <TableCellLayout className="cell-data">
                            {item.os || 'N/A'}
                        </TableCellLayout>
                    </TableCell>
                </TableRow>
            ))}
        </TableBody>
    </Table>
}


interface Props {
    open: boolean,
    setOpen: (state: boolean) => void
}

export default function Component(props: Props) {
    const [state, setState] = useState<FileMetadata | null>(null);
    const metadata = usePcapStore(state => state.metadata)
    useEffect(() => {
        if (props.open) {
            metadata().then(setState);
        }
    }, [props.open])
    if (!state) {
        return <></>
    }

    const buildBlock = (title: string, kv: [string, string][]) => {
        return <>
            {title ? <div className="divider">
                <Divider appearance="strong">{title}</Divider>
            </div> : null}
            {
                kv.map((v, index) => {
                    return <div className="property" key={`${index} + ${v[0]}`}>
                        <span className="label">{v[0]}</span>
                        <span className="value">{v[1]}</span>
                    </div>
                })
            }
        </>
    }
    const head1: [string, string][] = [];
    head1.push(['version', `${state.major}.${state.minor}`]);
    head1.push(['format', state.file_type.toLowerCase()]);
    if (state.start) {
        head1.push(['start', state.start]);
        head1.push(['end', state.end]);
        head1.push(['elapsed', state.elapsed]);
    }
    const capture: [string, string][] = [];
    if (state.capture) {
        capture.push(['hardware', state.capture.hardware]);
        capture.push(['os', state.capture.os]);
        capture.push(['application', state.capture.application]);
    }

    return <Dialog
        open={props.open}
        onOpenChange={(_event, data) => {
            props.setOpen(data.open);
        }}
    >
        <DialogSurface>
            <DialogBody>
                <DialogTitle>Details</DialogTitle>
                <DialogContent className="about-property">
                    {buildBlock('', head1)}
                    {capture.length ? buildBlock('Capture', capture) : null}
                    <div className="divider">
                        <Divider appearance="strong">Interfaces</Divider>
                    </div>
                    <Interfaces items={state.interfaces} />
                </DialogContent>
                <DialogActions>
                    <DialogTrigger disableButtonEnhancement>
                        <Button appearance="secondary" size="small">Close</Button>
                    </DialogTrigger>
                </DialogActions>
            </DialogBody>
        </DialogSurface>
    </Dialog>
}
