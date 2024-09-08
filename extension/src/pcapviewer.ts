import * as vscode from 'vscode';
import { Disposable, disposeAll } from './dispose';
// import { Client } from './client';
// import { Panel, ComMessage, ComLog, CTreeItem, HexV } from './common';
// import { FrameProvider } from './treeProvider';

function getNonce() {
	let text = '';
	const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
	for (let i = 0; i < 32; i++) {
		text += possible.charAt(Math.floor(Math.random() * possible.length));
	}
	return text;
}

const createWebviewHtml = (context: vscode.ExtensionContext, webview: vscode.Webview, file: string): string => {
	const scriptUri = webview.asWebviewUri(vscode.Uri.joinPath(context.extensionUri, 'media', file));
	const nonce = getNonce();
	const result = `<!DOCTYPE html>
		<html lang="en">
			<head>
			<meta charset="utf-8" />
			<meta name="viewport" content="width=device-width, initial-scale=1" />
			</head>
			<body>
			<div id="app"></div>
			<script nonce="${nonce}" src="${scriptUri}"></script>
			</body>
		</html>
		`;
	return result;
}

// class DetailProvider implements vscode.WebviewViewProvider {
// 	// public static instance: DetailProvider = new DetailProvider();
// 	context: vscode.ExtensionContext;
// 	webview!: vscode.WebviewView;
// 	_extensionUri: vscode.Uri;
// 	constructor(context: vscode.ExtensionContext) {
// 		this.context = context;
// 		this._extensionUri = context.extensionUri
// 	}
// 	resolveWebviewView(webviewView: vscode.WebviewView, context: vscode.WebviewViewResolveContext<unknown>, token: vscode.CancellationToken): void | Thenable<void> {
// 		this.webview = webviewView;
// 		webviewView.webview.options = {
// 			enableScripts: true,
// 			localResourceRoots: [
// 				this._extensionUri
// 			]
// 		};
// 		webviewView.webview.html = createWebviewHtml(this.context, webviewView.webview, 'hex.js');
// 	}
// 	load(data: Uint8Array, hightlight: [number, number]): void {
// 		if (!this.webview) return;
// 		const it = new HexV(data);
// 		it.index = hightlight;
// 		this.webview.webview.postMessage(new ComMessage('hex-data', it));
// 	}
// }
/**
 * Define the document (the data model) used for paw draw files.
 */
class PcapDocument extends Disposable implements vscode.CustomDocument {

	static async create(
		uri: vscode.Uri,
		backupId: string | undefined
	): Promise<PcapDocument | PromiseLike<PcapDocument>> {
		const dataFile = typeof backupId === 'string' ? vscode.Uri.parse(backupId) : uri;
		const fileData = await PcapDocument.readFile(dataFile);
		return new PcapDocument(uri, fileData);
	}

	private static async readFile(uri: vscode.Uri): Promise<Uint8Array> {
		if (uri.scheme === 'untitled') {
			return new Uint8Array();
		}
		return new Uint8Array(await vscode.workspace.fs.readFile(uri));
	}

	private readonly _uri: vscode.Uri;

	private _documentData: Uint8Array;

	private constructor(
		uri: vscode.Uri,
		initialContent: Uint8Array
	) {
		super();
		this._uri = uri;
		this._documentData = initialContent;
	}

	public get uri() { return this._uri; }

	public get documentData(): Uint8Array { return this._documentData; }

	dispose(): void {
		super.dispose();
	}
}


// export class PCAPClient extends Client {
// 	selectFrame(no: number): void {
// 		const items = this.buildFrameTree(no);
// 		const data = this.getPacket(no);
// 		// vscode.commands.executeCommand('pcaptree.load', no, items, data);
// 		// vscode.commands.executeCommand('detail.load', data, [0,0]);

// 		this.emitMessage(Panel.TREE, new ComMessage('frame', { items, data }));
// 		this.renderHexView(new HexV(data));
// 	}
// 	renderHexView(data: HexV): void {
// 		this.emitMessage(Panel.DETAIL, new ComMessage('hex-data', data));
// 	}
// 	webviewPanel: vscode.WebviewPanel;
// 	output: vscode.LogOutputChannel;
// 	treeProvider: FrameProvider;
// 	constructor(doc: Uint8Array, webviewPanel: vscode.WebviewPanel, output: vscode.LogOutputChannel, treeProvider: FrameProvider) {
// 		super();
// 		this.data = doc;
// 		this.webviewPanel = webviewPanel;
// 		this.output = output;
// 		this.treeProvider = treeProvider;
// 	}
// 	emitMessage(panel: Panel, msg: ComMessage<any>): void {
// 		this.webviewPanel.webview.postMessage(msg);
// 	}
// 	printLog(log: ComLog): void {
// 		switch(log.level){
// 			case 'error':
// 				vscode.window.showErrorMessage(log.msg.toString());
// 				break;
// 		}
// 		this.output.info(log.msg.toString());
// 	}
// 	init(): void {
// 		super.init();
// 	}

