import React from "react";
import ReactECharts from 'echarts-for-react';
import { ILines } from "../../common";

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

class Props {
    data: ILines
}
function Overview(props: Props) {
  const title = 'Network Traffic';
  const { x, y, data } = props.data;
  const datas = data.map(d => {
    const rs = {
      ...d,
      yAxisIndex: 1,
      smooth: true,
      type: "line",
      areaStyle: null,
    };
    if (d.name == "total"){
      rs.areaStyle = {};
    }
    return rs;
  })
  const option = {
    title: {
      textStyle: { color: fore },
      text: title
    },
    textStyle: { color: fore },
    legend: {
      right: 50,
      textStyle: { color: fore },
      data: y
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
        // saveAsImage: {}
      }
    },
    grid: {
      padding: '20px',
      // top: '70',
      left: '3%',
      right: '3%',
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
        // axisLabel: {
        //   formatter: labelFormater
        // },
        data: x,
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
  return (<ReactECharts option={option} className="overview-frames"/>);
}

export default Overview;