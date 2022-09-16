const { description } = require('../../package')

module.exports = {
  /**
   * Ref：https://v1.vuepress.vuejs.org/config/#title
   */
  title: 'Subnet Guru Docs',
  /**
   * Ref：https://v1.vuepress.vuejs.org/config/#description
   */
  description: description,

  /**
   * Extra tags to be injected to the page HTML `<head>`
   *
   * ref：https://v1.vuepress.vuejs.org/config/#head
   */
  head: [
    ['meta', { name: 'theme-color', content: '#3eaf7c' }],
    ['meta', { name: 'apple-mobile-web-app-capable', content: 'yes' }],
    ['meta', { name: 'apple-mobile-web-app-status-bar-style', content: 'black' }]
  ],

  /**
   * Theme configuration, here is the default theme configuration for VuePress.
   *
   * ref：https://v1.vuepress.vuejs.org/theme/default-theme-config.html
   */
  themeConfig: {
    repo: '',
    editLinks: false,
    docsDir: '',
    editLinkText: '',
    lastUpdated: false,
    nav: [
      {
        text: 'For Builders',
        link: '/builders/',
      },
      {
        text: 'For Operators',
        link: '/nodes/'
      },
      {
        text: 'My Gurus',
        link: 'https://subnet.guru'
      }
    ],
    sidebar: {
      '/builders/': [
        {
          title: 'Builders Guide',
          collapsable: false,
          children: [
            '',
            'FAQ',
            'upgrades',
          ]
        }
      ],
      '/nodes/': [
        {
          title: 'Node Operators Guide',
          collapsable: false,
          children: [
            '',
            'FAQ',
            'monitoring',
            'security',
          ]
        }
      ],
    }
  },

  /**
   * Apply plugins，ref：https://v1.vuepress.vuejs.org/zh/plugin/
   */
  plugins: [
    '@vuepress/plugin-back-to-top',
    '@vuepress/plugin-medium-zoom',
  ]
}
