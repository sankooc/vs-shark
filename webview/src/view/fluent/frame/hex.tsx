// import { TabView, TabPanel } from "primereact/tabview";
// import Hex from "./hex";
import "./hex.scss";
import { Cursor } from "../../../share/common";
import { Tab, TabList } from "@fluentui/react-components";
// import { useStore } from "../../../store";
// import { useEffect, useState } from "react";


const ALL_EXCEPT_PRINTABLE_LATIN = /[^\x20-\x7f]/g;
// const CONTROL_CHARACTERS_ONLY = /[\x00-\x1f]/g
const ascii_escape = function (str: string) {
    return str.replace(ALL_EXCEPT_PRINTABLE_LATIN, ".");
};
const to_string = (data: Uint8Array): string => {
    let text = "";
    data.forEach((ch) => (text += String.fromCharCode(ch)));
    return ascii_escape(text);
};

function Hex(props: { bin?: Uint8Array; highlight?: [number, number] }) {
    const indexes = [];
    const codes = [];
    let start = 0;
    let end = 0;
    const getActive = (inx: number): string => {
        if (end > 0 && inx >= start && inx < end) {
            return "active";
        }
        return "";
    };
    // const data = props.data;
    const data = { data: props.bin, index: props.highlight };
    let hasData = !!data?.data;
    const texts = [];
    if (data.data) {
        const lent = data.data!.length;
        if (data.index && data.index[1]) {
            start = data.index[0];
            end = start + data.index[1];
        }
        for (let i = 0; i < lent; i += 16) {
            const inx = `0x${i.toString(16).padStart(8, "0")}`;
            indexes.push(inx);
        }
        for (let i = 0; i < lent; i += 1) {
            codes.push(data.data![i].toString(16).padStart(2, "0"));
        }
        if (data.data && data.data.length) {
            const raw = data.data;
            let _indx = 0;
            while (_indx < raw.length) {
                const _fin = Math.min(_indx + 16, raw.length);
                if (end > start) {
                    const _start = Math.max(start, _indx);
                    const _end = Math.min(end, _fin);
                    if (start > _fin || end < _indx) {
                        texts.push(
                            <div className="asc" key={"pre-" + _indx}>
                                {to_string(raw.slice(_indx, _fin))}
                            </div>,
                        );
                    } else if (_start < _fin) {
                        texts.push(
                            <div className="asc" key={"pre-" + _indx}>
                                {to_string(raw.slice(_indx, _start))}
                                <span>{to_string(raw.slice(_start, _end))}</span>
                                {to_string(raw.slice(_end, _fin))}
                            </div>,
                        );
                    } else {
                        texts.push(
                            <div className="asc" key={"pre-" + _indx}>
                                {to_string(raw.slice(_indx, _fin))}
                            </div>,
                        );
                    }
                } else {
                    texts.push(
                        <div className="asc" key={"pre-" + _indx}>
                            {to_string(raw.slice(_indx, _fin))}
                        </div>,
                    );
                }
                _indx = _fin;
            }
        }
    }

    if (!hasData || !indexes.length) {
        return <div id="detail"></div>;
    }
    return (
        <div id="detail">
            <div className="index">
                {indexes.map((inx) => (
                    <pre key={"line" + inx}>{inx}</pre>
                ))}
            </div>
            <div className="hex">
                {codes.map((code, inx) => (
                    <code key={"code" + inx} className={getActive(inx)}>
                        {code}
                    </code>
                ))}
            </div>
            <div className="text">{texts}</div>
        </div>
    );
}


interface Props {
    cursor: Cursor | undefined;
}

function Component(props: Props) {
    // let hasSelected = false;
    // let select = new Uint8Array();

    let selected: [number, number] | undefined = undefined;
    const scope = props.cursor?.scope;
    // let fetch = "";
    if (scope) {
        // const size = scope.end - scope.start;
        // fetch = `${scope.start}-${size}`;
        const inx = props.cursor?.selected;
        if (inx) {
            const start = Math.max(inx.start - scope.start, 0);
            selected = [start, inx.size];
        }
    }

    let bin = props.cursor?.data;
    if (!props.cursor || !bin) {
        return <div style={{ padding: "10px" }}> No Data </div>;
    }

    // if (selected) {
    //     hasSelected = true;
    //     select = bin.slice(selected[0], selected[0] + selected[1]);
    // }
    return <div className="h-full flex flex-column">
        <TabList defaultSelectedValue="tab1">
            <Tab value="tab1">Frame</Tab>
            <Tab value="tab2">Select</Tab>
        </TabList>
        <div className="flex-1 flex-grow-1">
            <Hex bin={bin} highlight={selected} />
        </div>
    </div>
    //   return (
    //     <TabView
    //       className="w-full h-full flex flex-column detail-tab"
    //       style={{ padding: 0 }}
    //     >
    //       <TabPanel header="Frame" style={{ padding: 0 }}>
    //         <Hex bin={bin} highlight={selected} />
    //       </TabPanel>
    //       {hasSelected && (
    //         <TabPanel header="Selected">
    //           <Hex bin={select} highlight={[0, 0]} />
    //         </TabPanel>
    //       )}
    //     </TabView>
    //   );
}

export default Component;
