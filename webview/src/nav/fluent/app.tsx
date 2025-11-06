import React, { useRef, useState, useCallback } from "react";
import { IframeWithPlaceholder } from "../components/IframeWithPlaceholder";
import { useStore } from "../store";
import { ComMessage, ComType, PcapFile } from "../../share/common";
import {
    Button,
    Toolbar,
    Text,
} from "@fluentui/react-components";
import { AddRegular, DeleteRegular } from "@fluentui/react-icons";


// import { getTheme } from '@fluentui/react';
class PFile {
    name!: string;
    size!: number;
}


export default function CommandDemo() {
    const loadIFrame = useStore((state) => state.loadIFrame);
    const [isLoading, setIsLoading] = useState<boolean>(true);
    const send = useStore((state) => state.send);
    const loadData = useStore((state) => state.loadData);
    const reset = useStore((state) => state.reset);
    const [pFile, setPFile] = useState<PFile | undefined>(undefined);
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
                setPFile({ name, size });
                const fd: PcapFile = { name, size, start: Date.now() };
                send(ComMessage.new(ComType.TOUCH_FILE, fd));
                loadData(array).then(() => {
                    setBlocked(false);
                });
            };
            reader.readAsArrayBuffer(files[0]);
        }
    };
    const handleIframeLoad = useCallback(() => {
        setIsLoading(false);
    }, []);
    // const options = [
    //     "DarkTheme",
    //     "LightTheme",
    // ];
    return (
        <>
            {isLoading? null: (<div style={{ padding: "5px", borderBottom: 'solid 1px #ddd' }} className="flex flex-row justify-content-between">
                <div>
                    <Toolbar aria-label="Default" size="small">
                        {pFile && <Text style={{ marginRight: "10px" }}>{pFile.name}</Text>}
                        {pFile?
                        <Button
                            disabled={blocked}
                            size="small"
                            onClick={() => {
                                setPFile(undefined);
                                if(inputRef.current){
                                    inputRef.current.value = '';
                                }
                                reset();}} icon={<DeleteRegular />}
                        >Reset</Button>: <Button
                            disabled={blocked}
                            size="small"
                            onClick={() => inputRef.current?.click()} icon={<AddRegular />}
                        >Select PCAP File</Button>}
                        
                    </Toolbar>
                </div>
            </div>)}
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
                    <div className="custom-placeholder">
                        <div className="placeholder-text">Loading App Resource</div>
                    </div>
                }
            />
        </>
    );
}
