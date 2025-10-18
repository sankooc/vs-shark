
import { Image } from "@fluentui/react-components";



type Props = {
    raw: Uint8Array
    mime: string
}
export default function ImageView(props: Props){
    const blob = new Blob([props.raw], { type: props.mime });
    const imageUrl = URL.createObjectURL(blob);
    return <div className="flex h-full w-full justify-content-center align-content-center"><Image fit="center" src={imageUrl} /></div>
}