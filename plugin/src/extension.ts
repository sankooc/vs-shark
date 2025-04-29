import * as vscode from 'vscode';
import { PcapViewerProvider } from './pcapviewer';

export function activate(context: vscode.ExtensionContext) {
	context.subscriptions.push(PcapViewerProvider.register(context));
}
 
export function deactivate() {}
