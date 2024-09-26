import React from "react";
import ReactECharts from 'echarts-for-react';
import bt from '../ui';



class Proto {
}
const Pie = (props: Proto) => {

    const option = {
        
        legend: {
            top: 'middle',
            left: 'right',
            orient: 'vertical',
            textStyle: { color: bt.fore },
        },
        width: '90%',
        series: [
            {
                name: 'Nightingale Chart',
                type: 'pie',
                radius: [20, 100],
                center: ['50%', '50%'],
                roseType: 'area',
                itemStyle: {
                    borderRadius: 8
                },
                
                labelLine: {
                    show: false
                },
                label: {
                    show: false,
                    position: 'center'
                },
                data: [
                    { value: 40, name: 'rose 1' },
                    { value: 38, name: 'rose 2' },
                ]
            }
        ]
    };
    return (<ReactECharts option={option} />
    );
};

export default Pie;
