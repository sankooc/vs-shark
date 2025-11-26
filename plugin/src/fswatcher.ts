import * as fs from "fs";
import { Range } from "rshark";
import { PcapFile } from "./share/common";
import { IProgressStatus } from "./share/gen";

interface FileTailWatcherOptions {
  chunkSize?: number;
  intervalMs?: number;
}

type OnDataCallback = (data: Buffer, progress: IProgressStatus) => void;

export class FileTailWatcher {
  public filePath: string;
  private chunkSize: number;
  private intervalMs: number;
  private position: number = 0;
  private interval: NodeJS.Timeout | null = null;
  private running: boolean = false;
  private fd: fs.promises.FileHandle | null = null;

  constructor(filePath: string, options: FileTailWatcherOptions = {}) {
    this.filePath = filePath;
    this.chunkSize = options.chunkSize ?? 1024 * 1024;
    this.intervalMs = options.intervalMs ?? 1000;
  }

  info(): PcapFile {
    const state = fs.statSync(this.filePath);
    return { name: this.filePath, size: state.size };
  }

  async start(onData: OnDataCallback): Promise<void> {
    if (this.running) {
      return;
    }
    this.running = true;

    this.fd = await fs.promises.open(this.filePath, "r");

    const stats = await this.fd.stat();
    const totalSize = stats.size;

    while (this.position < totalSize && this.running) {
      const toRead = Math.min(this.chunkSize, totalSize - this.position);
      const buffer = Buffer.alloc(toRead);
      const { bytesRead } = await this.fd.read(
        buffer,
        0,
        toRead,
        this.position,
      );
      this.position += bytesRead;

      if (bytesRead > 0) {
        onData(buffer.subarray(0, bytesRead), {total: totalSize, cursor: this.position});
      }
    }

    this.interval = setInterval(async () => {
      if (!this.running || !this.fd) {
        return;
      }

      try {
        const stats = await this.fd.stat();
        const newSize = stats.size;

        if (newSize > this.position) {
          const toRead = newSize - this.position;
          const buffer = Buffer.alloc(toRead);
          const { bytesRead } = await this.fd.read(
            buffer,
            0,
            toRead,
            this.position,
          );
          this.position += bytesRead;

          if (bytesRead > 0) {
            onData(buffer.subarray(0, bytesRead), {total: newSize, cursor: this.position});
          }
        }
      } catch (err) {
        console.error("read file failed:", err);
      }
    }, this.intervalMs);
  }

  async readRandomAccess(position: number, length: number): Promise<Buffer> {
    const buffer = Buffer.alloc(length);
    await this.fd!.read(
      buffer,
      0,
      length,
      position
    );
    return buffer;
  }
  load(range: Range): Buffer | undefined {
    const fd = fs.openSync(this.filePath, "r+");
    try {
      const position = range.start;
      const len = range.end - position;
      return this.readRandomAccessSync(fd, position, len);
    } catch (e) {
      console.error(e);
    } finally {
      fs.closeSync(fd);
    }
    return;
  }
  loads(ranges: Range[]): Buffer | undefined {
    const fd = fs.openSync(this.filePath, "r+");
    try {
      const list = ranges.map((range: Range) => {
        const position = range.start;
        const len = range.end - position;
        return this.readRandomAccessSync(fd, position, len);
      });
      return Buffer.concat(list);
    } catch (e) {
      console.error(e);
    } finally {
      fs.closeSync(fd);
    }
    return;
  }
  readRandomAccessSync(fd: number, position: number, length: number): Buffer {
    const buffer = Buffer.alloc(length);
    const len = fs.readSync(fd, buffer, 0, length, position);
    return buffer;
  }

  async stop(): Promise<void> {
    if (!this.running) {
      return;
    }
    this.running = false;

    if (this.interval) {
      clearInterval(this.interval);
      this.interval = null;
    }

    if (this.fd) {
      await this.fd.close();
      this.fd = null;
    }

    console.log("stop watching file");
  }
}
