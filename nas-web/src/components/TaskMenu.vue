<script setup lang="js">
import { context_menu, lang, tasks } from '@/util/globel';
function close_task() {
  tasks.delete(context_menu.data[0])
}
</script>

<template>
  <div class="flex-col">
    <div class="context-section flex-row" @click="close_task">{{ lang.current.close }}</div>
    <!-- <div class="context-section flex-row">{{ lang.current.minimize }}</div> -->
  </div>
</template>