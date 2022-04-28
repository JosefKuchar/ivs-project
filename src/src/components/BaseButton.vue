<template>
  <button
    class="flex h-full cursor-pointer items-center justify-center rounded-lg p-6 font-semibold transition ease-in-out"
    :class="buttonClasses"
    tabindex="-1"
  >
    <slot></slot>
  </button>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'

export default defineComponent({
  props: {
    type: {
      type: String,
      default: '',
    },
  },
  setup(props) {
    let buttonClasses = ref('')

    switch (props.type) {
      case 'function':
        buttonClasses.value =
          'text-purple-600 bg-purple-50 dark:text-purple-400 dark:bg-gray-700 dark:hover:bg-gray-600 hover:bg-purple-100'
        break
      case 'filled':
        buttonClasses.value = 'text-white bg-purple-600 hover:bg-purple-700'
        break
      default:
        buttonClasses.value =
          'text-gray-900 bg-gray-50 hover:bg-gray-100 dark:text-white dark:bg-gray-700 dark:hover:bg-gray-600'
        break
    }

    return {
      buttonClasses,
    }
  },
})
</script>
