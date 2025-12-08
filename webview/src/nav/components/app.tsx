import React, { useRef, useState, useCallback } from "react";
import { IframeWithPlaceholder } from "./iframe";
import { useStore } from "../store";
import { PcapFile } from "../../share/common";
import Loading from "./loading";
import Header from './menu/MenuBar';

export interface HeaderProps {
    pFile: PcapFile | undefined;
    blocked: boolean;
    triggerNewFile: () => void;
    triggerReset: () => void;
}

export default function CommandDemo() {
    const loadIFrame = useStore((state) => state.loadIFrame);
    const [isLoading, setIsLoading] = useState<boolean>(true);
    const loadData = useStore((state) => state.loadData);
    const reset = useStore((state) => state.reset);
    const [pFile, setPFile] = useState<PcapFile | undefined>(undefined);
    const inputRef = useRef<HTMLInputElement>(null);
    const iframeRef = useRef<HTMLIFrameElement>(null);
    const [blocked, setBlocked] = useState<boolean>(false);
    const iframeSrc = 'app.html';
    loadIFrame(iframeRef.current);

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
                const pdata = { name, size };
                setPFile(pdata);
                // const fd: PcapFile = { name, size };
                // send(ComMessage.new(ComType.TOUCH_FILE, fd));
                loadData(pdata, array).then(() => {
                    setBlocked(false);
                });
            };
            reader.readAsArrayBuffer(files[0]);
        }
    };
    const handleIframeLoad = useCallback(() => {
        setIsLoading(false);
    }, []);
    const headerProps = {
        pFile,
        blocked,
        triggerNewFile: () => inputRef.current?.click(),
        triggerReset: () => {
            setPFile(undefined);
            if (inputRef.current) {
                inputRef.current.value = '';
            }
            reset();
        }
    }


    return (
        <div className="flex flex-column h-full">
            {isLoading ? null : <Header {...headerProps}/>}
            <input
                type="file"
                ref={inputRef}
                style={{ display: "none" }}
                onChangeCapture={onFileChangeCapture}
            />
            <IframeWithPlaceholder
                src={iframeSrc}
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
