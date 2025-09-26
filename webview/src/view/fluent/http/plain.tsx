

interface PlainProps {
    text: string
    mime: string    
}

const Component = (props: PlainProps) => {
    return <div style={{padding: '10px'}}>{props.text}</div>
}

export default Component;