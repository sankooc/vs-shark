import { usePcapStore } from "../../../share/context";
import { createTableColumn, TableCellLayout, TableColumnDefinition } from "@fluentui/react-components";
import { compute, ComRequest, ITLSInfo } from "../../../share/common";
import Grid from "../table";

import { infoLevel, TLSIcon } from "../common";
import { useParams } from "react-router";


function Component() {
  const tlsList = usePcapStore((state) => state.tlsConvList);
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
        return <TableCellLayout>{item.count}</TableCellLayout>
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
    return tlsList(data)
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