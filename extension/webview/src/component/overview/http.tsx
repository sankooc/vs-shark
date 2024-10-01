import React from "react";
import { IStatistic } from "../../common";
import { Card } from 'primereact/card';
import TypePie from './type';
import BarChart from './bar';
class Props {
    data: IStatistic;
}
function Http(props: Props) {
    const statistic = props.data;
    return (<>
        <Card className="ip-statistic-card">
        {statistic.ip && <div className="ip"><BarChart items={statistic.ip} /></div>}
        {statistic.ip_type && <div className="iptype"><TypePie items={statistic.ip_type} title="IP Types" tooltip="IP Types" /></div>}
        </Card>
        <Card className="http-statistic-card">
            {statistic.http_method && <Card>
                <TypePie items={statistic.http_method} title="HTTP Method Usage" tooltip="http method" />
            </Card>}
            {statistic.http_status && <Card>
                <TypePie items={statistic.http_status} title="Web Traffic Response Code Analysis" tooltip="status code" />
            </Card>}
            {statistic.http_type && <Card>
                <TypePie items={statistic.http_type} title="Content-Type Distribution" tooltip="resp type" />
            </Card>}
        </Card>
    </>);
}

export default Http;