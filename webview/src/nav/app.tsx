import React, {
  useEffect,
  useRef,
  useState,
  useCallback
} from "react";
import { Menubar } from "primereact/menubar";
import { Avatar } from "primereact/avatar";
import init from "rshark";
import { IframeWithPlaceholder } from "./components/IframeWithPlaceholder";
import "./app.scss";
import { ComLog, ComMessage } from "../core/common";
import { PCAPClient } from "../core/client";

class Client extends PCAPClient {
  data!: Uint8Array;
  constructor(private ref: React.RefObject<HTMLIFrameElement | null>) {
    super();
  }
  printLog(log: ComLog): void {
    console.log(log.level, log.msg);
  }
  emitMessage(msg: ComMessage<any>): void {
    this.ref.current?.contentWindow?.postMessage(msg, "*");
  }
}

const ready = init();
export default function CommandDemo() {
  const inputRef = useRef<HTMLInputElement>(null);
  const iframeRef = useRef<HTMLIFrameElement>(null);
  const [visible, setVisible] = useState(false);
  const [iframeSrc, _setIframeSrc] = useState<string>("index.html");
  const [isLoaded, setIsLoading] = useState(true);
  const [name, setName] = useState<string>("");

  function add_comment() {
    let script = document.createElement('script');
    let anchor = document.getElementById('comments');
    if (!anchor) return;
    script.setAttribute('src', 'https://utteranc.es/client.js');
    script.setAttribute('crossorigin', 'anonymous');
    script.setAttribute('async', 'true');
    script.setAttribute('repo', 'sankooc/comments');
    script.setAttribute('issue-term', 'pathname');
    script.setAttribute('theme', 'github-dark');
    anchor.appendChild(script);
  }
  useEffect(() => {
    ready.then((rs) => {
      console.log("wasm loaded", rs);
    });
    add_comment();
  }, []);

  let globalClient: Client;
  const onFileChangeCapture = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files && files.length) {
      iframeRef.current?.contentWindow?.postMessage(
        new ComMessage("_reset", ""),
        "*",
      );
      if (files[0].name) {
        setName(files[0].name);
      }
      document.title = name;
      const reader = new FileReader();
      reader.onload = function () {
        const arrayBuffer: ArrayBuffer = this.result as ArrayBuffer;
        const array = new Uint8Array(arrayBuffer);
        if (globalClient) {
          // globalClient.distroy();
        }
        const client = new Client(iframeRef);
        client.data = array;
        client.ready = true;
        try {
          client.init();
          client.update(client.data);
          globalClient = client;
          window.onmessage = function (e) {
            client.handle(e.data);
          };
        } catch (e) {
          console.error(e);
          // client.destory();
        }
      };
      reader.readAsArrayBuffer(files[0]);
    }
  };
  const disabled = isLoaded;
  const items = [
    {
      label: "File",
      icon: "pi pi-file",
      items: [
        {
          label: "Select pcap",
          icon: "pi pi-plus",
          disabled,
          command: () => {
            inputRef.current?.click();
          },
        },
      ],
    },
  ];

  const getStyle = (v: boolean): any => {
    return v ? { backgroundColor: "#2196F3", color: "#ffffff" } : {};
  };
  const end = (
    <div
      className="flex align-items-center gap-2"
      style={{ paddingRight: "10px" }}
    >
      <Avatar
        icon="pi pi-comment"
        shape="circle"
        style={getStyle(visible)}
        onClick={() => {
          setVisible(!visible);
        }}
      />
      <Avatar
        icon="pi pi-github"
        shape="circle"
        onClick={() => {
          window.open("https://github.com/sankooc/vs-shark");
        }}
      />
    </div>
  );
  const handleIframeLoad = useCallback(() => {
    setIsLoading(false);
  }, []);

  // const handleMessage = useCallback((event: MessageEvent) => {
  //     const { data } = event;
  //     // 处理消息
  //     console.log('Received message:', data);
  // }, []);

  return (
    <>
      <Menubar model={items} style={{ padding: "8px 0px" }} end={end} />
      <input
        type="file"
        ref={inputRef}
        style={{ display: "none" }}
        onChangeCapture={onFileChangeCapture}
      />
      <IframeWithPlaceholder
        src={iframeSrc}
        className="main-iframe"
        frameref={iframeRef}
        onLoad={handleIframeLoad}
        placeholderContent={
          <div className="custom-placeholder">
            <div className="placeholder-text">Loading App</div>
          </div>
        }
      />
      <div
        id="comments"
        className="utterances"
        style={{
          width: "100%",
          overflow: "auto",
          height: "540px",
          display: visible ? "block" : "none",
        }}
      ></div>
      <div className="footbar">
        <p>v0.3.10</p>
      </div>
    </>
  );
}
