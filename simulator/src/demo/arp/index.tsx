import React from "react";
import ReactECharts from 'echarts-for-react';
import { Grap } from "../../common";
import './index.css';
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
        repulsion: 1000,
      },
      edges: graph.links.map((edge: any) => {
        edge.color = '#E2DFD0';
        edge.width = 3;
        return edge;
      })
    }]
  };
  return <div className="w-full h-full">
    <ReactECharts className="arp-panel" option={option}/>
    </div>;
}

export default ARPReplies;