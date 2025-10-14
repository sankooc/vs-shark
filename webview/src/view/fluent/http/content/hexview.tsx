import React from 'react';
import styles from './hexview.module.scss';

interface HexViewProps {
  data: Uint8Array;
  maxLength?: number;
  rowSize?: number;
  pageRows?: number;
  highlight?: { row?: number; col?: number };
}

const DEFAULT_ROW_SIZE = 32;
const DEFAULT_PAGE_ROWS = 20;

const HexView: React.FC<HexViewProps> = ({
  data,
  maxLength = 32 * 30,
}) => {

  const totalBytes = Math.min(data?.length || 0, maxLength);

  const start = 0;

  const headers = [];
  for(let i = 0;i < DEFAULT_ROW_SIZE ; i +=1){
    headers.push((i + start).toString(16).padStart(2,'0').toUpperCase());
  }
  const cells = [];
  let index = start;
  for (let row = 0; row < DEFAULT_PAGE_ROWS; row += 1) {
    cells.push(<div key={'key'+row}>{(row).toString(16).padStart(4,'0').toUpperCase()}</div>);
    for(let c =0 ; c< DEFAULT_ROW_SIZE; c +=1) {
      if(index >= totalBytes){
        break;
      }
      cells.push(<div className={styles.cell} key={'cell'+index}>{data[index].toString(16).padStart(2,'0').toUpperCase()} </div>)
      index += 1;
    }
  }

  return (
    <div className={styles.hexViewContainer}>
      <div> </div>
      {headers.map((h) => (
        <div key={h} className={styles.headerCell}>
          {h}
        </div>
      ))}
      {cells}
    </div>
  );
};

export default HexView;