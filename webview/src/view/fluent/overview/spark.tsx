
import ReactECharts from 'echarts-for-react';


type SparkProps = {
  data: number[][],
}

export default function Component(props: SparkProps) {
  const data = props.data || [];
  const option = {
    animation: true,
    grid: { left: 2, right: 2, top: 2, bottom: 2, containLabel: false },
    xAxis: {
      type: 'time',
      show: false,
      min: 'dataMin',
      max: 'dataMax'
    },
    yAxis: {
      type: 'value',
      show: false,
      min: 'dataMin',
      max: 'dataMax'
    },
    tooltip: { show: false},
    series: [{
      type: 'line',
      showSymbol: false,
      smooth: true,
      lineStyle: { width: 1 },
      areaStyle: { opacity: 0.08 },
      data
    }]
  };
  return <ReactECharts option={option} style={{ height: '25px', width: '60px' }} />
}