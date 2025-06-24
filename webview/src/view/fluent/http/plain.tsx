

interface PlainProps {
    text: string
    mime: string    
}

const Component = (props: PlainProps) => {
    return <div>{props.text}</div>
}

export default Component;