<template>
    <!-- Navbar -->
    <main class="flex-shrink-0 bg-indigo-600">
        <div class="mx-auto max-w-7xl px-2 sm:px-4 lg:px-8">
            <div class="relative flex h-16 items-center justify-between">
                <!-- Logo section -->
                <div class="flex items-center px-2 lg:px-0 xl:w-64">
                    <div class="flex-shrink-0">
                        <img class="h-12 w-auto" :src="require('../assets/logo.png')" alt="Subnet Guru" />
                    </div>
                </div>

                <!-- Search section -->
                <div class="flex flex-1 justify-center lg:justify-end">
                    <div class="w-full px-2 lg:px-6">
                        <label for="search" class="sr-only">
                            Search subnets
                        </label>

                        <div class="relative text-indigo-200 focus-within:text-gray-400">
                            <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
                                <!-- Heroicon name: mini/magnifying-glass -->
                                <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                    <path fill-rule="evenodd" d="M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z" clip-rule="evenodd" />
                                </svg>
                            </div>

                            <input
                                id="search"
                                name="search"
                                class="block w-full rounded-md border border-transparent bg-indigo-400 bg-opacity-25 py-2 pl-10 pr-3 leading-5 text-base text-indigo-50 placeholder-indigo-200 focus:bg-white focus:text-gray-900 focus:placeholder-gray-400 focus:outline-none focus:ring-0"
                                placeholder="Search subnets, validators and nodes"
                                type="search"
                            />
                        </div>
                    </div>
                </div>

                <div class="flex lg:hidden">
                    <!-- Mobile menu button -->
                    <button
                        type="button"
                        class="inline-flex items-center justify-center rounded-md bg-indigo-600 p-2 text-indigo-400 hover:bg-indigo-600 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-indigo-600"
                        aria-controls="mobile-menu"
                        aria-expanded="false"
                    >
                        <span class="sr-only">Open main menu</span>
                        <!--
              Icon when menu is closed.

              Heroicon name: outline/bars-3-center-left

              Menu open: "hidden", Menu closed: "block"
            -->
                        <svg class="block h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12H12m-8.25 5.25h16.5" />
                        </svg>
                        <!--
              Icon when menu is open.

              Heroicon name: outline/x-mark

              Menu open: "block", Menu closed: "hidden"
            -->
                        <svg class="hidden h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>
                <!-- Links section -->
                <div class="hidden lg:block lg:w-80">
                    <div class="flex items-center justify-end">
                        <div class="flex">
                            <a href="https://subnet.builders/my-subnet" target="_blank" class="rounded-md px-3 py-2 text-base font-medium text-indigo-50 hover:text-white" aria-current="page">
                                Docs
                            </a>

                            <button
                                @click="signin"
                                type="button"
                                class="rounded-md px-3 py-2 text-base font-medium text-indigo-50 hover:text-white"
                            >
                                Disconnect
                            </button>
                        </div>

                        <!-- Profile dropdown -->
                        <div class="relative ml-4 flex-shrink-0">
                            <div>
                                <button
                                    type="button"
                                    class="flex rounded-full bg-indigo-700 text-sm text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-indigo-700"
                                    id="user-menu-button"
                                    aria-expanded="false"
                                    aria-haspopup="true"
                                >
                                    <span class="sr-only">Open user menu</span>
                                    <img class="h-8 w-8 rounded-full" src="https://images.unsplash.com/photo-1517365830460-955ce3ccd263?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=256&h=256&q=80" alt="" />
                                </button>
                            </div>

                            <!--
                Dropdown menu, show/hide based on menu state.

                Entering: "transition ease-out duration-100"
                  From: "transform opacity-0 scale-95"
                  To: "transform opacity-100 scale-100"
                Leaving: "transition ease-in duration-75"
                  From: "transform opacity-100 scale-100"
                  To: "transform opacity-0 scale-95"
              -->
                            <div
                                class="hidden absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
                                role="menu"
                                aria-orientation="vertical"
                                aria-labelledby="user-menu-button"
                                tabindex="-1"
                            >
                                <!-- Active: "bg-gray-100", Not Active: "" -->
                                <a href="javascript://" class="block px-4 py-2 text-sm text-gray-700" role="menuitem" tabindex="-1" id="user-menu-item-0">Your Profile</a>

                                <a href="javascript://" class="block px-4 py-2 text-sm text-gray-700" role="menuitem" tabindex="-1" id="user-menu-item-1">Settings</a>

                                <a href="javascript://" class="block px-4 py-2 text-sm text-gray-700" role="menuitem" tabindex="-1" id="user-menu-item-2">Sign out</a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Mobile menu, show/hide based on menu state. -->
        <div class="hidden lg:hidden" id="mobile-menu">
            <div class="space-y-1 px-2 pt-2 pb-3">
                <!-- Current: "text-white bg-indigo-800", Default: "text-indigo-200 hover:text-indigo-100 hover:bg-indigo-600" -->
                <a href="javascript://" class="text-white bg-indigo-800 block px-3 py-2 rounded-md text-base font-medium" aria-current="page">Dashboard</a>

                <a href="javascript://" class="text-indigo-200 hover:text-indigo-100 hover:bg-indigo-600 block px-3 py-2 rounded-md text-base font-medium">Domains</a>
            </div>

            <div class="border-t border-indigo-800 pt-4 pb-3">
                <div class="space-y-1 px-2">
                    <a href="javascript://" class="block rounded-md px-3 py-2 text-base font-medium text-indigo-200 hover:bg-indigo-600 hover:text-indigo-100">Your Profile</a>

                    <a href="javascript://" class="block rounded-md px-3 py-2 text-base font-medium text-indigo-200 hover:bg-indigo-600 hover:text-indigo-100">Settings</a>

                    <a href="javascript://" class="block rounded-md px-3 py-2 text-base font-medium text-indigo-200 hover:bg-indigo-600 hover:text-indigo-100">Sign out</a>
                </div>
            </div>
        </div>
    </main>
