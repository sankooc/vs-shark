import FrameChart from './frame';
import DNSChart from './dns';
import HTTPChart from './http';

export default function Component() {
    return <> 
        <FrameChart />
        <HTTPChart />
        <DNSChart />
    </>
}