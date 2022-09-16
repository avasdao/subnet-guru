<template>
    <main class="relative z-10 h-screen sm:overflow-y-hidden" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">
        <RootView @openPanel="openPanel" />

        <SidePanel v-if="showPanel" @closePanel="closePanel" />
    </main>

    <ModalView v-if="showModal" @closeModal="closeModal" />
</template>

<script>
/* Import root view. */
import RootView from './views/RootView'

/* Import components. */
import ModalView from '@/components/ModalView'
import SidePanel from '@/components/SidePanel'

export default {
    components: {
        RootView,
        ModalView,
        SidePanel,
    },
    data: () => ({
        showModal: null,
        showPanel: null,
    }),
    computed: {
        //
    },
    methods: {
        closeModal() {
            this.showModal = false
        },

        openPanel() {
            this.showPanel = true
        },

        closePanel() {
            this.showPanel = false
        },

    },
    created: function () {
        /* Initialize modal. */
        this.showModal = false

        /* Initialize (side) panel. */
        this.showPanel = false
    },
    mounted: function () {
        //
    },
}
</script>

<style>
/*  */
</style>
