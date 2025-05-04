// import * as vs from "vscode-webview";
import { ComLog, ComMessage, ComType } from "../share/common";

let vscode: any;
if (window["acquireVsCodeApi"]) {
  vscode = acquireVsCodeApi();
}
export const emitMessage = (message: ComMessage<any>) => {
  if (vscode) {
    vscode.postMessage(message);
  } else {
    window.top?.postMessage(message, "*");
  }
};

export const onMessage = (type: string, listen: (msg: any) => void) => {
  window.addEventListener(type, listen);
  return () => {
    window.removeEventListener(type, listen);
  };
};

export const log = (level: string, msg: any): void => {
  emitMessage(new ComMessage(ComType.log, new ComLog(level, msg)));
};
export const trace = (msg: any) => {
  log("trace", msg);
};

export const info = (msg: any) => {
  log("info", msg);
};

export const warn = (msg: any): void => {
  log("warn", msg);
};

export const error = (msg: any): void => {
  log("error", msg);
};
