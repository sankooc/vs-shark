import { MailInboxAllRegular } from "@fluentui/react-icons";



export default function Empty() {
    return <div className="flex flex-column justify-content-center align-items-center h-full" >
        <MailInboxAllRegular style={{fontSize: "12em"}}/>
        <span>No Content</span>
    </div>;
}