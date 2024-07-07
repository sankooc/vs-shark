import fs from "node:fs";

export const readLoc = (filepath, callback) => {
  const lists = [];
  const readStream = fs.createReadStream(filepath);
  readStream.on('data', (chunk) => {
    lists.push(chunk)
  });
  readStream.on('end', () => {
    console.log('read complete');
    const totalBuf = Buffer.concat(lists)
    const arr = new Uint8Array(totalBuf)
    callback(arr);
  });

}