<template>
  <div>
    <label class="cursor-pointer">
      <input type="checkbox" :value="modelValue" :checked="modelValue" class="hidden" @click="onInputEvent" />
      <div
        tabindex="0"
        class="flex items-center space-x-1 rounded-full p-1 text-sm font-semibold"
        :class="modelValue ? 'bg-purple-600' : 'bg-gray-300'"
      >
        <div v-if="modelValue" class="text-white">ON</div>
        <div class="h-5 w-5 rounded-full bg-white"></div>
        <div v-if="!modelValue" class="text-gray-600">OFF</div>
      </div>
    </label>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    modelValue: {
      type: Boolean,
      default: false,
    },
  },
  emits: ['input', 'update:modelValue'],
  setup(props, { emit }) {
    const onInputEvent = (event: Event) => {
      emit('input', (event.target as HTMLInputElement).checked)
      emit('update:modelValue', !props.modelValue)
    }

    return {
      onInputEvent,
    }
  },
})
</script>
