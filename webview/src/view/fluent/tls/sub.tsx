import { useStore } from "../../store";
import { createTableColumn, TableCellLayout, TableColumnDefinition, Toolbar, ToolbarButton, Tooltip } from "@fluentui/react-components";
import { compute, ComRequest, ITLSInfo } from "../../../share/common";
import Grid from "../table";

import { ActionInfoIcon, ActionMoreIcon, TLSIcon } from "../common";
import { CheckmarkSquareRegular, ShieldQuestionRegular, WarningRegular } from "@fluentui/react-icons";
import { useParams } from "react-router";


// const getLevel = (item: ITLSInfo) => {
//   let rs = 'unknown';
//   if (item && item.list) {
//     for (const it of item.list) {
//       if (it.security === 'low') {
//         return 'low'
//       }
//       if (it.security === 'high') {
//         rs = it.security;
//       }
//     }
//   }
//   return rs;
// }

function Component() {
  const tlsList = useStore((state) => state.tlsConvList);
  const ipMap = new Map();
  const columns: TableColumnDefinition<ITLSInfo>[] = [
    // createTableColumn<ITLSInfo>({
    //   columnId: "index",
    //   renderHeaderCell: () => 'Index',
    //   renderCell: (item) => {
    //     const level = getLevel(item);
    //     let media = <ShieldQuestionRegular />;
    //     let color = '#fabd2f';
    //     switch (level) {
    //       case 'high': {
    //         media = <CheckmarkSquareRegular />;
    //         color = '#b8bb26';
    //         break;
    //       }
    //       case 'low': {
    //         media = <WarningRegular />;
    //         color = '#fb4934';
    //         break;
    //       }
    //       default:
    //     }
    //     return <TableCellLayout media={media} style={{ color }}>{item.index}</TableCellLayout>;
    //     // const id = `tcl-${item.index}`
    //     // return <Tooltip content="Bold" relationship="label" mountNode={document.getElementById(id)}>
    //     //   <TableCellLayout media={media} style={{ color }} id={id}>{item.index}</TableCellLayout>
    //     // </Tooltip>
    //   },
    // }),
    createTableColumn<ITLSInfo>({
      columnId: "version",
      renderHeaderCell: () => 'Ver',
      renderCell: (item) => {
        let str = item.hostname;
        return <TableCellLayout>{str}</TableCellLayout>
      },
    }),
    createTableColumn<ITLSInfo>({
      columnId: "count",
      renderHeaderCell: () => 'Count',
      renderCell: (item) => {
        let str = item.count;
        return <TableCellLayout>{str}</TableCellLayout>
      },
    }),
    // createTableColumn<ITLSInfo>({
    //   columnId: "cs",
    //   renderHeaderCell: () => 'CipherSuite',
    //   renderCell: (item) => {
    //     let str = '';
    //     if (item.list && item.list.length) {
    //       for (const it of item.list) {
    //         if (it.cipher_suite) {
    //           str = it.cipher_suite;
    //           break;
    //         }
    //       }
    //     }
    //     return <TableCellLayout>{str}</TableCellLayout>
    //   },
    // }),
    // createTableColumn<ITLSInfo>({
    //   columnId: "primary",
    //   renderHeaderCell: () => 'Address',
    //   renderCell: (item) => {
    //     let str = '';
    //     if (ipMap.get(item.primary) >= ipMap.get(item.second)) {
    //       str = `${item.primary} -> ${item.second}`;
    //     } else {
    //       str = `${item.second} -> ${item.primary}`;
    //     }
    //     return <TableCellLayout> {str} </TableCellLayout>
    //   },
    // }),
    // createTableColumn<ITLSInfo>({
    //   columnId: "count",
    //   renderHeaderCell: () => 'Count',
    //   renderCell: (item) => <TableCellLayout>{item.list.length}</TableCellLayout>,
    // }),
    // createTableColumn<ITLSInfo>({
    //   columnId: "alpn",
    //   renderHeaderCell: () => 'ALPN',
    //   renderCell: (item) => {
    //     let str = 'N/A';
    //     if (item.list && item.list.length) {
    //       const set = new Set();
    //       for (const it of item.list) {
    //         if (it && it.alpn) {
    //           for (const p of it.alpn) {
    //             set.add(p)
    //           }
    //         }
    //       }
    //       str = [...set].join(', ')
    //     }
    //     return <TableCellLayout>{str}</TableCellLayout>
    //   },
    // }),
    // createTableColumn<ITLSInfo>({
    //   columnId: "sni",
    //   renderHeaderCell: () => 'SNI',
    //   renderCell: (item) => {
    //     let str = 'N/A';
    //     if (item.list && item.list.length) {
    //       const set = new Set();
    //       for (const it of item.list) {
    //         if (it.hostname) {
    //           set.add(it.hostname);
    //         }
    //       }
    //       str = [...set].join(', ')
    //     }
    //     return <TableCellLayout>{str}</TableCellLayout>
    //   },
    // }),
    // createTableColumn<ITLSInfo>({
    //   columnId: "ops",
    //   renderHeaderCell: () => "action",
    //   renderCell: (_item) => {
    //     return <Toolbar aria-label="Default" size="small">
    //       <ToolbarButton icon={ActionInfoIcon()} onClick={() => { }} />
    //       <ToolbarButton icon={ActionMoreIcon()}/>
    //     </Toolbar>
    //   },
    // }),
  ];
  const pageSize = 20;
  const { index } = useParams();

  const load = async (page: number) => {
    const data: ComRequest = {
      catelog: "tls_conv",
      type: "list",
      param: { ...compute(page, pageSize), index },
    };
    const rs = await tlsList(data);
    // ipMap.clear();
    // const add = (key: string) => {
    //   if (!key.length) {
    //     return;
    //   }
    //   const count = ipMap.get(key) || 0;
    //   ipMap.set(key, count + 1);
    // }
    // if (rs) {
    //   for (const item of rs.items) {
    //     add(item.primary);
    //     add(item.second);
    //   }
    // }
    return rs;
    // return []
  }

  const breads = [
    { name: "TLS", icon: <TLSIcon />, path: "/tlslist" },
    { name: `${index}`, path: "/tls/" + index }
  ]
  const columnSizingOptions = {
    index: {
      minWidth: 70,
      idealWidth: 70,
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
      minWidth: 300,
      idealWidth: 300,
      autoFitColumns: true,
    },
    second: {
      minWidth: 300,
      idealWidth: 300,
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
    sni: {
      minWidth: 500,
      idealWidth: 300,
      defaultWidth: 300,
      autoFitColumns: true,
    },
    ops: {
      idealWidth: 100,
      minWidth: 80,
      defaultWidth: 80,
      autoFitColumns: true,
    }
  };
  const gridProps = {
    columns, pageSize, columnSizingOptions, load, breads
  };
  return <Grid {...gridProps} />;
}

export default Component;