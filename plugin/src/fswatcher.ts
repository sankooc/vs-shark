import * as fs from "fs";

interface FileTailWatcherOptions {
  chunkSize?: number;
  intervalMs?: number;
}

type OnDataCallback = (data: Buffer) => void;

export class FileTailWatcher {
  private filePath: string;
  private chunkSize: number;
  private intervalMs: number;
  private position: number = 0;
  private interval: NodeJS.Timeout | null = null;
  private running: boolean = false;
  private fd: fs.promises.FileHandle | null = null;

  constructor(filePath: string, options: FileTailWatcherOptions = {}) {
    this.filePath = filePath;
    this.chunkSize = options.chunkSize ?? 1024 * 1024; // 默认 1MB
    this.intervalMs = options.intervalMs ?? 1000; // 默认 1 秒
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
        onData(buffer.subarray(0, bytesRead));
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
            onData(buffer.subarray(0, bytesRead));
          }
        }
      } catch (err) {
        console.error("读取新增内容出错:", err);
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

    console.log("文件监控已停止");
  }
}
