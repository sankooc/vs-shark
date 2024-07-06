import React, { useEffect, useState } from "react";
import ReactECharts from 'echarts-for-react';
import { OverviewSource } from "../../common";
import { TabView, TabPanel } from 'primereact/tabview';
import './index.css';

const bytesFormater = (v) => {
  let h = 0;
  let l = 0;
  let pt = 'kB';
  const mod = 1024;
  const cop = (num: number): [number, number] => {
    return [Math.floor(num / mod), num % mod]
  }
  [h, l] = cop(v);
  if (h < mod) {
    return `${h}.${l}kB`
  }
  [h, l] = cop(h);
  if (h < mod) {
    return `${h}.${l}mB`
  }
  [h, l] = cop(h);
  return `${h}.${l}gB`
};

class OverviewProps {
  data: OverviewSource
}
function Overview(props: OverviewProps) {
  const title = 'Network Traffic';

  const { legends, labels, counts, valMap } = props.data;
  console.log('---')

  const labelFormater = (v) => {
    const ts = Math.floor(v);
    const date = new Date(ts);
    const [minutes, seconds, ms] = [
      date.getMinutes(),
      date.getSeconds(),
      date.getMilliseconds()
    ];
    return `${minutes}:${seconds}:${ms}`
  };



  const keys = Object.keys(valMap);
  const datas = keys.map((key) => {
    const data = valMap[key];
    const rs: any = {
      name: key,
      yAxisIndex: 1,
      smooth: true,
      type: 'line',
      data
    };
    if (key === 'total') {
      rs.areaStyle = {};
    }
    return rs;
  });
  const option = {
    title: {
      text: title
    },
    legend: {
      right: 50,
      data: legends

    },
    tooltip: {
      trigger: 'axis',
      valueFormatter: bytesFormater,
      axisPointer: {
        type: 'cross',
        label: {
          backgroundColor: '#6a7985'
        }
      }
    },
    toolbox: {
      feature: {
        saveAsImage: {}
      }
    },
    grid: {
      left: '1%',
      right: '2%',
      bottom: '3%',
      containLabel: true
    },
    animation: true,
    animationDuration: 1400,
    xAxis: [
      {
        type: 'category',
        boundaryGap: false,
        axisLabel: {
          formatter: labelFormater
        },
        data: labels
      }
    ],
    yAxis: [
      {
        type: 'value',
        name: '',
        position: 'right',
        alignTicks: true,
        axisLine: {
          show: true,
        },
        axisLabel: {
          formatter: '{value}'
        }
      },
      {
        type: 'value',
        name: '',
        position: 'left',
        alignTicks: true,
        axisLine: {
          show: true,
        },
        axisLabel: {
          formatter: bytesFormater
        }
      }
    ],
    series: [
      ...datas
    ]
  };
  return (<TabView className="w-full">
    <TabPanel header="Connection">
      <ReactECharts option={option} className="overview" />
    </TabPanel>
    <TabPanel header="DNS">
    </TabPanel>
    <TabPanel header="TLS">
    </TabPanel>
  </TabView>);
}

export default Overview;