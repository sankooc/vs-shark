import { useStore } from "../../store";
import { createTableColumn, TableCellLayout, TableColumnDefinition, Toolbar, ToolbarButton, Tooltip } from "@fluentui/react-components";
import { compute, ComRequest, ITLSInfo } from "../../../share/common";
import Grid from "../table";

import { ActionInfoIcon, ActionMoreIcon, infoLevel, TLSIcon } from "../common";
import { CheckmarkSquareRegular, ShieldQuestionRegular, WarningRegular } from "@fluentui/react-icons";
import { useParams } from "react-router";


function Component() {
  const tlsList = useStore((state) => state.tlsConvList);
  const columns: TableColumnDefinition<ITLSInfo>[] = [
    createTableColumn<ITLSInfo>({
      columnId: "id",
      renderHeaderCell: () => '',
      renderCell: (item) => {
        const secure = item.security || '';
        const [color, media] = infoLevel(secure);
        return <TableCellLayout media={media} style={{color}} ></TableCellLayout>;
      },
    }),
    createTableColumn<ITLSInfo>({
      columnId: "sni",
      renderHeaderCell: () => 'SNI',
      renderCell: (item) => <TableCellLayout>{item.hostname || '<none>'}</TableCellLayout>,
    }),
    createTableColumn<ITLSInfo>({
      columnId: "alpn",
      renderHeaderCell: () => 'ALPN',
      renderCell: (item) => {
        let str = 'N/A';
        if(item && item.alpn){
          str = item.alpn.join(',')
        }
        return <TableCellLayout>{str}</TableCellLayout>
      },
    }),
    createTableColumn<ITLSInfo>({
      columnId: "version",
      renderHeaderCell: () => 'Ver',
      renderCell: (item) => <TableCellLayout>{item.version}</TableCellLayout>,
    }),
    createTableColumn<ITLSInfo>({
      columnId: "cipher_suite",
      renderHeaderCell: () => 'Ciphersuite',
      renderCell: (item) => <TableCellLayout>{item.cipher_suite}</TableCellLayout>,
    }),
    createTableColumn<ITLSInfo>({
      columnId: "count",
      renderHeaderCell: () => 'Count',
      renderCell: (item) => {
        let str = item.count;
        return <TableCellLayout>{str}</TableCellLayout>
      },
    }),

    createTableColumn<ITLSInfo>({
      columnId: "addr",
      renderHeaderCell: () => 'IP',
      renderCell: (item) => {
        const str = `${item.addr_1} -> ${item.addr_2}`;
        return <TableCellLayout>{str}</TableCellLayout>
      },
    }),
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
    id: {
      minWidth: 30,
      idealWidth: 30,
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
    cipher_suite: {
      idealWidth: 300,
      minWidth: 300,
      defaultWidth: 300,
      autoFitColumns: true,
    }
  };
  const gridProps = {
    columns, pageSize, columnSizingOptions, load, breads
  };
  return <Grid {...gridProps} />;
}

export default Component;