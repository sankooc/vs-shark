import React from "react";
import ReactECharts from 'echarts-for-react';
import { ICase } from "../../common";

class Proto {
    items: ICase[]
}
const BarChart = (props: Proto) => {
    const x = props.items.map((cc) => cc.name);
    const y = props.items.map((cc) => cc.value);
    const option = {
        title: {
            text: "IP Address Distribution",
            left: 'center',
            top: 'bottom',
        },
        grid: {
            padding: '20px',
            // top: '70',
            left: '3%',
            right: '3%',
            bottom: '12%',
            // borderColor: fore,
            containLabel: true
        },
        xAxis: {
            type: 'category',
            data: x
        },
        yAxis: {
            type: 'value'
        },
        legend: {
            top: '5%',
            left: 'center'
        },
        series: [
            {
                data: y,
                type: 'bar'
            }
        ]
    };
    return (<ReactECharts option={option} />
    );
};

export default BarChart;
