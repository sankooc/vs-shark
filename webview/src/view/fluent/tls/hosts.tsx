
import { useStore } from "../../store";
import { useState, useEffect } from 'react';
import {
  LockClosedColor,
} from "@fluentui/react-icons";
import {
  TableBody,
  TableCell,
  TableRow,
  Table,
  TableHeader,
  TableHeaderCell,
  TableCellLayout,
} from "@fluentui/react-components";

import { PageFrame } from '../table';
import { TLSIcon } from "../common";
import Empty from "../http/content/empty";
import { ITLSInfo } from "../../../share/common";


export default function Component() {

  const [list, setList] = useState<ITLSInfo[]>([]);

  const tlsList = useStore((state) => state.tlsList);
  useEffect(() => {
    tlsList().then(setList);
    // stat({field: 'tls_sni'}).then(setList);
  }, [])


  const breads = [
    { name: "TLS", icon: <TLSIcon />, path: "/tls/hosts" }
  ]
  if (list.length == 0){
    return <Empty/>
  }
  const headCell = (item: ITLSInfo) => {
    if(item.alpn && item.alpn.length){
      return <TableCellLayout media={<LockClosedColor />}>
                  {item.alpn.join(',')}
    </TableCellLayout>
    }
    return <TableCellLayout media={<LockClosedColor />} style={{color: '#999', fontStyle: 'italic'}}>
                  none
    </TableCellLayout>
  }
  return (
    <PageFrame breads={breads}>
      <Table size="small" style={{ minWidth: "510px" }}>
        <TableHeader>
          <TableRow>
            <TableHeaderCell style={{width: "100px"}}>
              Protocol
            </TableHeaderCell>
            <TableHeaderCell>
              Host
            </TableHeaderCell>
            <TableHeaderCell>
              Count
            </TableHeaderCell>
          </TableRow>
        </TableHeader>
        <TableBody>
          {list.map((item) => (
            <TableRow key={item.hostname}>
              <TableCell>
                {headCell(item)}
              </TableCell>
              <TableCell>
                <TableCellLayout>
                  {item.hostname}
                </TableCellLayout>
              </TableCell>
              <TableCell>
                <TableCellLayout>
                  {item.count}
                </TableCellLayout>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </PageFrame>
  );
};