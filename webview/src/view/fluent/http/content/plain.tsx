import { Button, makeStyles, tokens, Tooltip } from "@fluentui/react-components";
import { CopyRegular } from "@fluentui/react-icons";
import { Highlight, themes } from "prism-react-renderer"

interface PlainProps {
  text: string
  mime: string
}
const useStyles = makeStyles({
  codeaction: {
    position: 'absolute',
    top: '2px',
    right: '20px',
    zIndex: 1,
  },
  codearea: {
    height: '100%',
    overflow: 'auto', 
    margin: '0',
  },
  codeindex: {
    paddingLeft: '4px',
    width: '25px',
    display: 'inline-block',
    color: tokens.colorNeutralStencil1,
  }
});
export default function CodeBlock(props: PlainProps) {
  const cus = useStyles();
  async function copyText(text: string) {
    try {
      await navigator.clipboard.writeText(text);
    } catch (err) {
      console.error('复制失败:', err);
    }
  }
  return <>
    <Tooltip content="Copy" relationship="label">
      <Button size="small" icon={<CopyRegular />} className={cus.codeaction} onClick={() => copyText(props.text)} />
    </Tooltip>
    <Highlight
      theme={themes.gruvboxMaterialDark}
      code={props.text}
      language={props.mime}
    >
      {({ className: _, style, tokens, getLineProps, getTokenProps }) => (
        <pre style={{ ...style }} className={cus.codearea + " flex-1"}>
          {tokens.map((line, i) => (
            <div key={i} {...getLineProps({ line })}>
              <span className={cus.codeindex}>{i + 1}</span>
              {line.map((token, key) => (
                <span key={key} {...getTokenProps({ token })} />
              ))}
            </div>
          ))}
        </pre>
      )}
    </Highlight></>
}