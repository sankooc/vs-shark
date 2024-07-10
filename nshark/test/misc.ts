import fs from "node:fs";

export const readLoc = (filepath: string, callback: (arr: Uint8Array) => void): void => {
  const lists: Buffer[] = [];
  const readStream = fs.createReadStream(filepath);
  readStream.on('data', (chunk) => {
    lists.push(chunk as Buffer)
  });
  readStream.on('end', () => {
    const totalBuf = Buffer.concat(lists)
    const arr = new Uint8Array(totalBuf)
    callback(arr);
  });
}