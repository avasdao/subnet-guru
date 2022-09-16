<template>
    <main class="relative z-10" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">

        <BaseView />

        <SidePanel v-if="showPanel" />
    </main>

    <ModalView v-if="showModal" @closeModal="closeModal" />
</template>

<script>
/* Import home view. */
import BaseView from './views/BaseView'

/* Import components. */
import ModalView from '@/components/ModalView'
import SidePanel from '@/components/SidePanel'

export default {
    components: {
        BaseView,
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

    },
    created: function () {
        /* Initialize modal. */
        this.showModal = true

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
