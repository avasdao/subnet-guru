<template>
    <!--
      This example requires updating your template:

      ```
      <html class="h-full">
      <body class="h-full">
      ```
    -->
    <!-- Background color split screen for large screens -->
    <div class="fixed top-0 left-0 h-full w-1/2 bg-white" aria-hidden="true"></div>
    <div class="fixed top-0 right-0 h-full w-1/2 bg-gray-50" aria-hidden="true"></div>

    <main class="relative flex min-h-full flex-col">
        <SiteHeader />

        <!-- 3 column wrapper -->
        <div class="mx-auto w-full max-w-7xl flex-grow lg:flex xl:px-8">
            <!-- Left sidebar & main wrapper -->
            <div class="min-w-0 flex-1 bg-white xl:flex">
                <SideBar :gurus="gurus" />

                <MyGurus :gurus="gurus" />
            </div>

            <ActivityFeed />
        </div>
    </main>
</template>

<script>
/* Import components. */
import ActivityFeed from '@/components/ActivityFeed'
import MyGurus from '@/components/MyGurus'
import SideBar from '@/components/SideBar'
import SiteHeader from '@/components/SiteHeader'
// import { v4 as uuidv4 } from 'uuid'

export default {
    components: {
        ActivityFeed,
        MyGurus,
        SideBar,
        SiteHeader,
    },
    data: () => ({
        showOptionsMenu: null,
        gurus: null,
    }),
    computed: {
        //
    },
    methods: {
        //
    },
    created: async function () {
        this.showOptionsMenu = false

        this.gurus = []

        this.gurus.push({
            id: 1,
        })

        this.gurus.push({
            id: 2,
        })

        this.gurus.push({
            id: 3,
        })
    },
    mounted: function () {
        //
    },
}
</script>
