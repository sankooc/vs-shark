import React, { useEffect } from "react";
import ReactECharts from 'echarts-for-react';
import { OverviewSource } from "../../common";
import { MainProto } from '../../wasm';
import { FrameInfo } from 'rshark';
import { TabView, TabPanel } from 'primereact/tabview';
import { Statc } from '../../client';
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

function convert(frames: FrameInfo[]) {
  const scale = 24;
  const start = frames[0].time;
  const end = frames[frames.length - 1].time;
  const duration = end - start;
  const per = Math.floor(duration / scale);
  const result: Statc[] = [];
  let cur = start;
  let limit = cur + per;
  let rs = Statc.create(start, per);
  const ps = new Set<string>();
  const getArray = (num: number): Statc => {
    if (num < limit) {
      return rs;
    }
    result.push(rs);
    rs = Statc.create(limit, per);
    limit = limit + per;
    return getArray(num);
  }
  let _total = 0;
  for (const item of frames) {
    const origin = item.len;
    _total += item.len;
    const it = getArray(item.time);
    it.size += origin;
    it.count += 1;
    const pname = item.protocol?.toLowerCase() || '';
    it.addLable(pname, item);
    ps.add(pname);
  }

  const categories = ['total'];
  const map: any = {
    total: []
  };
  ps.forEach((c) => {
    categories.push(c);
    map[c] = [];
  });
  const labels = [];
  const countlist = [];
  for (const rs of result) {
    const { size, count, stc, start } = rs;
    labels.push(start);
    countlist.push(count);
    map.total.push(size)
    ps.forEach((c) => {
      map[c].push(stc.get(c) || 0);
    });
  }
  const overview = new OverviewSource();
  overview.legends = categories;
  overview.labels = labels;
  overview.counts = countlist;
  overview.valMap = map;
  return overview;
}

// const fore = 'var(--vscode-list-focusForeground)';
const fore = '#ebdbb2';
const lineColor = '#ebdbb2';
function Overview(props: MainProto) {
  const title = 'Network Traffic';
  const { legends, labels, counts, valMap } = convert(props.instance.getFrames());
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
      lineStyle: {color: lineColor},
      data
    };
    if (key === 'total') {
      rs.areaStyle = {};
    }
    return rs;
  });
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