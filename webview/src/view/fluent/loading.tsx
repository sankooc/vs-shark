import { makeStyles } from "@fluentui/react-components";
import { PcapFile } from "../../share/common";
import { IProgressStatus } from "../../share/gen";

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
            return <div className={styles.root}>No Selected file</div>
        }
    }
    return <div className={styles.root}>Loading</div>
}

export default Component;