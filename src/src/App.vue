<template>
  <div :class="darkMode ? 'bg-gray-900' : 'bg-gray-50'" class="transition ease-in-out min-h-screen flex flex-col">
    <header class="flex justify-between items-center px-9 pt-8">
        <InformationCircleIcon class="h-6 w-6 text-gray-600" />
        <div class="space-x-3 flex items-center">
            <LargeSwitch v-model="darkMode" @input="onDarkModeToggle"/>
            <MoonIcon class="h-6 w-6 text-gray-600" v-if="!darkMode" />
            <SolidMoonIcon v-else class="h-6 w-6 text-purple-600"/>
        </div>
    </header>
    <div class="px-9 pt-14 pb-3 flex flex-col items-end">
        <p class="text-lg font-semibold text-gray-600" :class="darkMode ? 'text-grey-300' : ''">10 + 10 =</p>
        <h3 class="text-5xl font-bold" :class="darkMode ? 'text-white': ''">20</h3>
    </div>
    <div class="rounded-t-3xl px-9 py-4 space-y-6 grow transition ease-in-out" :class="darkMode ? 'bg-gray-800' : 'bg-white'">
        <div class="grid grid-cols-4 gap-x-6">
            <BaseButton type="function" :dark="darkMode" :key="updateKey">ABS</BaseButton>
            <BaseButton type="function" :dark="darkMode" :key="updateKey">!</BaseButton>
            <BaseButton type="function" :dark="darkMode" :key="updateKey">xⁿ</BaseButton>
            <BaseButton type="function" :dark="darkMode" :key="updateKey">√x</BaseButton>
        </div>
        <div class="grid grid-cols-4 gap-x-6">
            <BaseButton type="function" :dark="darkMode" :key="updateKey">+</BaseButton>
            <BaseButton type="function" :dark="darkMode" :key="updateKey">-</BaseButton>
            <BaseButton type="function" :dark="darkMode" :key="updateKey">/</BaseButton>
            <BaseButton type="function" :dark="darkMode" :key="updateKey">*</BaseButton>
        </div>
        <div class="grid grid-cols-4 gap-x-6">
            <BaseButton :dark="darkMode" :key="updateKey">7</BaseButton>
            <BaseButton :dark="darkMode" :key="updateKey">8</BaseButton>
            <BaseButton :dark="darkMode" :key="updateKey">9</BaseButton>
            <BaseButton type="function" :dark="darkMode" :key="updateKey"><BackspaceIcon class="h-6 w-6"/></BaseButton>
        </div>
        <div class="grid grid-cols-4 gap-x-6">
            <BaseButton :dark="darkMode" :key="updateKey">4</BaseButton>
            <BaseButton :dark="darkMode" :key="updateKey">5</BaseButton>
            <BaseButton :dark="darkMode" :key="updateKey">6</BaseButton>
            <BaseButton type="function" :dark="darkMode" :key="updateKey">AC</BaseButton>
        </div>
        <div class="grid grid-cols-4 gap-x-6">
            <div class="col-span-2 flex flex-col gap-y-6">
                <div class="grid grid-cols-2 gap-x-6">
                    <BaseButton :dark="darkMode" :key="updateKey">1</BaseButton>
                    <BaseButton :dark="darkMode" :key="updateKey">2</BaseButton>
                </div>
                <BaseButton :dark="darkMode" :key="updateKey">0</BaseButton>
            </div>
            <div class="flex flex-col space-y-6">
                <BaseButton :dark="darkMode" :key="updateKey">3</BaseButton>
                <BaseButton :dark="darkMode" :key="updateKey">.</BaseButton>
            </div>
            <BaseButton type="filled">=</BaseButton>
        </div>
    </div>
  </div>
</template>

<script lang="ts">
import { InformationCircleIcon, MoonIcon, BackspaceIcon } from "@heroicons/vue/outline";
import { MoonIcon as SolidMoonIcon } from "@heroicons/vue/solid"
import { defineComponent, ref } from "vue";
import LargeSwitch from './components/LargeSwitch.vue'
import BaseButton from "./components/BaseButton.vue";

export default defineComponent({
    components: {
        BaseButton,
        InformationCircleIcon,
        MoonIcon,
        SolidMoonIcon,
        LargeSwitch,
        BackspaceIcon,
    },
    setup() {
        let darkMode = ref(false)
        let updateKey = ref(0)

        const onDarkModeToggle = () => {
            darkMode.value = !darkMode.value
            updateKey.value++
        }

        return {
            darkMode,
            onDarkModeToggle,
            updateKey,
        }
    }
})
</script>
