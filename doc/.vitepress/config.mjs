import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "PcapViewer",
  description: "Pcap/Pcapng analyzer",
  base: "/vs-shark/",
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
          text: 'Document',
          items: [
            { text: 'Overview', link: '/pages/overview' },
            { text: 'Quick Start', link: '/pages/getting-started' },
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
