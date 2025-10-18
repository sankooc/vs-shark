
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
import { ICounterItem } from "../../../share/gen";

import { PageFrame } from '../table';
import { TLSIcon } from "../common";
import Empty from "../http/content/empty";


export default function Component() {

  const [list, setList] = useState<ICounterItem[]>([]);

  const stat = useStore((state) => state.stat);
  useEffect(() => {
    stat({field: 'tls_sni'}).then(setList);
  }, [])


  const breads = [
    { name: "TLS", icon: <TLSIcon />, path: "/tls/hosts" }
  ]
  if (list.length == 0){
    return <Empty/>
  }
  return (
    <PageFrame breads={breads}>
      <Table arial-label="Default table" style={{ minWidth: "510px" }}>
        <TableHeader>
          <TableRow>
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
            <TableRow key={item.key}>
              <TableCell>
                <TableCellLayout media={<LockClosedColor />}>
                  {item.key}
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