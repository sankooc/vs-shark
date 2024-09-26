import React from "react";
import ReactECharts from 'echarts-for-react';
import bt from '../ui';



class Proto {
    // items: any[]
}
const Pie = (props: Proto) => {

    const option = {
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
            
            labelLine: {
                show: false
            },
            label: {
                show: false,
                position: 'center'
            },
            name: 'Access From',
            type: 'pie',
            radius: ['40%', '70%'],
            center: ['50%', '70%'],
            // adjust the start and end angle
            startAngle: 180,
            endAngle: 360,
            data: [
              { value: 1048, name: 'Search Engine' },
              { value: 735, name: 'Direct' },
              { value: 580, name: 'Email' },
              { value: 484, name: 'Union Ads' },
              { value: 300, name: 'Video Ads' }
            ]
          }
        ]
      };
    return (<ReactECharts option={option}/>
    );
};

export default Pie;
