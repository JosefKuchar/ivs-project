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
          class="w-full appearance-none bg-transparent text-right text-lg font-semibold text-gray-600 focus:text-purple-600 focus:outline-none dark:text-gray-300 dark:focus:text-purple-400"
        />
        <h3 class="text-5xl font-bold dark:text-white">20</h3>
      </div>
      <div
        class="flex h-full flex-col space-y-6 rounded-t-3xl bg-white px-7 pt-5 pb-8 transition ease-in-out dark:bg-gray-800"
      >
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton type="function" @click="inputValue += '|| '">ABS</BaseButton>
          <BaseButton type="function" @click="inputValue += '!'">!</BaseButton>
          <BaseButton type="function" @click="inputValue += '^'">xⁿ</BaseButton>
          <BaseButton type="function" @click="inputValue += 'sqrt() '">√x</BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton type="function" @click="inputValue += ' + '">+</BaseButton>
          <BaseButton type="function" @click="inputValue += ' - '">-</BaseButton>
          <BaseButton type="function" @click="inputValue += ' / '">/</BaseButton>
          <BaseButton type="function" @click="inputValue += ' * '">*</BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton @click="inputValue += '7'">7</BaseButton>
          <BaseButton @click="inputValue += '8'">8</BaseButton>
          <BaseButton @click="inputValue += '9'">9</BaseButton>
          <BaseButton type="function" @click="inputValue = inputValue.slice(0, -1)"
            ><BackspaceIcon class="h-6 w-6"
          /></BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton @click="inputValue += '4'">4</BaseButton>
          <BaseButton @click="inputValue += '5'">5</BaseButton>
          <BaseButton @click="inputValue += '6'">6</BaseButton>
          <BaseButton type="function" @click="inputValue = ''">AC</BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <div class="col-span-2 flex flex-col gap-y-6">
            <div class="grid grow grid-cols-2 gap-x-6">
              <BaseButton @click="inputValue += '1'">1</BaseButton>
              <BaseButton @click="inputValue += '2'">2</BaseButton>
            </div>
            <div class="grid grow">
              <BaseButton @click="inputValue += '0'">0</BaseButton>
            </div>
          </div>
          <div class="flex grow flex-col space-y-6">
            <BaseButton @click="inputValue += '3'">3</BaseButton>
            <BaseButton @click="inputValue += '.'">.</BaseButton>
          </div>
          <BaseButton type="filled" @click="inputValue += ' ='">=</BaseButton>
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
