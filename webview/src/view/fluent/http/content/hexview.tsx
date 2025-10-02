import React from 'react';
import styles from './hexview.module.scss';

interface HexViewProps {
  data: Uint8Array;
  maxLength?: number;
}

const HexView: React.FC<HexViewProps> = ({ data, maxLength = 32 }) => {
  if (!data || data.length === 0) {
    return null;
  }

  const toHexString = (bytes: Uint8Array) =>
    Array.from(bytes)
      .map((b) => b.toString(16).padStart(2, '0').toUpperCase())
      .join(' ');

  const isTruncated = data.length > maxLength;
  const displayedData = isTruncated ? data.slice(0, maxLength) : data;
  const hexString = toHexString(displayedData);
  const remainingBytes = data.length - maxLength;

  return (
    <div className={styles.hexView + " flex-grow-1"}>
      <span>{hexString}</span>
      {isTruncated && (
        <span className={styles.truncationIndicator}>
          {' ... '}(+{remainingBytes} bytes)
        </span>
      )}
    </div>
  );
};

export default HexView;