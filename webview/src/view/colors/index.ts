import tinycolor from "tinycolor2";

const fontCollect = ['#001219', '#f5f5f5', '#fbf1c7', '#1d2021', '#a34343', '#ffe5cf'];

const createPair = (backgroundColor: string): any => {
    const color = tinycolor.mostReadable(backgroundColor, fontCollect).toHexString();
    return {
        backgroundColor, 
        color
     };
}

const createFrameStyle = (backgroundColor: string): any => {
    const hov = tinycolor(backgroundColor).darken().toString()
    const main = createPair(backgroundColor);
    const hover = createPair(hov);
    
    return {
        ...main,
        cursor: 'pointer',
        ':hover': {
            ...hover
        },
        '&[aria-selected=true]': {
            ...hover,
        }
    }
}

export const frameColor = {
    tcp: createFrameStyle('#458588'),
    udp: createFrameStyle('#d79921'),
    http: createFrameStyle('#98971a'),
    arp: createFrameStyle('#b8bb26'),
    tls: createFrameStyle('#689d6a'),
    ssdp: createFrameStyle('#739588'),
    dns: createFrameStyle('#c0d6e8'),
    icmp: createFrameStyle('#604cc3'),
    icmp6: createFrameStyle('#604cc3'),
    pppoes: createFrameStyle('#779988'),
    pppoed: createFrameStyle('#669988'),
}