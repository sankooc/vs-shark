import React, {useState} from 'react';
import styles from './hexview.module.scss';
// import { Pagination } from '../../../../share/common';
import Pagination from '../../pagination2';

interface HexViewProps {
  data: Uint8Array;
  rowSize?: number;
  pageRows?: number;
  highlight?: { row?: number; col?: number };
}

const DEFAULT_ROW_SIZE = 32;
const DEFAULT_PAGE_ROWS = 20;
const maxLength = DEFAULT_ROW_SIZE * DEFAULT_PAGE_ROWS;

const HexView: React.FC<HexViewProps> = ({
  data,
}) => {
  const [page, setPage] = useState<number>(1);
  const total = data?.length || 0;
  const totalBytes = Math.min(total, maxLength);

  const start = (page - 1) * maxLength;

  const headers = [];
  for(let i = 0;i < DEFAULT_ROW_SIZE ; i +=1){
    headers.push((i).toString(16).padStart(2,'0').toUpperCase());
  }
  const cells = [];
  let index = start;
  const max = start + totalBytes;
  main: for (let row = 0; row < DEFAULT_PAGE_ROWS; row += 1) {
    cells.push(<div key={'key'+row}>{(row + (start/DEFAULT_ROW_SIZE)).toString(16).padStart(4,'0').toUpperCase()}</div>);
    for(let c =0 ; c< DEFAULT_ROW_SIZE; c +=1) {
      if(index >= max || data[index] == undefined){
        break main;
      }
      cells.push(<div className={styles.cell} key={'cell'+index}>{data[index].toString(16).padStart(2,'0').toUpperCase()} </div>)
      index += 1;
    }
  }
  return (
    <div className="flex flex-column flex-1 ">
      
    <div className={styles.hexViewContainer}>
      <div></div>
      {headers.map((h) => (
        <div key={h} className={styles.headerCell}>
          {h}
        </div>
      ))}
      {cells}
    </div>
      <Pagination page={page} total={total} pageSize={maxLength} onPageChange={(_page) => {setPage(_page)}} />
    </div>
  );
};

export default HexView;