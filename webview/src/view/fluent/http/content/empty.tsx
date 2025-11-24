import { MailInboxAllRegular } from "@fluentui/react-icons";


type EmptyProps = {
    content?: string;
}   

export default function Empty(props: EmptyProps) {
    const content = props.content || "No Content";
    return <div className="flex flex-column justify-content-center align-items-center h-full" >
        <MailInboxAllRegular style={{fontSize: "12em"}}/>
        <span>{content}</span>
    </div>;
}