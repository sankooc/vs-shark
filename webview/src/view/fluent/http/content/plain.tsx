import { Highlight, themes } from "prism-react-renderer"

interface PlainProps {
    text: string
    mime: string    
}

export default function CodeBlock( props: PlainProps) {
  return <Highlight
    theme={themes.gruvboxMaterialDark}
    code={props.text}
    language={props.mime}
  >
    {({ className: _, style, tokens, getLineProps, getTokenProps }) => (
      <pre style={{...style, overflow: 'auto'}} className="flex-1">
        {tokens.map((line, i) => (
          <div key={i} {...getLineProps({ line })}>
            <span style={{display: 'inline-block', width: '25px'}}>{i + 1}</span>
            {line.map((token, key) => (
              <span key={key} {...getTokenProps({ token })} />
            ))}
          </div>
        ))}
      </pre>
    )}
  </Highlight>
}