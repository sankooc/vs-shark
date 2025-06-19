import { PcapFile } from "../../share/common";
import { IProgressStatus } from "../../share/gen";



interface LoadingProps {
    info?: PcapFile;
    progress?: IProgressStatus;
}
function Component (props: LoadingProps) {
    if (!props.info && !props.progress) {
        return <>No Selected file</>
    }
    return <>Loading</>
}

export default Component;