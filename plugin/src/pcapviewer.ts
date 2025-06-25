import * as vscode from "vscode";
import { Disposable, disposeAll } from "./dispose";
import { ComLog, ComMessage, ComType, PcapFile } from "./share/common";
import { FileTailWatcher } from "./fswatcher";
import { BATCH_SIZE, PCAPClient } from "./share/client";

function getNonce() {
  let text = "";
  const possible =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  for (let i = 0; i < 32; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}

const ENTRY = "app.js";

const createWebviewHtml = (
  context: vscode.ExtensionContext,
  webview: vscode.Webview,
  file: string,
): string => {
  const scriptUri = webview.asWebviewUri(
    vscode.Uri.joinPath(context.extensionUri, 'dist', 'web', 'js', 'main.js'),
  );
  const cssUri = webview.asWebviewUri(
    vscode.Uri.joinPath(context.extensionUri, 'dist', 'web', 'assets', 'main.css'),
  );
  const nonce = getNonce();
  const result = `<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>VSCode Webview Dev</title>
    <script nonce="${nonce}" type="module" crossorigin src="${scriptUri}"></script>
    <link rel="stylesheet" nonce="${nonce}" crossorigin href="${cssUri}">
  </head>
  <body>
    <div id="app"></div>
  </body>
</html>`;
  return result;
};


class PcapDocument extends Disposable implements vscode.CustomDocument {
  static async create(
    uri: vscode.Uri,
    backupId: string | undefined,
  ): Promise<PcapDocument | PromiseLike<PcapDocument>> {
    const dataFile =
      typeof backupId === "string" ? vscode.Uri.parse(backupId) : uri;
    const watcher = new FileTailWatcher(dataFile.fsPath, {
      chunkSize: BATCH_SIZE,
      intervalMs: 1000,
    });

    return new PcapDocument(dataFile, watcher);
  }

  // private readonly _uri: vscode.Uri;

  public client?: Client;
  private constructor(
    public uri: vscode.Uri,
    // public instance: WContext,
    public watcher: FileTailWatcher,
  ) {
    super();
  }

  // public get uri() { return this._uri; }

  dispose(): void {
    if (this.client) {
      console.log(this.uri.path + " dispose");
      this.client.dispose();
    }
    super.dispose();
  }
}

export class Client extends PCAPClient {
  doReady(): void {
    this.init();
    const info: PcapFile = { name: this.watcher.filePath, size: 0, start: 0 };
    this.handle(ComMessage.new(ComType.TOUCH_FILE, info));
    this.watcher.start((buffer: Buffer) => {
      this.handle(ComMessage.new(ComType.PROCESS_DATA, { data: bufferToUint8Array(buffer) }));
    });
  }
  async pickData(start: number, end: number): Promise<Uint8Array> {
    const buffer = await this.watcher.readRandomAccess(start, (end - start));
    return bufferToUint8Array(buffer);
  }
  appendData(data: Uint8Array): void {
    // TODO
  }
  constructor(private view: vscode.Webview, private output: vscode.LogOutputChannel, private watcher: FileTailWatcher) {
    super();
  }
  printLog(log: ComLog): void {
    switch (log.level) {
      case 'error':
        vscode.window.showErrorMessage(log.msg.toString());
        break;
    }
    this.output.info(log.msg.toString());
  }
  emitMessage(msg: ComMessage<any>): void {
    this.view.postMessage(msg);
  }
  dispose(): void{
    this.watcher.stop();
    this.ctx?.free();
  }
}




const bufferToUint8Array = (buffer: Buffer): Uint8Array => {
  return new Uint8Array(buffer.buffer, buffer.byteOffset, buffer.byteLength);
  // return buffer;
};

export class PcapViewerProvider
  implements vscode.CustomReadonlyEditorProvider<PcapDocument> {
  // private static newPawDrawFileId = 1;
  private static output: vscode.LogOutputChannel =
    vscode.window.createOutputChannel("pcap console", { log: true });

  public get output(): vscode.LogOutputChannel {
    return PcapViewerProvider.output;
  }

  public static register(context: vscode.ExtensionContext): vscode.Disposable {
    return vscode.window.registerCustomEditorProvider(
      PcapViewerProvider.viewType,
      new PcapViewerProvider(context),
      {
        webviewOptions: {
          retainContextWhenHidden: true,
        },
        supportsMultipleEditorsPerDocument: false,
      },
    );
  }

  private static readonly viewType = "proto.pcapng";

  /**
   * Tracks all known webviews
   */
  private readonly webviews = new WebviewCollection();

  constructor(private readonly _context: vscode.ExtensionContext) { }

  //#region CustomEditorProvider

  async openCustomDocument(
    uri: vscode.Uri,
    openContext: { backupId?: string },
    _token: vscode.CancellationToken,
  ): Promise<PcapDocument> {
    return await PcapDocument.create(uri, openContext.backupId);
  }

  async resolveCustomEditor(
    document: PcapDocument,
    webviewPanel: vscode.WebviewPanel,
    _token: vscode.CancellationToken,
  ): Promise<void> {
    webviewPanel.title = "";
    webviewPanel.webview.options = {
      enableScripts: true,
    };
    webviewPanel.webview.html = createWebviewHtml(
      this._context,
      webviewPanel.webview,
      ENTRY,
    );
    vscode.window.onDidChangeActiveColorTheme(theme => {
      // console.log('theme', theme);
      // const customColors = vscode.workspace.getConfiguration('workbench').get('colorCustomizations');
      // const tokens = vscode.workspace.getConfiguration('workbench.colorCustomizations');
      // console.log('theme', theme);
      // const themes = vscode.extensions.all.flatMap(ext => ext.packageJSON?.contributes?.themes || []).filter(t => t.label === vscode.window.activeColorTheme.kind);

      // const themeFile = themes[0]?.path;
      // console.log('theme', theme);
      webviewPanel.webview.postMessage({
        type: 'vscode-theme-change',
        themeKind: theme.kind,
      });
    });

    // const info: PcapFile = { name: document.uri.fsPath, size: 0, start: 0 };
    if (!document.client) {
      // document.watcher;
      const client = new Client(webviewPanel.webview, PcapViewerProvider.output, document.watcher);
      // client.handle(ComMessage.new(ComType.TOUCH_FILE, info));
      document.client = client;
      webviewPanel.webview.onDidReceiveMessage((data) => {
        const id = data.id;
        const type = data.type;
        if (type === ComType.DATA && id) {
          const { start, end } = data.body;
          const size = end - start;
          //todo
          if (start >= 0 && size > 0) {
            document.watcher.readRandomAccess(start, size).then((buffer: Buffer) => {
              const data = bufferToUint8Array(buffer);
              client.emitMessage({
                type: ComType.RESPONSE,
                id,
                body: { data },
              });
            });
          } else {
            document.watcher.readRandomAccess(start, size).then((data) => {
              client.emitMessage({
                type: ComType.RESPONSE,
                id,
                body: {},
              });
            });
          }
          return;
        }
        client.handle(data);
      });
    }
    // const client = document.client;

    this.webviews.add(document.uri, webviewPanel);
  }
}

class WebviewCollection {
  private readonly _webviews = new Set<{
    readonly resource: string;
    readonly webviewPanel: vscode.WebviewPanel;
  }>();

  /**
   * Get all known webviews for a given uri.
   */
  public *get(uri: vscode.Uri): Iterable<vscode.WebviewPanel> {
    const key = uri.toString();
    for (const entry of this._webviews) {
      if (entry.resource === key) {
        yield entry.webviewPanel;
      }
    }
  }

  /**
   * Add a new webview to the collection.
   */
  public add(uri: vscode.Uri, webviewPanel: vscode.WebviewPanel) {
    const entry = { resource: uri.toString(), webviewPanel };
    this._webviews.add(entry);

    webviewPanel.onDidDispose(() => {
      this._webviews.delete(entry);
    });
  }
}
