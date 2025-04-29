"use strict";
var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __commonJS = (cb, mod) => function __require() {
  return mod || (0, cb[__getOwnPropNames(cb)[0]])((mod = { exports: {} }).exports, mod), mod.exports;
};
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(
  // If the importer is in node compatibility mode or this is not an ESM
  // file that has been converted to a CommonJS file using a Babel-
  // compatible transform (i.e. "__esModule" has not been set), then set
  // "default" to the CommonJS "module.exports" for node compatibility.
  isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target,
  mod
));
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// node_modules/.pnpm/wasm-pcps@file+..+crates+wasm2+node/node_modules/wasm-pcps/wasm_pcps.js
var require_wasm_pcps = __commonJS({
  "node_modules/.pnpm/wasm-pcps@file+..+crates+wasm2+node/node_modules/wasm-pcps/wasm_pcps.js"(exports2, module2) {
    var imports = {};
    imports["__wbindgen_placeholder__"] = module2.exports;
    var wasm;
    var { TextDecoder } = require("util");
    var cachedTextDecoder = new TextDecoder("utf-8", { ignoreBOM: true, fatal: true });
    cachedTextDecoder.decode();
    var cachedUint8ArrayMemory0 = null;
    function getUint8ArrayMemory0() {
      if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
      }
      return cachedUint8ArrayMemory0;
    }
    function getStringFromWasm0(ptr, len) {
      ptr = ptr >>> 0;
      return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
    }
    function _assertClass(instance, klass) {
      if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
      }
    }
    module2.exports.load = function(conf) {
      _assertClass(conf, Conf2);
      var ptr0 = conf.__destroy_into_raw();
      const ret = wasm.load(ptr0);
      return WContext2.__wrap(ret);
    };
    var ConfFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
    }, unregister: () => {
    } } : new FinalizationRegistry((ptr) => wasm.__wbg_conf_free(ptr >>> 0, 1));
    var Conf2 = class _Conf {
      static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(_Conf.prototype);
        obj.__wbg_ptr = ptr;
        ConfFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
      }
      __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ConfFinalization.unregister(this);
        return ptr;
      }
      free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_conf_free(ptr, 0);
      }
      /**
       * @param {boolean} resolve_all
       * @returns {Conf}
       */
      static new(resolve_all) {
        const ret = wasm.conf_new(resolve_all);
        return _Conf.__wrap(ret);
      }
      /**
       * @returns {boolean}
       */
      resolve_all() {
        const ret = wasm.conf_resolve_all(this.__wbg_ptr);
        return ret !== 0;
      }
    };
    module2.exports.Conf = Conf2;
    var WContextFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
    }, unregister: () => {
    } } : new FinalizationRegistry((ptr) => wasm.__wbg_wcontext_free(ptr >>> 0, 1));
    var WContext2 = class _WContext {
      static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(_WContext.prototype);
        obj.__wbg_ptr = ptr;
        WContextFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
      }
      __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WContextFinalization.unregister(this);
        return ptr;
      }
      free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wcontext_free(ptr, 0);
      }
      /**
       * @param {Conf} conf
       */
      constructor(conf) {
        _assertClass(conf, Conf2);
        var ptr0 = conf.__destroy_into_raw();
        const ret = wasm.load(ptr0);
        this.__wbg_ptr = ret >>> 0;
        WContextFinalization.register(this, this.__wbg_ptr, this);
        return this;
      }
      /**
       * @param {Uint8Array} s
       * @returns {string}
       */
      update(s) {
        let deferred1_0;
        let deferred1_1;
        try {
          const ret = wasm.wcontext_update(this.__wbg_ptr, s);
          deferred1_0 = ret[0];
          deferred1_1 = ret[1];
          return getStringFromWasm0(ret[0], ret[1]);
        } finally {
          wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
      }
    };
    module2.exports.WContext = WContext2;
    module2.exports.__wbg_buffer_6e1d53ff183194fc = function(arg0) {
      const ret = arg0.buffer;
      return ret;
    };
    module2.exports.__wbg_length_2e63ba34c4121df5 = function(arg0) {
      const ret = arg0.length;
      return ret;
    };
    module2.exports.__wbg_new_23362fa370a0a372 = function(arg0) {
      const ret = new Uint8Array(arg0);
      return ret;
    };
    module2.exports.__wbg_set_7b70226104a82921 = function(arg0, arg1, arg2) {
      arg0.set(arg1, arg2 >>> 0);
    };
    module2.exports.__wbindgen_init_externref_table = function() {
      const table = wasm.__wbindgen_export_0;
      const offset = table.grow(4);
      table.set(0, void 0);
      table.set(offset + 0, void 0);
      table.set(offset + 1, null);
      table.set(offset + 2, true);
      table.set(offset + 3, false);
      ;
    };
    module2.exports.__wbindgen_memory = function() {
      const ret = wasm.memory;
      return ret;
    };
    module2.exports.__wbindgen_throw = function(arg0, arg1) {
      throw new Error(getStringFromWasm0(arg0, arg1));
    };
    var path = require("path").join(__dirname, "wasm_pcps_bg.wasm");
    var bytes = require("fs").readFileSync(path);
    var wasmModule = new WebAssembly.Module(bytes);
    var wasmInstance = new WebAssembly.Instance(wasmModule, imports);
    wasm = wasmInstance.exports;
    module2.exports.__wasm = wasm;
    wasm.__wbindgen_start();
  }
});

