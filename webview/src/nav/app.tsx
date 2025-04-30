import React, { useEffect, useRef, useState } from 'react';
import { Menubar } from 'primereact/menubar';
import { Avatar } from 'primereact/avatar';
import { Sidebar } from 'primereact/sidebar';
import './app.scss';

// init();
export default function CommandDemo() {
    const inputRef = useRef(null);
    const iframeRef = useRef(null);

    const [name, setName] = useState('');
    const [visible, setVisible] = useState(false);
    
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
        add_comment();
        window.onmessage = function (e) {
            // if (e.data?.type === 'ready'){
            //     iframeRef.current.contentWindow.postMessage(new ComMessage('_status', -1), '*');
            // }
        };
      }, []);
    const onFileChangeCapture = (e: React.ChangeEvent<HTMLInputElement>) => {
       
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
                    }
                },
            ]
        },
    ];
    const _style= visible ? { backgroundColor: '#2196F3', color: '#ffffff' } : {};

    const getStyle = (v: boolean): any => {
        return v ? { backgroundColor: '#2196F3', color: '#ffffff' } : {};
    }
    const end = (
        <div className="flex align-items-center gap-2" style={{ paddingRight: '10px' }}>
            <Avatar icon="pi pi-comment" shape="circle" style={getStyle(visible)} onClick={() => {
                setVisible(!visible);
            }} />
            <Avatar icon="pi pi-github" shape="circle"  onClick={() => {
                window.open('https://github.com/sankooc/vs-shark');
            }} />
        </div>
    );
    return (
        <>
            <Menubar model={items} style={{ padding: '8px 0px' }} end={end} />
            <input type="file" ref={inputRef} style={{ display: "none" }} onChangeCapture={onFileChangeCapture} />
            <iframe id="main" ref={iframeRef} src="index.html" style={{ width: '100', display: visible? 'none': 'block'}}></iframe>
            <div id="comments" className="utterances" style={{width:"100%", overflow: "auto", height: "540px", display: visible? 'block': 'none'}}></div>
            <div className="footbar">
                <p>v0.3.10</p>
            </div>
        </>
    )
}