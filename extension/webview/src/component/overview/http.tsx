import React from "react";
import { IStatistic } from "../../common";
import { Card } from 'primereact/card';
import TypePie from './type';

class Props {
    data: IStatistic;
}
function Http(props: Props) {
    const statistic = props.data;
    return (<Card className="http-statistic-card">
        <Card>
            <TypePie items={statistic.http_method} title="HTTP Method Usage" tooltip="http method" />
        </Card>
        <Card>
            <TypePie items={statistic.http_status} title="Web Traffic Response Code Analysis" tooltip="status code" />
        </Card>
        <Card>
            <TypePie items={statistic.http_type} title="Content-Type Distribution" tooltip="resp type" />
        </Card>
    </Card>);
}

export default Http;