import { Button, makeStyles } from "@fluentui/react-components";
import { bundleIcon, TriangleLeft20Filled, TriangleLeft20Regular, TriangleRight20Filled, TriangleRight20Regular } from "@fluentui/react-icons";

interface PageProps {
    page: number;
    total: number;
    pageSize: number;
    onPageChange: (page: number) => void
}

const useCSS = makeStyles({
    pagnation: {
        textAlign: "right",
        overflow: "hidden",
        padding: "5px 10px",
        fontSize: "0.5em",
        flexShrink: 0,
    },
    icon: {
        minWidth: "1.7em",
        padding: "0",
        border: "none",
    },
    iconSelect: {
        minWidth: "1.7em",
        backgroundColor: "#344",
        padding: "0",
    }
});

const NextIcon = bundleIcon(TriangleRight20Filled, TriangleRight20Regular);
const PrevIcon = bundleIcon(TriangleLeft20Filled, TriangleLeft20Regular);
function Component(props: PageProps) {
    const styles = useCSS();
    const hasTotal = props.total >= 0;

    if (hasTotal) {
        const max = Math.ceil(props.total / props.pageSize);
        const start = Math.max(1, props.page - 2);
        const end = Math.min(max, props.page + 2);

        const pages = [];
        for (let i = start; i <= end; i++) {
            pages.push(i);
        }
        return <div className={styles.pagnation}>
            {props.page > 1 && <Button appearance="transparent" onClick={() => { props.onPageChange(props.page - 1) }} className={styles.icon} icon={<PrevIcon />}> </Button>}
            {pages.map((page) => (<Button key={page} shape="circular" onClick={() => { props.onPageChange(page) }} className={page == props.page ? styles.iconSelect : styles.icon}>{page}</Button>))}
            {props.page < max && <Button appearance="transparent" onClick={() => { props.onPageChange(props.page + 1) }} className={styles.icon} icon={<NextIcon />}> </Button>}
        </div>

    } else {
        return <div className={styles.pagnation}>
            {props.page > 1 && <Button appearance="transparent" className={styles.icon} icon={<PrevIcon />}> </Button>}
            <Button appearance="transparent" className={styles.icon} icon={<NextIcon />}> </Button>
        </div>
    }
}

export default Component;