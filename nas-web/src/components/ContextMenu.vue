<template>
  <teleport to="body">
    <Transition name="scale">
      <div class="context-menu" v-if="option.visible" :style="{ left: `${option.x}px`, top: `${option.y}px` }"
        :ref="(el) => menu = el">
        <component v-bind:is="option.component"></component>
      </div>
    </Transition>
  </teleport>
</template>

<script setup>
import { onMounted, onUnmounted, ref } from 'vue';

const option = defineModel('option')
const menu = ref(null)

function hideContextMenu() {
  option.value.visible = false
  option.value.data = null
}

onMounted(() => {
  window.addEventListener('click', hideContextMenu);
});

onUnmounted(() => {
  window.removeEventListener('click', hideContextMenu);
});
</script>

<style scoped lang="scss">
.context-menu {
  position: fixed;
  background: white;
  border: 1px solid #ccc;
  border-radius: 6px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
  width: 100px;
  min-height: 30px;
  z-index: 9999;
  overflow: hidden;
}
</style>
