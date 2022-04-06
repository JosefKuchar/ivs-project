<template>
  <button class="p-6 rounded-lg cursor-pointer transition ease-in-out font-semibold flex items-center justify-center transition ease-in-out"
          :class="buttonClasses">
      <slot></slot>
  </button>
</template>

<script lang="ts">
import { defineComponent, ref, watch } from "vue";

export default defineComponent({
  props: {
      type: {
          type: String,
          default: ''
      },
      dark: {
          type: Boolean,
          default: false
      }
  },
  setup(props) {
      let buttonClasses = ref('')

      switch (props.type) {
          case 'function':
              buttonClasses.value = 'text-purple-600 bg-purple-50 hover:bg-purple-100';
              break
          case 'filled':
              buttonClasses.value = 'text-white bg-purple-600 hover:bg-purple-700'
              break
          default:
              buttonClasses.value = 'text-gray-900 bg-gray-50 hover:bg-gray-100'
              break
      }

      if (props.dark) {
          switch (props.type) {
              case 'function':
                  buttonClasses.value = 'text-purple-400 bg-gray-700 hover:bg-gray-600';
                  break
              case 'filled': break
              default:
                  buttonClasses.value = 'text-white bg-gray-700 hover:bg-gray-600'
                  break
          }
      }

      return {
          buttonClasses
      }
  }
})
</script>