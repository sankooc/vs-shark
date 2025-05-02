import * as vscode from "vscode";
import { Disposable, disposeAll } from "./dispose";
import { ComLog, ComMessage, PcapFile } from "./core/common";
import { FileTailWatcher } from "./fswatcher";
import { PCAPClient } from "./core/client";

function getNonce() {
  let text = "";
  const possible =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  for (let i = 0; i < 32; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}

const DIST = "media";
const ENTRY = "app.js";

const createWebviewHtml = (
  context: vscode.ExtensionContext,
  webview: vscode.Webview,
  file: string,
): string => {
  const scriptUri = webview.asWebviewUri(
    vscode.Uri.joinPath(context.extensionUri, 'dist','web', 'js', 'main.js'),
  );
  const cssUri = webview.asWebviewUri(
    vscode.Uri.joinPath(context.extensionUri, 'dist','web', 'assets', 'main.css'),
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
      chunkSize: 1024 * 1024,
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
    // this._uri = uri;
    // const config = Conf.new(true);
    // const ins = await load(config);
  }

  // public get uri() { return this._uri; }

  dispose(): void {
    super.dispose();
  }
}

export class Client extends PCAPClient {
	constructor(private view: vscode.Webview, private output: vscode.LogOutputChannel) {
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
}

export class PcapViewerProvider
  implements vscode.CustomReadonlyEditorProvider<PcapDocument>
{
  // private static newPawDrawFileId = 1;
  private static output: vscode.LogOutputChannel =
    vscode.window.createOutputChannel("pcap console", { log: true });
  // private static pcapProvider: FrameProvider = new FrameProvider();

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

  constructor(private readonly _context: vscode.ExtensionContext) {}

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
    const info: PcapFile = { name: document.uri.fsPath, size: 0, start: 0 };
    if (!document.client) {
      const client = new Client(webviewPanel.webview, PcapViewerProvider.output);
      client.touchFile(info);
      document.client = client;
      webviewPanel.webview.onDidReceiveMessage(client.handle.bind(client));
    }
    const client = document.client;
    document.watcher.start((buffer: Buffer) => {
      client.update(buffer).then((rs) => {
        PcapViewerProvider.output.info(rs);
      });
      // const rs = document.instance.update(buffer);
      // console.log(rs);
      // const arr = new Uint8Array(buffer);
    });

    this.webviews.add(document.uri, webviewPanel);
  }

  private _requestId = 1;
  private readonly _callbacks = new Map<number, (response: any) => void>();

  private postMessageWithResponse<R = unknown>(
    panel: vscode.WebviewPanel,
    type: string,
    body: any,
  ): Promise<R> {
    const requestId = this._requestId++;
    const p = new Promise<R>((resolve) =>
      this._callbacks.set(requestId, resolve),
    );
    panel.webview.postMessage({ type, requestId, body });
    return p;
  }

  private postMessage(
    panel: vscode.WebviewPanel,
    type: string,
    body: any,
  ): void {
    panel.webview.postMessage({ type, body });
  }

  private onMessage(document: PcapDocument, message: any) {
    switch (message.type) {
      case "response": {
        const callback = this._callbacks.get(message.requestId);
        callback?.(message.body);
        return;
      }
    }
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
