<template>
  <div class="time-display">
    {{ formattedTime }}
  </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'

const formattedTime = ref('')

function updateTime() {
  const now = new Date()
  const yy = String(now.getFullYear()).slice(2)
  const mm = String(now.getMonth() + 1).padStart(2, '0')
  const dd = String(now.getDate()).padStart(2, '0')
  const hh = String(now.getHours()).padStart(2, '0')
  const min = String(now.getMinutes()).padStart(2, '0')
  const ss = String(now.getSeconds()).padStart(2, '0')
  formattedTime.value = `${yy}:${mm}:${dd}:${hh}:${min}:${ss}`
}

let timer
onMounted(() => {
  updateTime()
  timer = setInterval(updateTime, 1000)
})

onBeforeUnmount(() => {
  clearInterval(timer)
})
</script>

<style scoped>
.time-display {
  font-family: monospace;
}
</style>
