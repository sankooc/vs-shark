import { usePcapStore } from "../../../share/context";
import { createTableColumn, TableCellLayout, TableColumnDefinition, Toolbar, ToolbarButton } from "@fluentui/react-components";
import { compute, ComRequest, ITLSConnect } from "../../../share/common";
import Grid from "../table";

import { ActionInfoIcon, ActionMoreIcon, TLSIcon } from "../common";
import { CheckmarkSquareRegular, ShieldQuestionRegular, WarningRegular } from "@fluentui/react-icons";
import { useNavigate } from "react-router";


const getLevel = (item: ITLSConnect) => {
  let rs = 'unknown';
  if (item && item.list) {
    for (const it of item.list) {
      if (it.security === 'low') {
        return 'low'
      }
      if (it.security === 'high') {
        rs = it.security;
      }
    }
  }
  return rs;
}

function Component() {
  const tlsList = usePcapStore((state) => state.tlsList);
  const navigate = useNavigate();
  const ipMap = new Map();
  const onClick = (item: ITLSConnect) => {
    // cachehttp(item);
    const index = item.index;
    if (undefined != index) {
      navigate('/tls/' + index);
    }
  };
  const columns: TableColumnDefinition<ITLSConnect>[] = [
    createTableColumn<ITLSConnect>({
      columnId: "index",
      renderHeaderCell: () => 'Index',
      renderCell: (item) => {
        const level = getLevel(item);
        let media = <ShieldQuestionRegular />;
        let color = '#fabd2f';
        switch (level) {
          case 'high': {
            media = <CheckmarkSquareRegular />;
            color = '#b8bb26';
            break;
          }
          case 'low': {
            media = <WarningRegular />;
            color = '#fb4934';
            break;
          }
          default:
        }
        return <TableCellLayout media={media} style={{ color }}>{item.index}</TableCellLayout>;
        // const id = `tcl-${item.index}`
        // return <Tooltip content="Bold" relationship="label" mountNode={document.getElementById(id)}>
        //   <TableCellLayout media={media} style={{ color }} id={id}>{item.index}</TableCellLayout>
        // </Tooltip>
      },
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
      columnId: "primary",
      renderHeaderCell: () => 'Address',
      renderCell: (item) => {
        let str = '';
        if (ipMap.get(item.primary) >= ipMap.get(item.second)) {
          str = `${item.primary} -> ${item.second}`;
        } else {
          str = `${item.second} -> ${item.primary}`;
        }
        return <TableCellLayout> {str} </TableCellLayout>
      },
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
    createTableColumn<ITLSConnect>({
      columnId: "ops",
      renderHeaderCell: () => "action",
      renderCell: (_item) => {
        return <Toolbar aria-label="Default" size="small">
          <ToolbarButton icon={ActionInfoIcon()} onClick={() => { onClick(_item) }} />
          <ToolbarButton icon={ActionMoreIcon()}/>
        </Toolbar>
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
    const rs = await tlsList(data);
    ipMap.clear();
    const add = (key: string) => {
      if (!key.length) {
        return;
      }
      const count = ipMap.get(key) || 0;
      ipMap.set(key, count + 1);
    }
    if (rs) {
      for (const item of rs.items) {
        add(item.primary);
        add(item.second);
      }
    }
    return rs;
  }

  const breads = [
    { name: "TLS", icon: <TLSIcon />, path: "/tlslist" }
  ]
  const columnSizingOptions = {
    index: {
      minWidth: 70,
      idealWidth: 70,
    },
    version: {
      minWidth: 90,
      idealWidth: 90,
    },
    cs: {
      minWidth: 330,
      idealWidth: 330,

    },
    primary: {
      minWidth: 300,
      idealWidth: 300,
    },
    second: {
      minWidth: 300,
      idealWidth: 300,
    },
    alpn: {
      minWidth: 80,
      idealWidth: 80,
    },
    count: {
      minWidth: 50,
      idealWidth: 50,
    },
    sni: {
      minWidth: 500,
      idealWidth: 300,
      defaultWidth: 300,
    },
    ops: {
      idealWidth: 100,
      minWidth: 80,
      defaultWidth: 80,
    }
  };
  const gridProps = {
    columns, pageSize, columnSizingOptions, load, breads
  };
  return <Grid {...gridProps} />;
}

export default Component;