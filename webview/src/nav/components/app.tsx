import React, { useRef, useCallback, useState } from "react";
import { IframeWithPlaceholder } from "./iframe";
import { useStore } from "../store";
import { PcapFile } from "../../share/common";
import Loading from "./loading";
// import Header from './menu/MenuBar';

export interface HeaderProps {
    pFile: PcapFile | undefined;
    blocked: boolean;
    triggerNewFile: () => void;
    triggerReset: () => void;
}

export interface NavProps {
    entry: string
}

export default function CommandDemo(props: NavProps) {
    const bind = useStore((state) => state.bindElement);
    const loadData = useStore((state) => state.loadData);
    const inputRef = useRef<HTMLInputElement>(null);
    const iframeRef = useRef<HTMLIFrameElement>(null);
    const [loaded, setLoaded] = useState<boolean>(false);
    if(loaded){
        bind(iframeRef.current || undefined, inputRef.current || undefined);
    }
    const onFileChangeCapture = (e: React.ChangeEvent<HTMLInputElement>) => {
        const files = e.target.files;
        if (files && files.length) {
            const name = files[0].name;
            document.title = name;
            const reader = new FileReader();
            reader.onload = function () {
                const arrayBuffer: ArrayBuffer = this.result as ArrayBuffer;
                const array = new Uint8Array(arrayBuffer);
                const size = array.length;
                const pdata = { name, size };
                loadData(pdata, array);
            };
            reader.readAsArrayBuffer(files[0]);
        }
    };
    const handleIframeLoad = useCallback(() => {
        setLoaded(true)
    }, []);


    return (
        <div className="flex flex-column h-full">
            <input
                type="file"
                ref={inputRef}
                style={{ display: "none" }}
                onChangeCapture={onFileChangeCapture}
            />
            <IframeWithPlaceholder
                src={props.entry}
                className="main-iframe flex-grow-1"
                frameref={iframeRef}
                onLoad={handleIframeLoad}
                placeholderContent={
                    <Loading />
                }
            />
        </div>
    );
}
