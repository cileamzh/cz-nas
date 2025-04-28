<template>
  <div class="folder-tree-item">
    <div class="folder-label"
      :style="{ color: is_open ? 'rgb(10, 108, 255)' : '', backgroundColor: is_selected ? 'rgba(255, 255, 255, 0.836)' : '' }"
      @dragover.prevent="dovr" @dragleave="dout" @drop="drop">
      <span @click="toggle">{{ is_open ? '📂' : '📁' }}</span>
      <span @click="show_dir">{{ props.folder.name }}</span>
    </div>
    <div v-if="is_open" class="children">
      <FolderTreeItem v-for="f in dir" :key="f.name" :folder="f" @update="handle_update" />
    </div>
  </div>
</template>

<script setup>
import { upload } from '@/util/func'
import { explorer_status } from '@/util/globel'
import { computed, ref, watch } from 'vue'
import { defineProps } from 'vue'

const props = defineProps({ folder: Object })
const emit = defineEmits(['update'])
const is_open = ref(false)
const dir = ref([])
const dir_all = ref([])
const loaded = ref(false)
const is_drag = ref(false)

function dovr() {
  is_drag.value = true
}

function dout() {
  is_drag.value = false
}

function drop(e) {
  upload(props.folder.path, e.dataTransfer.files).then(() => {
    loaded.value = false
  }).catch(e => {
    console.log(e);
  })
}

function toggle() {
  is_open.value = !is_open.value
  if (is_open.value && !loaded.value) {
    fetch("/auth/open/" + props.folder.path)
      .then(res => res.json())
      .then(rst => {
        dir_all.value = rst.rows;
        dir.value = rst.rows.filter(f => f.is_dir)
        loaded.value = true
      })
  }
}
let is_selected = computed(() => {
  return explorer_status.current_dir.path == props.folder.path ? true : false;
})

watch(is_selected, (a) => {
  if (a) {
    emit('update', true)
  }
})

function show_dir() {
  if (!loaded.value) {
    fetch("/auth/open/" + props.folder.path)
      .then(res => res.json())
      .then(rst => {
        dir_all.value = rst.rows;
        dir.value = rst.rows.filter(f => f.is_dir)
        loaded.value = true
        explorer_status.current_dir.dir = dir_all.value;
        explorer_status.current_dir.path = props.folder.path
      })
  } else {
    explorer_status.current_dir.dir = dir_all.value;
    explorer_status.current_dir.path = props.folder.path
  }


}
function handle_update(v) {
  is_selected.value = v;
}
</script>

<style scoped>
.folder-tree-item {
  user-select: none;
}

.folder-label {
  padding-right: 2px;
  transition: 0.4s;
  font-size: 18px;
  height: 25px;
  cursor: pointer;
  display: flex;
  align-items: center;

}

.children {
  padding-left: 5px;
}
</style>
