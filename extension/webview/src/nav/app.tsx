import React, { MutableRefObject, useRef, useState } from 'react';
import { Menubar } from 'primereact/menubar';
import { Avatar } from 'primereact/avatar';
import { ComLog, ComMessage } from '../common';
import { PCAPClient } from '../client';
import init from 'rshark';

class Client extends PCAPClient {
    constructor(private ref: MutableRefObject<any>){
        super();
    }
    printLog(log: ComLog): void {
        console.log(log.level, log.msg);
    }
    emitMessage(msg: ComMessage<any>): void {
        this.ref.current.contentWindow.postMessage(msg, '*');
    }

}

init();
export default function CommandDemo() {
    const inputRef = useRef(null);
    const iframeRef = useRef(null);
    const [name, setName] = useState('');
    const [client, setClient] = useState<Client>(null);
    const onFileChangeCapture = (e: React.ChangeEvent<HTMLInputElement>) => {
        const files = e.target.files;
        if (files.length) {
            const {name} = files[0];
            setName(name);
            document.title = name;
            const reader = new FileReader();
            reader.onload = function () {
                const arrayBuffer: ArrayBuffer = this.result as ArrayBuffer;
                const array = new Uint8Array(arrayBuffer);
                const client = new Client(iframeRef);
                client.initData(array);
                setClient(client);
                client.ready = true;
                try {
                    client.init();
                } catch(e){

                }
                window.onmessage = function(e) {
                    client.handle(e.data);
                };
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
                    label: 'Select pcap',
                    icon: 'pi pi-plus',
                    command: () => {
                        inputRef.current.click();
                    }
                },
            ]
        },
    ];
    const end = (
        <div className="flex align-items-center gap-2" style={{paddingRight: '10px'}}>
            <Avatar icon="pi pi-github" shape="circle" onClick={() => {
                window.open('https://github.com/sankooc/vs-shark');
            }}/>
        </div>
    );
    return (
        <>
            <Menubar model={items} style={{ padding: 0 }} end={end}/>
            <input type="file" ref={inputRef} style={{ display: "none" }} onChangeCapture={onFileChangeCapture} />
            <iframe id="main" ref={iframeRef} src="frame.html" style={{ width: '100' }}></iframe>
            {/* <div className="footbar"></div> */}
        </>
    )
}