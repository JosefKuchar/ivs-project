<template>
  <div :class="darkMode ? 'dark' : ''" class="relative min-h-screen">
    <div class="flex h-screen flex-col bg-gray-50 transition ease-in-out dark:bg-gray-900">
      <header class="flex items-center justify-between pl-7 pr-9 pt-8">
        <div
          class="ease hover:dark:bg-gray-70 cursor-pointer rounded-full p-2 transition hover:bg-gray-200"
          :class="infoActive ? 'bg-gray-200 dark:bg-gray-700' : ''"
          @click="onInfoToggle"
        >
          <InformationCircleIcon
            :class="infoActive ? 'text-purple-600 dark:text-purple-400 hover:dark:text-gray-300' : 'text-gray-600'"
            class="h-6 w-6"
          />
        </div>
        <div class="flex items-center space-x-3">
          <LargeSwitch v-model="darkMode" @input="onDarkModeToggle" />
          <MoonIcon v-if="!darkMode" class="h-6 w-6 text-gray-600" />
          <SolidMoonIcon v-else class="h-6 w-6 text-purple-600" />
        </div>
      </header>
      <div class="items-end px-9 pt-14 pb-3">
        <input
          v-model="operationDisplay"
          placeholder="0"
          disabled
          class="w-full appearance-none truncate bg-transparent text-right text-xl font-bold focus:outline-none disabled:text-gray-900 dark:disabled:text-white"
        />
        <input
          ref="mathInput"
          v-model="currentNum"
          placeholder="0"
          disabled
          class="w-full appearance-none truncate bg-transparent text-right text-3xl font-bold focus:outline-none disabled:text-gray-900 dark:disabled:text-white"
        />
      </div>
      <div
        class="flex h-full flex-col space-y-6 rounded-t-3xl bg-white px-7 pt-5 pb-8 transition ease-in-out dark:bg-gray-800"
      >
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton type="function" @click="onOperation('abs')">ABS</BaseButton>
          <BaseButton type="function" @click="onOperation('factorial')">!</BaseButton>
          <BaseButton type="function" @click="onOperation('pow')">xⁿ</BaseButton>
          <BaseButton type="function" @click="onOperation('root')">√x</BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton type="function" @click="onOperation('add')">+</BaseButton>
          <BaseButton type="function" @click="onOperation('subtract')">-</BaseButton>
          <BaseButton type="function" @click="onOperation('divide')">/</BaseButton>
          <BaseButton type="function" @click="onOperation('multiply')">*</BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton @click="onNumber('7')">7</BaseButton>
          <BaseButton @click="onNumber('8')">8</BaseButton>
          <BaseButton @click="onNumber('9')">9</BaseButton>
          <BaseButton type="function" @click="onBackspace()"><BackspaceIcon class="h-6 w-6" /></BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <BaseButton @click="onNumber('4')">4</BaseButton>
          <BaseButton @click="onNumber('5')">5</BaseButton>
          <BaseButton @click="onNumber('6')">6</BaseButton>
          <BaseButton type="function" @click="onEraseAll()">AC</BaseButton>
        </div>
        <div class="grid grow grid-cols-4 gap-x-6">
          <div class="col-span-2 flex flex-col gap-y-6">
            <div class="grid grow grid-cols-2 gap-x-6">
              <BaseButton @click="onNumber('1')">1</BaseButton>
              <BaseButton @click="onNumber('2')">2</BaseButton>
            </div>
            <div class="grid grow">
              <BaseButton @click="onNumber('0')">0</BaseButton>
            </div>
          </div>
          <div class="flex grow flex-col space-y-6">
            <BaseButton @click="onNumber('3')">3</BaseButton>
            <BaseButton @click="onNumber('.')">.</BaseButton>
          </div>
          <BaseButton type="filled" @click="doCalculation()">=</BaseButton>
        </div>
      </div>
    </div>
    <InfoCard v-if="infoActive" />
  </div>
</template>

<script lang="ts">
import { InformationCircleIcon, MoonIcon, BackspaceIcon } from '@heroicons/vue/outline'
import { MoonIcon as SolidMoonIcon } from '@heroicons/vue/solid'
import { defineComponent, ref, onMounted, onUnmounted } from 'vue'
import LargeSwitch from './components/LargeSwitch.vue'
import BaseButton from './components/BaseButton.vue'
import { invoke } from '@tauri-apps/api/tauri'
import InfoCard from './components/InfoCard.vue'

