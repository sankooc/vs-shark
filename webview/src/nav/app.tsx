import React, { useRef } from 'react';
import { Menubar } from 'primereact/menubar';
import { ComMessage } from '../common';

export default function CommandDemo() {
    const inputRef = useRef(null);
    const iframeRef = useRef(null);
    const onFileChangeCapture = (e: React.ChangeEvent<HTMLInputElement>) => {
        const files = e.target.files;
        if (files.length) {
            const reader = new FileReader();
            reader.onload = function () {
                const arrayBuffer: ArrayBuffer = this.result as ArrayBuffer;
                const array = new Uint8Array(arrayBuffer);
                iframeRef.current.contentWindow.postMessage(new ComMessage<Uint8Array>('raw-data', array), '*');
            };
            reader.readAsArrayBuffer(files[0]);
        }
    };
    const items = [
        {
            label: 'File',
            icon: 'pi pi-file',
            items: [
                {
                    label: 'Load PCAP',
                    icon: 'pi pi-plus',
                    command: () => {
                        inputRef.current.click();
                    }
                },
            ]
        },
    ];

    return (
        <>
            <Menubar model={items} style={{ padding: 0 }} />
            <input type="file" ref={inputRef} style={{ display: "none" }} onChangeCapture={onFileChangeCapture} />
            <iframe id="main" ref={iframeRef} src="frame.html" style={{ width: '100' }}></iframe>
        </>
    )
}