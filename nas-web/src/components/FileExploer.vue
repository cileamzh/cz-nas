<!-- eslint-disable vue/require-toggle-inside-transition -->
<template>
  <div id="file-explorer">
    <div id="folders">
      <FolderTreeItem :folder="user.user_dir" :depth="0" :parent-path="''"></FolderTreeItem>
    </div>
    <div id="folder-detail" class="flex-row" @dragover.prevent="in_dragging" @dragleave="un_dragging"
      @drop.prevent="accept_drag" :class="explorer_status.mode.current + '-detail'"
      @contextmenu="open_explorer_context_menu">
      <FileItem class="fitem" v-for="f in explorer_status.current_dir.dir" :key="f.name" :file="f"
        :mode="explorer_status.mode.current">
      </FileItem>
    </div>
  </div>
</template>
<script setup>
import { onMounted, ref } from 'vue';
import FolderTreeItem from './FolderTreeItem.vue';
import { add_msg, context_menu, explorer_status, lang, load_current_dir, user } from '@/util/globel';
import FileItem from './FileItem.vue';
import { upload } from '@/util/func';
import FileExplorerMenu from './FileExplorerMenu.vue';

const is_dragging = ref(false);

onMounted(() => {
  load_current_dir(user.user_dir.path)
})
function accept_drag(e) {
  if (e.dataTransfer.files.length < 1) {
    return
  }
  upload(explorer_status.current_dir.path, e.dataTransfer.files).then((rst) => {
    if (!rst.ok) {
      return add_msg({ type: "warn", msg: lang.current.fail_upload })
    }
    add_msg({ type: 'common', msg: lang.current.success_upload })
    load_current_dir(explorer_status.current_dir.path)
  }).catch(e => {
    console.log(e);
  })
}

function in_dragging() {
  is_dragging.value = true
}
function un_dragging() {
  is_dragging.value = false
}
function open_explorer_context_menu(e) {
  context_menu.visible = false
  context_menu.x = e.clientX;
  context_menu.y = e.clientY;
  context_menu.visible = true
  context_menu.data = null;
  context_menu.component = FileExplorerMenu
}


</script>
<style scoped lang="scss">
#file-explorer {
  display: flex;
  height: 100%;
  // color: rgba(255, 255, 255, 0.836);
  background-color: rgb(252, 252, 252);

  #folders {
    height: 100%;
    background-color: rgb(190, 226, 255);
    padding: 5px;
  }

  #folder-detail {
    flex: 1;
    padding: 8px;
  }

  .view-detail {

    height: unset;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
    gap: 8px;
    align-items: start;
    overflow-y: auto;
  }

  .list-detail {
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    gap: 1px;
  }
}
</style>
