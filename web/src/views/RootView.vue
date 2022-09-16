<template>
    <div class="fixed top-0 left-0 h-full w-1/2 bg-white" aria-hidden="true"></div>
    <div class="fixed top-0 right-0 h-full w-1/2 bg-gray-50" aria-hidden="true"></div>

    <main class="relative flex min-h-full flex-col">
        <SiteHeader
            @openPanel="openPanel"
        />

        <div class="mx-auto w-full max-w-7xl flex-grow lg:flex xl:px-8">
            <div class="min-w-0 flex-1 bg-white xl:flex">
                <SideBar
                    :gurus="gurus"
                    @openPanel="openPanel"
                    @showGurus="showGurus"
                    @showSubnets="showSubnets"
                />

                <MyGurus v-if="isShowingGurus" :gurus="gurus" />
                <MySubnets v-if="isShowingSubnets" :subnets="subnets" />
            </div>

            <ActivityFeed />
        </div>
    </main>
</template>

<script>
/* Import components. */
import ActivityFeed from '@/components/ActivityFeed'
import MyGurus from '@/components/MyGurus'
import MySubnets from '@/components/MySubnets'
import SideBar from '@/components/SideBar'
import SiteHeader from '@/components/SiteHeader'
// import { v4 as uuidv4 } from 'uuid'

export default {
    components: {
        ActivityFeed,
        MyGurus,
        MySubnets,
        SideBar,
        SiteHeader,
    },
    data: () => ({
        gurus: null,
        subnets: null,

        isShowingOptionsMenu: null,
        isShowingGurus: null,
        isShowingSubnets: null,
    }),
    computed: {
        //
    },
    methods: {
        openPanel() {
            this.$emit('openPanel')
        },

        showGurus() {
            this.isShowingGurus = true
            this.isShowingSubnets = false
        },

        showSubnets() {
            this.isShowingGurus = false
            this.isShowingSubnets = true
        },

    },
    created: async function () {
        this.isShowingOptionsMenu = false

        /* Initialize gurus. */
        this.gurus = []
        this.isShowingGurus = true

        /* Initialize subnets. */
        this.subnets = []
        this.isShowingSubnets = false

        /* Load Gurus. */
        this.gurus.push({
            id: '02264e27-4d37-45b5-aac1-8a064315a26c',
            title: 'Fuji Tester',
        })

        this.gurus.push({
            id: 'b893b1dd-a7d6-4cb5-85b0-9034bccdd3ea',
            title: 'Production (Mainnet)',
        })

        this.gurus.push({
            id: 'b9ba5062-5519-4376-bdfe-d29569010392',
            title: 'Dev Runner',
        })

        /* Load Subnets. */
        this.subnets.push({
            id: 'f02d52e7-2318-46b7-a7b1-719a7e388195',
            title: 'Nexaverse',
        })

        this.subnets.push({
            id: '4b10eedd-fb27-4c63-8d6a-0bfddf7c65f0',
            title: 'Incognito',
        })

        this.subnets.push({
            id: '1ad9a6ab-f8aa-4e2b-8d6e-3d4834bff569',
            title: 'Real Money Gaming',
        })

    },
    mounted: function () {
        //
    },
}
</script>
