import * as vscode from 'vscode';
import { CTreeItem } from './common';


class NTreeItem extends vscode.TreeItem {
	data: CTreeItem
    children: CTreeItem[] = [];
	constructor(data: CTreeItem){
		super(data.label);
		this.data = data;
		this.children = data.children;
		this.collapsibleState = data.children.length?vscode.TreeItemCollapsibleState.Collapsed: vscode.TreeItemCollapsibleState.None;
	}

}

export class FrameProvider implements vscode.TreeDataProvider<NTreeItem> {

	private _onDidChangeTreeData: vscode.EventEmitter<NTreeItem | undefined | void> = new vscode.EventEmitter<NTreeItem | undefined | void>();
	readonly onDidChangeTreeData: vscode.Event<NTreeItem | undefined | void> = this._onDidChangeTreeData.event;
	private items: CTreeItem[] = [];
	
	constructor() {
	}
    fire(event: NTreeItem): void {
		this._onDidChangeTreeData.fire(event);
	}
	refresh(items: CTreeItem[]): void {
		this.items = items;
		this._onDidChangeTreeData.fire();
	}
	getTreeItem(element: NTreeItem): NTreeItem {
		return element;
	}

	getChildren(element?: NTreeItem): Thenable<NTreeItem[]> {
		try{
			if(element){
				// const its: NTreeItem[] = element.children.map((it) => new NTreeItem(it));
				return Promise.resolve(element.children.map((it) => new NTreeItem(it)));
			} else {
				if(this.items && this.items.length){
					// const its: NTreeItem[] = this.items.map((it) => new NTreeItem(it));
					return Promise.resolve(this.items.map((it) => new NTreeItem(it)));
				}
			}
		}catch(e){
			console.error(e);
		}
		return Promise.resolve([]);
	}
}

class CCommand implements vscode.Command {
    title: string;
    command: string;
    tooltip?: string | undefined;
    arguments?: any[] | undefined;
    constructor(title: string, command: string){
        this.title = title;
        this.command = command;

    }
}
class Title implements vscode.TreeItemLabel {
    label: string;
    highlights?: [number, number][] | undefined;
    constructor(label: string, highlights: [number, number][]){
        this.label = label;
        this.highlights = highlights;
    }
    
}