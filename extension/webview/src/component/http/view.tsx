import React, { useState } from "react";
import { IHttpEnity } from "../../common";

class Proto {
    message: IHttpEnity
}
enum ContentType {
    Raw,
}

const Content = (props: Proto) => {

    const [type, setType] = useState<ContentType>(ContentType.Raw);
    const [txt, setTxt] = useState<string>('');

    const {content, header} = props.message;

    switch (type) {
        case ContentType.Raw: {
            new Promise(r => {
                const reader = new FileReader()
                reader.onload = () => r(reader.result)
                reader.readAsDataURL(new Blob([content]))
              }).then((_rs) => {
                const str = _rs as string;
                setTxt(str.slice(str.indexOf(',') + 1));
              });
            return <div className="http-content"><span className="base64-content">{txt}</span></div>;
        }
    }

    return <div />;
};

export default Content;
