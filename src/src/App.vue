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
      <div class="items-end px-9 pt-14 pb-3">
        <input
          ref="mathInput"
          v-model="currentNum"
          placeholder="0"
          disabled
          class="disabled: w-full appearance-none truncate bg-transparent text-right text-3xl font-bold text-gray-900 focus:outline-none dark:text-white"
        />
      </div>
      <div
        class="flex h-full flex-col space-y-6 rounded-t-3xl bg-white px-7 pt-5 pb-8 transition ease-in-out dark:bg-gray-800"
      >
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton type="function" @click="onFunctionInput('abs')">ABS</BaseButton>
          <BaseButton type="function" @click="onFunctionInput('factorial')">!</BaseButton>
          <BaseButton type="function" @click="onFunctionInput('pow')">xⁿ</BaseButton>
          <BaseButton type="function" @click="onFunctionInput('root')">√x</BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton type="function" @click="onFunctionInput('add')">+</BaseButton>
          <BaseButton type="function" @click="onFunctionInput('subtract')">-</BaseButton>
          <BaseButton type="function" @click="onFunctionInput('divide')">/</BaseButton>
          <BaseButton type="function" @click="onFunctionInput('multiply')">*</BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton @click="parseNumber('7')">7</BaseButton>
          <BaseButton @click="parseNumber('8')">8</BaseButton>
          <BaseButton @click="parseNumber('9')">9</BaseButton>
          <BaseButton type="function" @click="onBackspace()"><BackspaceIcon class="h-6 w-6" /></BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton @click="parseNumber('4')">4</BaseButton>
          <BaseButton @click="parseNumber('5')">5</BaseButton>
          <BaseButton @click="parseNumber('6')">6</BaseButton>
          <BaseButton type="function" @click="onEraseAll()">AC</BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <div class="col-span-2 flex flex-col gap-y-6">
            <div class="grid grow grid-cols-2 gap-x-6">
              <BaseButton @click="parseNumber('1')">1</BaseButton>
              <BaseButton @click="parseNumber('2')">2</BaseButton>
            </div>
            <div class="grid grow">
              <BaseButton @click="parseNumber('0')">0</BaseButton>
            </div>
          </div>
          <div class="flex grow flex-col space-y-6">
            <BaseButton @click="parseNumber('3')">3</BaseButton>
            <BaseButton @click="parseNumber('.')">.</BaseButton>
          </div>
          <BaseButton type="filled" @click="onEqualSign('=')">=</BaseButton>
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
import { invoke } from '@tauri-apps/api/tauri'

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

    let currentNum = ref('')
    let previousNum = ref('')
    let functiond = ref('')
    let calcError = ref(false)
    let isResult = ref(false)

    const onDarkModeToggle = () => {
      darkMode.value = !darkMode.value
    }

    const errorCheck = () => {
      if (calcError.value) {
        currentNum.value = ''
        calcError.value = false
      }
    }

    // onNumInput
    const parseNumber = (num: string) => {
      errorCheck()

      if (isResult.value) {
        // When commented out, using result works as previous num
        // previousNum.value = currentNum.value
        currentNum.value = num
        isResult.value = false
        return
      }

      if (num === '.') {
        if (!currentNum.value.includes('.')) {
          if (!currentNum.value) currentNum.value = '0'
          currentNum.value += '.'
        }
        return
      }

      if (currentNum.value === '0') currentNum.value = ''

      currentNum.value += num
    }

    const onEraseAll = () => {
      currentNum.value = ''
      previousNum.value = ''
      functiond.value = ''
      calcError.value = false
    }

    const onBackspace = () => {
      if (calcError.value || isResult.value) onEraseAll()

      currentNum.value = currentNum.value.slice(0, -1)
    }

    const onFunctionInput = (func: string) => {
      errorCheck()

      // If current number is not set
      if (!currentNum.value) currentNum.value = '0'

      // If function is already set
      if (functiond.value && previousNum.value) {
        if (func === 'factorial' || func === 'abs') {
          currentNum.value = ''
        }

        onEqualSign()
        functiond.value = func
        return
      }

      previousNum.value = currentNum.value
      currentNum.value = ''

      functiond.value = func
    }

    const onEqualSign = (arg?: string) => {
      if (!functiond.value || !previousNum.value) return
      console.log(functiond.value)

      invoke('math_operation', {
        payload: {
          a: previousNum.value,
          b: currentNum.value == '' ? undefined : currentNum.value,
          operation: functiond.value,
        },
      })
        .then((result) => {
          currentNum.value = result as string
          previousNum.value = ''
          isResult.value = true

          if (arg === '=') {
            functiond.value = ''
          }
        })
        .catch((error) => {
          calcError.value = true
          currentNum.value = error
          functiond.value = ''
        })
    }

    const parseInput = (e: KeyboardEvent) => {
      console.log(e.key)
      switch (e.key) {
        case '1':
          parseNumber('1')
          break
        case '2':
          parseNumber('2')
          break
        case '3':
          parseNumber('3')
          break
        case '4':
          parseNumber('4')
          break
        case '5':
          parseNumber('5')
          break
        case '6':
          parseNumber('6')
          break
        case '7':
          parseNumber('7')
          break
        case '8':
          parseNumber('8')
          break
        case '9':
          parseNumber('9')
          break
        case '.':
          parseNumber('.')
          break
        case '+':
          onFunctionInput('add')
          break
        case '-':
          onFunctionInput('subtract')
          break
        case '*':
          onFunctionInput('multiply')
          break
        case '/':
          onFunctionInput('divide')
          break
        case 'a':
          onFunctionInput('abs')
          break
        case 'f':
          onFunctionInput('factorial')
          break
        case 'p':
          onFunctionInput('pow')
          break
        case 'r':
          onFunctionInput('root')
          break
        case 'c':
          onEraseAll()
          break
        case '=':
          onEqualSign()
          break
        case 'Backspace':
          onBackspace()
          break
      }
    }

    onMounted(() => {
      document.addEventListener('keydown', parseInput)
    })

    onUnmounted(() => {
      document.removeEventListener('keydown', parseInput)
    })

    return {
      darkMode,
      onDarkModeToggle,
      mathInput,
      currentNum,
      parseNumber,
      onFunctionInput,
      onEqualSign,
      onEraseAll,
      onBackspace,
    }
  },
})
</script>
