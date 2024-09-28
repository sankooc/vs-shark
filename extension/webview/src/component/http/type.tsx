import React from "react";
import ReactECharts from 'echarts-for-react';
import bt from '../ui';

class Proto {
    items: any[]
    title?: string
    tooltip: string
}
const Pie = (props: Proto) => {

    const option = {
        title: {
            text: props.title,
            left: 'center'
          },
        tooltip: {
            trigger: 'item'
        },
        legend: {
            top: 'middle',
            left: 'right',
            orient: 'vertical',
            textStyle: { color: bt.fore },
        },
        width: '90%',
        series: [
            {
                name: props.tooltip,
                type: 'pie',
                radius: ['40%', '70%'],
                avoidLabelOverlap: false,
                padAngle: 5,
                itemStyle: {
                    borderRadius: 10
                },
                label: {
                    show: false,
                    position: 'center'
                },
                emphasis: {
                    label: {
                        show: true,
                        fontSize: 10,
                        fontWeight: 'bold'
                    }
                },
                labelLine: {
                    show: false
                },
                data: props.items
            }
        ]
    };
    return (<ReactECharts option={option}/>
    );
};

export default Pie;
