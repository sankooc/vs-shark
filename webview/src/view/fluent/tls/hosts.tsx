import { useStore } from "../../store";
import { createTableColumn, TableCellLayout, TableColumnDefinition } from "@fluentui/react-components";
import { compute, ComRequest, ITLSConnect } from "../../../share/common";
import Grid from "../table";

import { TLSIcon } from "../common";
function Component() {
  const tlsList = useStore((state) => state.tlsList);
  const columns: TableColumnDefinition<ITLSConnect>[] = [
    createTableColumn<ITLSConnect>({
      columnId: "index",
      renderHeaderCell: () => 'Index',
      renderCell: (item) => <TableCellLayout> {item.index} </TableCellLayout>,
    }),
    createTableColumn<ITLSConnect>({
      columnId: "version",
      renderHeaderCell: () => 'Ver',
      renderCell: (item) => {
        let str = '';
        if (item.list && item.list.length) {
          for (const it of item.list) {
            if (it.version) {
              str = it.version;
              break;
            }
          }
        }
        return <TableCellLayout>{str}</TableCellLayout>
      },
    }),
    createTableColumn<ITLSConnect>({
      columnId: "cs",
      renderHeaderCell: () => 'CipherSuite',
      renderCell: (item) => {
        let str = '';
        if (item.list && item.list.length) {
          for (const it of item.list) {
            if (it.cipher_suite) {
              str = it.cipher_suite;
              break;
            }
          }
        }
        return <TableCellLayout>{str}</TableCellLayout>
      },
    }),
    createTableColumn<ITLSConnect>({
      columnId: "primary",
      renderHeaderCell: () => 'Address A',
      renderCell: (item) => <TableCellLayout> {item.primary} </TableCellLayout>,
    }),
    createTableColumn<ITLSConnect>({
      columnId: "second",
      renderHeaderCell: () => 'Address B',
      renderCell: (item) => <TableCellLayout> {item.second} </TableCellLayout>,
    }),
    createTableColumn<ITLSConnect>({
      columnId: "count",
      renderHeaderCell: () => 'Count',
      renderCell: (item) => <TableCellLayout>{item.list.length}</TableCellLayout>,
    }),
    createTableColumn<ITLSConnect>({
      columnId: "alpn",
      renderHeaderCell: () => 'ALPN',
      renderCell: (item) => {
        let str = 'N/A';
        if (item.list && item.list.length) {
          const set = new Set();
          for (const it of item.list) {
            if (it && it.alpn) {
              for (const p of it.alpn) {
                set.add(p)

              }
            }
          }
          str = [...set].join(', ')
        }
        return <TableCellLayout>{str}</TableCellLayout>
      },
    }),
    createTableColumn<ITLSConnect>({
      columnId: "sni",
      renderHeaderCell: () => 'SNI',
      renderCell: (item) => {
        let str = 'N/A';
        if (item.list && item.list.length) {
          const set = new Set();
          for (const it of item.list) {
            if (it.hostname) {
              set.add(it.hostname);
            }
          }
          str = [...set].join(', ')
        }
        return <TableCellLayout>{str}</TableCellLayout>
      },
    }),
  ];
  const pageSize = 20;
  const load = async (page: number) => {
    const data: ComRequest = {
      catelog: "tls",
      type: "list",
      param: { ...compute(page, pageSize) },
    };
    return tlsList(data);;
  }

  const breads = [
    { name: "TLS", icon: <TLSIcon />, path: "/tls/hosts" }
  ]
  const columnSizingOptions = {
    index: {
      minWidth: 50,
      idealWidth: 50,
      autoFitColumns: true,
    },
    version: {
      minWidth: 90,
      idealWidth: 90,
      autoFitColumns: true,
    },
    cs: {
      minWidth: 330,
      idealWidth: 330,
      autoFitColumns: true,

    },
    primary: {
      minWidth: 160,
      idealWidth: 160,
      autoFitColumns: true,
    },
    second: {
      minWidth: 160,
      idealWidth: 160,
      autoFitColumns: true,
    },
    alpn: {
      minWidth: 80,
      idealWidth: 80,
      autoFitColumns: true,
    },
    count: {
      minWidth: 50,
      idealWidth: 50,
      autoFitColumns: true,
    },

  };
  const gridProps = {
    columns, pageSize, columnSizingOptions, load, breads
  };
  return <Grid {...gridProps} />;
}

export default Component;