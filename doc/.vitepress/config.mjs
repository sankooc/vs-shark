import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "PcapViewer",
  description: "Pcap/Pcapng analyzer",
  base: "/vs-shark/",
  head: [['link', { rel: 'icon', href: './favicon.ico' }]],
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Docs', link: '/pages/overview' },
      { text: 'Demo', link: 'https://sankooc.github.io/vs-shark/app/' },
    ],

    sidebar: {
      '/pages/': [
        {
          text: '📓 Document',
          items: [
            { text: 'Overview', link: '/pages/overview' },
            { text: 'Getting Started', link: '/pages/getting-started' },
          ]
        },
        {
          text: '🚧 Protocol Status',
          items: [
            { text: 'ICMP', link: '/specs/icmp' },
            { text: 'IEEE802/11', link: '/specs/ieee' },
          ]
        },
        {
          text: '🚥 Specs',
          items: [
            { text: 'Wifi', link: '/pages/link_127.md' },
          ]
        },
        {
          text: '⭐️ RoadMap',
          items: [
            { text: 'Plan', link: '/pages/roadmap' },
          ]
        },
        {
          text: '📊 Benchmark',
          items: [
            { text: '0.3.11', link: '/pages/benchmark_0.3.1' },
          ]
        }
      ]
    },
    footer: {
      message: 'Released under the <a href="https://github.com/sankooc/vs-shark/blob/master/LICENSE">MIT License</a>.',
      copyright: 'Copyright © 2024-present <a href="https://github.com/sankooc">Sankooc</a>'
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/sankooc/vs-shark' }
    ]
  }
})
