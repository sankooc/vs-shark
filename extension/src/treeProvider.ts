import * as vscode from 'vscode';
import { CTreeItem } from './common';


class CCommand implements vscode.Command {
	title: string;
	command: string;
	tooltip?: string | undefined;
	arguments?: any[] | undefined;
	constructor(title: string, command: string, args: any[]) {
		this.title = title;
		this.command = command;
		this.arguments = args;
	}
}
class Title implements vscode.TreeItemLabel {
	label: string;
	highlights?: [number, number][] | undefined;
	constructor(label: string, highlights: [number, number][]) {
		this.label = label;
		this.highlights = highlights;
	}

}

class NTreeItem extends vscode.TreeItem {
	data: CTreeItem
	children: CTreeItem[] = [];
	raw: Uint8Array;
	constructor(data: CTreeItem, raw: Uint8Array) {
		super(data.label);
		if (data.index) {
			this.command = new CCommand('pickone', 'detail.load', [raw, data.index]);
		}
		this.data = data;
		this.raw = raw;
		this.children = data.children;
		this.collapsibleState = data.children.length ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None;
	}

}

export class FrameProvider implements vscode.TreeDataProvider<NTreeItem> {

	private _onDidChangeTreeData: vscode.EventEmitter<NTreeItem | undefined | void> = new vscode.EventEmitter<NTreeItem | undefined | void>();
	readonly onDidChangeTreeData: vscode.Event<NTreeItem | undefined | void> = this._onDidChangeTreeData.event;
	private items: CTreeItem[] = [];
	private data!: Uint8Array;

	constructor() {
	}
	fire(event: NTreeItem): void {
		this._onDidChangeTreeData.fire(event);
	}
	refresh(items: CTreeItem[], data: Uint8Array): void {
		this.items = items;
		this.data = data;
		this._onDidChangeTreeData.fire();
	}
	getTreeItem(element: NTreeItem): NTreeItem {
		return element;
	}

	getChildren(element?: NTreeItem): Thenable<NTreeItem[]> {
		try {
			if (element) {
				return Promise.resolve(element.children.map((it) => new NTreeItem(it, this.data)));
			} else {
				if (this.items && this.items.length) {
					return Promise.resolve(this.items.map((it) => new NTreeItem(it, this.data)));
				}
			}
		} catch (e) {
			console.error(e);
		}
		return Promise.resolve([]);
	}
}