export default defineComponent({
  components: {
    InfoCard,
    BaseButton,
    InformationCircleIcon,
    MoonIcon,
    SolidMoonIcon,
    LargeSwitch,
    BackspaceIcon,
  },
  setup() {
    let darkMode = ref(false) // If true, the theme is dark
    let mathInput = ref<InstanceType<typeof HTMLInputElement>>() // The displayed number
    let operationDisplay = ref('') // The displayed operation
    let infoActive = ref(false) // If true, the info card is active
    let currentNum = ref('') // Current number
    let previousNum = ref('') // Previous number
    let operation = ref('') // Operation
    let calcError = ref(false) // If error is present
    let isResult = ref(false) // If result is currently shown

    /**
     * @brief Dark mode switch handler
     */
    const onDarkModeToggle = () => {
      darkMode.value = !darkMode.value
    }

    /**
     * @brief Info button handler
     */
    const onInfoToggle = () => {
      infoActive.value = !infoActive.value
    }

    /**
     * @brief Handles clicks when error or result is present
     */
    const middleware = () => {
      if (calcError.value || isResult.value) {
        currentNum.value = ''
        isResult.value = false
        calcError.value = false
      }
    }

    /**
     * @brief Display last operation
     */
    const displayOperation = () => {
      if (['add', 'subtract', 'multiply', 'divide'].includes(operation.value)) {
        const map = { add: '+', subtract: '-', multiply: '*', divide: '/' }
        operationDisplay.value = `${previousNum.value} ${map[operation.value]} ${currentNum.value} =`
      } else if ('factorial' === operation.value) {
        operationDisplay.value = `${previousNum.value}! =`
      } else if ('abs' === operation.value) {
        operationDisplay.value = `| ${previousNum.value} | =`
      } else {
        operationDisplay.value = `${operation.value}(${previousNum.value}, ${currentNum.value}) =`
      }
    }

    /**
     * @brief Check if operation is unary
     *
     * @param op Operation
     */
    const isUnaryOperation = (op: string) => {
      return op === 'abs' || op === 'factorial'
    }

    /**
     * @brief Number handler, also handles dot
     *
     * @param num Number or dot to be added
     */
    const onNumber = (num: string) => {
      middleware()

      if (num === '.') {
        // Prevent multiple dots in one number
        if (!currentNum.value.includes('.')) {
          if (!currentNum.value) currentNum.value = '0'
          currentNum.value += '.'
        }
        return
      }

      // Prevent leading zeros
      if (currentNum.value === '0') currentNum.value = ''

      currentNum.value += num
    }

    /**
     * @brief AC button handler
     */
    const onEraseAll = () => {
      currentNum.value = ''
      previousNum.value = ''
      operation.value = ''
      calcError.value = false
      operationDisplay.value = ''
    }

    /**
     * @brief Backspace handler
     */
    const onBackspace = () => {
      middleware()

      currentNum.value = currentNum.value.slice(0, -1)
    }

    /**
     * @brief Operation handler
     *
     * @param op Function character
     */
    const onOperation = (op: string) => {
      // middleware()

      // If current number is not set
      if (!currentNum.value) currentNum.value = '0'

      // Handle unary operations
      if (isUnaryOperation(op)) {
        previousNum.value = currentNum.value
        operation.value = op
        doCalculation()
        return
      }

      // If function is already set
      if (operation.value && previousNum.value) {
        doCalculation()
        operation.value = op
        return
      }

      // Shift numbers
      previousNum.value = currentNum.value
      currentNum.value = ''

      operation.value = op
    }

    /**
     * @brief Do actual calculation (by calling to the backend via API)
     */
    const doCalculation = () => {
      if (!operation.value || !previousNum.value) return

      invoke('math_operation', {
        payload: {
          a: previousNum.value,
          b: currentNum.value == '' || isUnaryOperation(operation.value) ? undefined : currentNum.value,
          operation: operation.value,
        },
      })
        .then((result: string) => {
          displayOperation()
          calcError.value = false
          currentNum.value = result
          previousNum.value = ''
          isResult.value = true
          operation.value = ''
        })
        .catch((error) => {
          displayOperation()
          calcError.value = true
          currentNum.value = error
          previousNum.value = ''
          isResult.value = false
          operation.value = ''
        })
    }

    /**
     * @brief Keyboard input handler
     */
    const onKeyPress = (e: KeyboardEvent) => {
      const events = {
        '0': () => onNumber('0'),
        '1': () => onNumber('1'),
        '2': () => onNumber('2'),
        '3': () => onNumber('3'),
        '4': () => onNumber('4'),
        '5': () => onNumber('5'),
        '6': () => onNumber('6'),
        '7': () => onNumber('7'),
        '8': () => onNumber('8'),
        '9': () => onNumber('9'),
        '.': () => onNumber('.'),
        '+': () => onOperation('add'),
        '-': () => onOperation('subtract'),
        '*': () => onOperation('multiply'),
        '/': () => onOperation('divide'),
        a: () => onOperation('abs'),
        f: () => onOperation('factorial'),
        p: () => onOperation('pow'),
        r: () => onOperation('root'),
        c: () => onOperation('cos'),
        '=': () => doCalculation(),
        Enter: () => doCalculation(),
        Backspace: () => onBackspace(),
        Delete: () => onEraseAll(),
      }
      if (typeof events[e.key] === 'function') {
        events[e.key]()
      }
    }

    // Register event listeners
    onMounted(() => {
      document.addEventListener('keydown', onKeyPress)
    })
    onUnmounted(() => {
      document.removeEventListener('keydown', onKeyPress)
    })

    return {
      darkMode,
      onDarkModeToggle,
      mathInput,
      currentNum,
      operationDisplay,
      onNumber,
      onOperation,
      doCalculation,
      onEraseAll,
      onBackspace,
      infoActive,
      onInfoToggle,
    }
  },
})
</script>
