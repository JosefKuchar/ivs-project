<template>
  <div :class="darkMode ? 'dark' : ''" class="min-h-screen">
    <div class="flex h-screen flex-col bg-gray-50 transition ease-in-out dark:bg-gray-900">
      <header class="flex items-center justify-between px-9 pt-8">
        <InformationCircleIcon class="h-6 w-6 text-gray-600" />
        <div class="flex items-center space-x-3">
          <LargeSwitch v-model="darkMode" @input="onDarkModeToggle" />
          <MoonIcon v-if="!darkMode" class="h-6 w-6 text-gray-600" />
          <SolidMoonIcon v-else class="h-6 w-6 text-purple-600" />
        </div>
      </header>
      <div class="flex flex-col items-end px-9 pt-14 pb-3">
        <input
          ref="mathInput"
          v-model="inputValue"
          autofocus
          class="w-full appearance-none bg-transparent text-right text-lg font-semibold text-gray-600 focus:text-purple-600 focus:outline-none dark:text-gray-300 dark:focus:text-purple-400"
        />
        <h3 class="text-5xl font-bold dark:text-white">20</h3>
      </div>
      <div
        class="flex h-full flex-col space-y-6 rounded-t-3xl bg-white px-7 pt-5 pb-8 transition ease-in-out dark:bg-gray-800"
      >
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton type="function">ABS</BaseButton>
          <BaseButton type="function">!</BaseButton>
          <BaseButton type="function">xⁿ</BaseButton>
          <BaseButton type="function">√x</BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton type="function">+</BaseButton>
          <BaseButton type="function">-</BaseButton>
          <BaseButton type="function">/</BaseButton>
          <BaseButton type="function">*</BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton>7</BaseButton>
          <BaseButton>8</BaseButton>
          <BaseButton>9</BaseButton>
          <BaseButton type="function"><BackspaceIcon class="h-6 w-6" /></BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton>4</BaseButton>
          <BaseButton>5</BaseButton>
          <BaseButton>6</BaseButton>
          <BaseButton type="function">AC</BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <div class="col-span-2 flex flex-col gap-y-6">
            <div class="grid grow grid-cols-2 gap-x-6">
              <BaseButton>1</BaseButton>
              <BaseButton>2</BaseButton>
            </div>
            <div class="grid grow">
              <BaseButton>0</BaseButton>
            </div>
          </div>
          <div class="flex grow flex-col space-y-6">
            <BaseButton>3</BaseButton>
            <BaseButton>.</BaseButton>
          </div>
          <BaseButton type="filled">=</BaseButton>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { InformationCircleIcon, MoonIcon, BackspaceIcon } from '@heroicons/vue/outline'
import { MoonIcon as SolidMoonIcon } from '@heroicons/vue/solid'
import { defineComponent, ref, onMounted, onUnmounted } from 'vue'
import LargeSwitch from './components/LargeSwitch.vue'
import BaseButton from './components/BaseButton.vue'

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
    let mathInput = ref<InstanceType<typeof HTMLInputElement>>()
    let inputValue = ref('')

    const onDarkModeToggle = () => {
      darkMode.value = !darkMode.value
    }

    const focusInput = () => {
      if (mathInput.value) mathInput.value.focus()
    }

    onMounted(() => {
      document.addEventListener('keydown', focusInput)
    })

    onUnmounted(() => {
      document.removeEventListener('keydown', focusInput)
    })

    return {
      darkMode,
      onDarkModeToggle,
      focusInput,
      mathInput,
      inputValue,
    }
  },
})
</script>
