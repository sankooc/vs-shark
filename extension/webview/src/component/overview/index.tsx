import React, { useEffect, useState } from "react";
import ReactECharts from 'echarts-for-react';
import {emitMessage, onMessage} from '../../connect';
import { TabView, TabPanel } from 'primereact/tabview';
import './index.css';
import { ComMessage, IOverviewData } from "../../common";
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

// const fore = 'var(--vscode-list-focusForeground)';
const fore = '#ebdbb2';
const lineColor = '#ebdbb2';

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
function Overview() {
  const title = 'Network Traffic';
  const [{legends, labels, datas}, setData] = useState<IOverviewData>({legends:[], labels: [],datas:[] });
  const mountHook = () => {
    const remv = onMessage('message', (e: any) => {
      const { type, body, requestId } = e.data;
      switch (type) {
        case '_overview': {
          setData(body);
          break;
        }
      }
    });
    emitMessage(new ComMessage('overview', null));
    return remv;
  };
  useEffect(mountHook, []);
  const option = {
    title: {
      textStyle: {color: fore },
      text: title
    },
    textStyle: {color: fore},
    legend: {
      right: 50,
      textStyle: {color: fore },
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
      borderColor: fore,
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
        // nameTextStyle: {color: 'white'},
        data: labels,
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
        // nameTextStyle: {color: 'white'},
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