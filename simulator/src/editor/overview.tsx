import React, { useEffect, useState } from "react";
import ReactECharts from 'echarts-for-react';
import { OverviewSource } from "../common";

const bytesFormater = (v) => {
  let h = 0;
  let l = 0;
  let pt = 'kB';
  const mod = 1024;
  const cop = (num: number): [number, number] => {
    return [Math.floor(v / mod), v % mod]
  }
  [h, l] = cop(v);
  if(h < mod){
    return `${h}.${l}kB`
  }
  [h, l] = cop(v);
  if(h < mod){
    return `${h}.${l}mB`
  }
  [h, l] = cop(v);
  return `${h}.${l}gB`
};

class OverviewProps {
  data: OverviewSource
}
function Overview(props: OverviewProps) {
  const title = 'Network Traffic by Protocol Over Time';

  const {legends, labels, counts, valMap} = props.data;


  const labelFormater = (v) => {
    const ts = Math.floor(v);
    const date = new Date(ts);
    const [minutes, seconds, ms ] = [
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
    if(key === 'total'){
      rs.areaStyle = {};
    }
    console.log(rs);
    return rs;
  });
  const option = {
    title: {
      text: title
    },
    legend: {
      data: legends
    },
    tooltip: {
      trigger: 'axis',
      // valueFormatter : bytesFormater,
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
    animationDuration: 3000,
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
      {
        yAxisIndex: 0,
        type: 'line',
        smooth: true,
        data: counts,
        label: {
          show: true,
          position: 'top'
        },
      },
      ...datas
    ]
  };
  const style = { height: '400px', width: '100%', padding: '10px', border: '1px solid #222' };
  return <ReactECharts option={option} style={style} />;
}

export default Overview;