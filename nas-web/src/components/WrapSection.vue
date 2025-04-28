<script setup>
import { ref, reactive, onMounted, onBeforeUnmount } from 'vue';

const props = defineProps({
  list: Array
});
const emit = defineEmits(['change']);
const is_open = ref(false);
const selected = defineModel('selected');
const positionStyle = reactive({
  position: 'absolute',
  left: '0px',
  top: '0px',
  width: '0px',
});

const previewRef = ref(null);

function toggle() {
  is_open.value = !is_open.value;
  if (is_open.value && previewRef.value) {
    const rect = previewRef.value.getBoundingClientRect();
    positionStyle.left = rect.left + 'px';
    positionStyle.top = rect.bottom + 'px';
    positionStyle.width = rect.width + 'px';
  }
}

function select(v) {
  is_open.value = false;
  selected.value = v;
  emit('change', v);
}

function handleClickOutside(e) {
  if (is_open.value && previewRef.value && !previewRef.value.contains(e.target)) {
    is_open.value = false;
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
});

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside);
});
</script>

<template>
  <div class="wrap-section flex-col">
    <div ref="previewRef" class="wrap-preview flex-row" @click="toggle">
      {{ selected.name || selected }}
    </div>
    <teleport to="body">
      <div v-if="is_open" class="dropdown" :style="positionStyle">
        <div v-for="(section, index) in props.list" :key="index" class="wrap-item flex-row" @click="select(section)">
          {{ section.name || section }}
        </div>
      </div>
    </teleport>
  </div>
</template>

<style scoped lang="scss">
.wrap-section {
  position: relative;
  transition: 0.3s;
  height: 30px;
  overflow: visible;
}

.wrap-preview {
  height: 30px;
  width: 100%;
  padding-left: 5px;
  background-color: #fff;
  cursor: pointer;
  display: flex;
  align-items: center;
}

.dropdown {
  background-color: white;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  display: flex;
  flex-direction: column;
  z-index: 9999;
  animation: fadeIn 0.2s ease-out;

  .wrap-item {
    padding-left: 5px;
    width: 100%;
    flex: 0 0 30px;
    display: flex;
    align-items: center;
    background-color: rgba(255, 255, 255, 0.9);
    cursor: pointer;

    &:hover {
      background-color: rgb(218, 218, 218);
    }
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-5px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
