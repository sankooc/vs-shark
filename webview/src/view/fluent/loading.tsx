import { makeStyles } from "@fluentui/react-components";
import { PcapFile } from "../../share/common";
import { IProgressStatus } from "../../share/gen";
import Empty from "./http/content/empty";

const useCSS = makeStyles({
    root: {
        padding: "10px",
    }
})

interface LoadingProps {
    info?: PcapFile;
    progress?: IProgressStatus;
}
function Component(props: LoadingProps) {
    const styles = useCSS();
    if (!props.progress) {
        if (window["acquireVsCodeApi"]) {
            return <div className={styles.root}>Loading</div>
        } else {
            return <Empty content="Please choose a PCAP file to get started."/>
        }
    }
    return <div className={styles.root}>Loading</div>
}

export default Component;