// }

export class PcapViewerProvider implements vscode.CustomReadonlyEditorProvider<PcapDocument> {

	// private static newPawDrawFileId = 1;
	private static output: vscode.LogOutputChannel = vscode.window.createOutputChannel('pcap console', { log: true });
	// private static pcapProvider: FrameProvider = new FrameProvider();

	public get output(): vscode.LogOutputChannel { return this.output };

	public static register(context: vscode.ExtensionContext): vscode.Disposable {
		// vscode.window.registerTreeDataProvider('pcap.tree', PcapViewerProvider.pcapProvider);
		// vscode.commands.registerCommand('pcaptree.load', (no: number, items: CTreeItem[], data: Uint8Array) => { PcapViewerProvider.pcapProvider.refresh(items, data) });
		// const detailProvider = new DetailProvider(context);
		// vscode.commands.registerCommand('detail.load', (data: Uint8Array, index: [number, number]) => {
		// 	detailProvider.load(data, index);
		// });
		// vscode.window.registerWebviewViewProvider("pcap.detail", detailProvider, {
		// 	webviewOptions: {
		// 		retainContextWhenHidden: true,
		// 	},
		// });
		return vscode.window.registerCustomEditorProvider(
			PcapViewerProvider.viewType,
			new PcapViewerProvider(context),
			{
				webviewOptions: {
					retainContextWhenHidden: true,
				},
				supportsMultipleEditorsPerDocument: false,
			});
	}

	private static readonly viewType = 'proto.pcapng';

	/**
	 * Tracks all known webviews
	 */
	private readonly webviews = new WebviewCollection();

	constructor(
		private readonly _context: vscode.ExtensionContext
	) { }

	//#region CustomEditorProvider

	async openCustomDocument(
		uri: vscode.Uri,
		openContext: { backupId?: string },
		_token: vscode.CancellationToken
	): Promise<PcapDocument> {
		const document: PcapDocument = await PcapDocument.create(uri, openContext.backupId);

		const listeners: vscode.Disposable[] = [];

		return document;
	}

	async resolveCustomEditor(
		document: PcapDocument,
		webviewPanel: vscode.WebviewPanel,
		_token: vscode.CancellationToken
	): Promise<void> {

		this.webviews.add(document.uri, webviewPanel);

		webviewPanel.webview.options = {
			enableScripts: true,
		};
		webviewPanel.webview.html = createWebviewHtml(this._context, webviewPanel.webview, 'app.js');
		// this.getHtmlForWebview(webviewPanel.webview);

		// const client = new PCAPClient(document.documentData, webviewPanel, PcapViewerProvider.output, PcapViewerProvider.pcapProvider);


		webviewPanel.webview.onDidReceiveMessage((msg) => {
			const { type, body } = msg
			try {
				switch (type) {
					case 'ready':
						try {
							// console.log('inited');
							const start = Date.now();
							webviewPanel.webview.postMessage({ type: 'raw-data', body: document.documentData });
							// console.log('spend', Date.now() - start);
						} catch (e) {
							console.error(e);
						}
						break;
					case 'log':
						// console.log(body);
						if(body.level === 'error'){
							vscode.window.showErrorMessage(body.msg?.toString());
						}
						this.output.appendLine(JSON.stringify(body));
						break;
					default:
						// console.log('unknown type', msg.type);
				}
			} catch (e) {
				console.error(e);
			}
		});
	}

	private _requestId = 1;
	private readonly _callbacks = new Map<number, (response: any) => void>();

	private postMessageWithResponse<R = unknown>(panel: vscode.WebviewPanel, type: string, body: any): Promise<R> {
		const requestId = this._requestId++;
		const p = new Promise<R>(resolve => this._callbacks.set(requestId, resolve));
		panel.webview.postMessage({ type, requestId, body });
		return p;
	}

	private postMessage(panel: vscode.WebviewPanel, type: string, body: any): void {
		panel.webview.postMessage({ type, body });
	}

	private onMessage(document: PcapDocument, message: any) {
		switch (message.type) {
			case 'response':
				{
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