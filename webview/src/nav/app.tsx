import React, { useEffect, useRef, useState, useCallback } from "react";
import { Menubar } from "primereact/menubar";
import { Avatar } from "primereact/avatar";
import { IframeWithPlaceholder } from "./components/IframeWithPlaceholder";
import "./app.scss";
import { useStore } from "./store";
import { ComMessage, ComType, PcapFile } from "../core/common";

export default function CommandDemo() {
  const loadIFrame = useStore((state) => state.loadIFrame);
  const send = useStore((state) => state.send);
  // const loadFile = useStore((state) => state.loadFile);
  // const unloadFile = useStore(state => state.unloadFile);
  const loadData = useStore((state) => state.loadData);

  const inputRef = useRef<HTMLInputElement>(null);
  const iframeRef = useRef<HTMLIFrameElement>(null);
  const [blocked, setBlocked] = useState<boolean>(false);
  const [visible, setVisible] = useState(false);
  const [iframeSrc, _setIframeSrc] = useState<string>("index.html");
  const [isLoaded, setIsLoading] = useState(true);
  loadIFrame(iframeRef.current);
  // console.log(iframeRef.current);
  function add_comment() {
    let script = document.createElement("script");
    let anchor = document.getElementById("comments");
    if (!anchor) return;
    script.setAttribute("src", "https://utteranc.es/client.js");
    script.setAttribute("crossorigin", "anonymous");
    script.setAttribute("async", "true");
    script.setAttribute("repo", "sankooc/comments");
    script.setAttribute("issue-term", "pathname");
    script.setAttribute("theme", "github-dark");
    anchor.appendChild(script);
  }
  useEffect(() => {
    add_comment();
  }, []);

  // let globalClient: Client;
  const onFileChangeCapture = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files && files.length) {
      setBlocked(true);
      const name = files[0].name;
      document.title = name;
      const reader = new FileReader();
      reader.onload = function () {
        const arrayBuffer: ArrayBuffer = this.result as ArrayBuffer;
        const array = new Uint8Array(arrayBuffer);
        const size = array.length;
        const fd: PcapFile = { name, size, start: Date.now() };
        send(ComMessage.new(ComType.TOUCH_FILE, fd));
        loadData(array).then(() => {
          setBlocked(false);
        });
        // if (globalClient) {
        //   // globalClient.distroy();
        // }
        // const client = new Client(iframeRef);
        // client.data = array;
        // client.ready = true;
        // try {
        //   client.init();
        //   client.update(client.data);
        //   globalClient = client;
        //   window.onmessage = function (e) {
        //     client.handle(e.data);
        //   };
        // } catch (e) {
        //   console.error(e);
        //   // client.destory();
        // }
      };
      reader.readAsArrayBuffer(files[0]);
    }
  };
  const disabled = isLoaded || blocked;
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
            if (blocked) {
              return;
            }
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