// src/extension.ts
var extension_exports = {};
__export(extension_exports, {
  activate: () => activate,
  deactivate: () => deactivate
});
module.exports = __toCommonJS(extension_exports);

// src/pcapviewer.ts
var vscode = __toESM(require("vscode"));

// src/dispose.ts
function disposeAll(disposables) {
  while (disposables.length) {
    const item = disposables.pop();
    if (item) {
      item.dispose();
    }
  }
}
var Disposable = class {
  _isDisposed = false;
  _disposables = [];
  dispose() {
    if (this._isDisposed) {
      return;
    }
    this._isDisposed = true;
    disposeAll(this._disposables);
  }
  _register(value) {
    if (this._isDisposed) {
      value.dispose();
    } else {
      this._disposables.push(value);
    }
    return value;
  }
  get isDisposed() {
    return this._isDisposed;
  }
};

// src/pcapviewer.ts
var import_rshark = __toESM(require_wasm_pcps());
function getNonce() {
  let text = "";
  const possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  for (let i = 0; i < 32; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}
var DIST = "media";
var ENTRY = "app.js";
var createWebviewHtml = (context, webview, file) => {
  const scriptUri = webview.asWebviewUri(vscode.Uri.joinPath(context.extensionUri, DIST, file));
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
};
var PcapDocument = class _PcapDocument extends Disposable {
  static async create(uri, backupId) {
    const dataFile = typeof backupId === "string" ? vscode.Uri.parse(backupId) : uri;
    const fileData = await _PcapDocument.readFile(dataFile);
    return new _PcapDocument(uri, fileData);
  }
  static async readFile(uri) {
    if (uri.scheme === "untitled") {
      return new Uint8Array();
    }
    return new Uint8Array(await vscode.workspace.fs.readFile(uri));
  }
  _uri;
  _documentData;
  // client?: Client
  constructor(uri, initialContent) {
    super();
    this._uri = uri;
    this._documentData = initialContent;
  }
  get uri() {
    return this._uri;
  }
  get documentData() {
    return this._documentData;
  }
  dispose() {
    super.dispose();
  }
};
var PcapViewerProvider = class _PcapViewerProvider {
  constructor(_context) {
    this._context = _context;
  }
  // private static newPawDrawFileId = 1;
  static output = vscode.window.createOutputChannel("pcap console", { log: true });
  // private static pcapProvider: FrameProvider = new FrameProvider();
  get output() {
    return this.output;
  }
  static register(context) {
    return vscode.window.registerCustomEditorProvider(
      _PcapViewerProvider.viewType,
      new _PcapViewerProvider(context),
      {
        webviewOptions: {
          retainContextWhenHidden: true
        },
        supportsMultipleEditorsPerDocument: false
      }
    );
  }
  static viewType = "proto.pcapng";
  /**
   * Tracks all known webviews
   */
  webviews = new WebviewCollection();
  //#region CustomEditorProvider
  async openCustomDocument(uri, openContext, _token) {
    return PcapDocument.create(uri, openContext.backupId);
  }
  async resolveCustomEditor(document, webviewPanel, _token) {
    const config = import_rshark.Conf.new(true);
    const ins = await (0, import_rshark.load)(config);
    this.webviews.add(document.uri, webviewPanel);
    webviewPanel.title = "";
    webviewPanel.webview.options = {
      enableScripts: true
    };
    webviewPanel.webview.html = createWebviewHtml(this._context, webviewPanel.webview, ENTRY);
  }
  _requestId = 1;
  _callbacks = /* @__PURE__ */ new Map();
  postMessageWithResponse(panel, type, body) {
    const requestId = this._requestId++;
    const p = new Promise((resolve) => this._callbacks.set(requestId, resolve));
    panel.webview.postMessage({ type, requestId, body });
    return p;
  }
  postMessage(panel, type, body) {
    panel.webview.postMessage({ type, body });
  }
  onMessage(document, message) {
    switch (message.type) {
      case "response": {
        const callback = this._callbacks.get(message.requestId);
        callback?.(message.body);
        return;
      }
    }
  }
};
var WebviewCollection = class {
  _webviews = /* @__PURE__ */ new Set();
  /**
   * Get all known webviews for a given uri.
   */
  *get(uri) {
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
  add(uri, webviewPanel) {
    const entry = { resource: uri.toString(), webviewPanel };
    this._webviews.add(entry);
    webviewPanel.onDidDispose(() => {
      this._webviews.delete(entry);
    });
  }
};

// src/extension.ts
function activate(context) {
  context.subscriptions.push(PcapViewerProvider.register(context));
}
function deactivate() {
}
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {
  activate,
  deactivate
});
//# sourceMappingURL=extension.js.map
