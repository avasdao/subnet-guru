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

/* Import modules. */
import { ethers } from 'ethers'

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
        async initWeb3() {
            /* Set Smartstarter Address. */
            const addr = '0xDbd91DD51A3152cB26f0b3AcaDE6E25910E71F10'

            /* Set Smartstarter ABI. */
            const abi = require('./contracts/SubnetGuru.json')

            /* Initialize provider. */
            const provider = new ethers.providers
                // .JsonRpcProvider('https://wispy-damp-meadow.avalanche-mainnet.discover.quiknode.pro/e47ea5d0f6aa7820a004234e9ef2a5156d260fe1')
                .JsonRpcProvider('https://api.avax.network/ext/bc/C/rpc')
            console.log('PROVIDER', provider)

            /* Initialize campaign instance. */
            const guruStore = new ethers.Contract(addr, abi, provider)
            console.log('CONTRACT (guruStore):', guruStore)

            /* Request smartstarter nickname. */
            const version = await guruStore
                .getVersion()
                .catch(err => {
                    console.error(err)

                    /* Handle invalid call. */
                    if (err.code === 'CALL_EXCEPTION') {
                        throw new Error('Failed to load (on-chain) Smartstarter contract.')
                    }
                })
            console.log('GURU STORE (version):', version, version.toNumber())
        },

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

        /* Initialize Web3. */
        this.initWeb3()
    },
    mounted: function () {
        //
    },
}
</script>

<style>
/*  */
</style>