</template>

<script>
/* global ethereum */

/* Import modules. */
import { ethers } from 'ethers'
import moment from 'moment'

export default {
    data: () => ({
        //
    }),
    computed: {
        //
    },
    methods: {
        async signin() {
            /* Validate email address. */
            // if (!this.validateEmail()) return

            /* Validate embedded Web3 objects. */
            if (!window.ethereum) {
                /* Validate embedded ethereum object. */
                if (window.ethereum) {
                    console.info('Found Ethereum provider.')
                } else {
                    return console.error('No Web3 provider found.')
                }
            }

            /* Connect accounts. */
            const accounts = await ethereum.request({
                method: 'eth_requestAccounts'
            })
            // console.info('Connected Web3 accounts:', accounts)

            if (!accounts || accounts.length < 1) {
                return alert('Please connect your MetaMask account to continue.')
            }

            this.accounts = accounts
            console.log('ACCOUNTS', accounts)

            /* Initialize provider. */
            const provider = new ethers
                .providers
                .Web3Provider(window.ethereum, 'any')

            /* Set signer. */
            const signer = provider.getSigner()
            // console.log('SIGNER', signer)

            /* Set message name. */
            const name = 'Subnet Guru'

            /* Set message version. */
            const version = require('../../package.json').version

            /* Set (connected) chain id. */
            const chainId = await signer.getChainId()
            // console.log('CHAIN ID', chainId);

            /* Set verifying contract. */
            // FIXME
            const verifyingContract = '0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC'

            /* Initialize the message domain. */
            const domain = {
                name,
                version,
                chainId,
                verifyingContract,
            }
            // console.log('DOMAIN', domain)

            /* Set the named list of all type definitions. */
            const types = {
                Profile: [
                    { name: 'account', type: 'address' },
                    { name: 'address', type: 'string' },
                    { name: 'email', type: 'string' },
                    { name: 'phone', type: 'string' },
                    { name: 'notes', type: 'string' },
                ],
                Notif: [
                    { name: 'profile', type: 'Profile' },
                    { name: 'action', type: 'string' },
                    { name: 'created', type: 'string' },
                    { name: 'expires', type: 'string' },
                ],
            }

            const profile = {
                account: this.accounts[0] || '0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC',
                address: this.address || 'n/a',
                email: this.email || 'n/a',
                phone: this.phone || 'n/a',
                notes: this.notes || 'n/a',
            }
            // console.log('PROFILE', profile)

            const action = 'signin_auth'
            const created = moment().format('lll ZZ')
            const expires = moment().add(7, 'days').format('lll ZZ')

            const notif = {
                profile,
                action,
                created,
                expires,
            }
            // console.log('NOTIF', notif)

            /* Create authentication package. */
            const auth = { ...notif }
            console.log('AUTH PACKAGE', auth)

            const signature = await signer
                ._signTypedData(domain, types, auth)
            console.log('SIGNATURE', signature)

            // const rawResponse = await fetch('http://127.0.0.1:3000/v1/sessions', {
            //     method: 'POST',
            //     headers: {
            //         'Accept': 'application/json',
            //         'Content-Type': 'application/json'
            //     },
            //     body: JSON.stringify({
            //         auth,
            //         signature,
            //     })
            // })

            // const content = await rawResponse.json()
            // console.log(content)
        },

    },
    created: function () {
        //
    },
    mounted: function () {
        //
    },
}
</script>
