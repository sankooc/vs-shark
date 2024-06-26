import React from "react";
import ReactECharts from 'echarts-for-react';
import { Grap } from "../common";
class ARPProps {
  graph: Grap;
  legends: string[];
}
function ARPReplies(props: ARPProps) {
  const { legends, graph } = props;
  const option = {
    legend: {
      data: legends
    },
    series: [{
      type: 'graph',
      layout: 'force',
      animation: true,
      symbolSize: 30,
      roam: true,
      label: {
        show: true,
      },
      edgeSymbol: ['arrow'],
      data: graph.nodes,
      categories: graph.categories,
      force: {
        repulsion: 800,
      },
      edges: graph.links
    }]
  };
  return <div style={{ padding: '8px' }}><ReactECharts option={option} style={{ height: '400px', width: '100%' }} /></div>;
}

export default ARPReplies;