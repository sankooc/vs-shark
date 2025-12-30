import FrameChart from './frame';
import DNSChart from './dns';
import HTTPChart from './http';
import IPChart from './ip';

export default function Component() {
    return <div className="flex flex-column intern gap-1"> 
        <FrameChart />
        <HTTPChart />
        <IPChart />
        <DNSChart />
    </div>
}