import { makeStyles, ToolbarButton, ToolbarRadioButton, ToolbarRadioGroup } from "@fluentui/react-components";


import {
  Toolbar
} from "@fluentui/react-components";
import { NextIcon, PrevIcon } from "./common";

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
  pageNum: {
    paddingLeft: 0,
    paddingRight: 0,
    minWidth: '30px'
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

function Component(props: PageProps) {
  const styles = useCSS();
  const hasTotal = props.total >= 0;
  if (hasTotal) {
    const max = Math.ceil(props.total / props.pageSize);
    let start = Math.max(1, props.page - 2);
    let end = Math.min(max, props.page + 2);
    if (start === 1) {
      end = Math.min(max, start + 4);
    } else if (end === max) {
      start = Math.max(1, end - 4);
    }
    const pages = [];
    for (let i = start; i <= end; i++) {
      pages.push(i);
    }
    return <Toolbar
      checkedValues={{pageNum: [props.page + '']}}
      style={{ justifyContent: "right", alignSelf: "flex-end" }}
    >
      <ToolbarButton
        icon={<PrevIcon />}
        name="textOptions"
        value="bold"
        disabled={props.page <= 1}
        onClick={() => { props.onPageChange(props.page - 1) }}
      />
      <ToolbarRadioGroup>
        {
          pages.map((page) => (
            <ToolbarRadioButton appearance="transparent"  className={styles.pageNum} key={'pjk'+page} name="pageNum" value={page + ''} onClick={() => { props.onPageChange(page) }} >
              {page}
            </ToolbarRadioButton>
          ))
        }
      </ToolbarRadioGroup>
      <ToolbarButton
        disabled={props.page >= max}
        icon={<NextIcon />}
        name="textOptions"
        value="underline"
        onClick={() => { props.onPageChange(props.page + 1) }}
      />
    </Toolbar>

  } else {
    return <Toolbar style={{ justifyContent: "right" }} >
      {props.page > 1 && <ToolbarButton
        icon={<PrevIcon />}
        name="textOptions"
        value="bold"
        onClick={() => { props.onPageChange(props.page - 1) }}
      />}
      <ToolbarButton
        icon={<NextIcon />}
        name="textOptions"
        value="underline"
        onClick={() => { props.onPageChange(props.page + 1) }}
      />
    </Toolbar>
  }
}

export default Component;