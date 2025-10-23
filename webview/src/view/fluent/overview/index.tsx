import FrameChart from './frame';
import DNSChart from './dns';
import HTTPChart from './http';

export default function Component() {
    return <div className="flex flex-column overview-page"> 
        <FrameChart />
        <HTTPChart />
        <DNSChart />
    </div>
}