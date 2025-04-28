<script setup>
import { delete_file, get_file_url, rename_file, set_as_bg, set_avator } from '@/util/func'
import { add_msg, context_menu, explorer_status, lang, load_current_dir, user } from '@/util/globel'
import { onMounted, reactive, ref, watch } from 'vue'
import FileContextMenu from './FileContextMenu.vue'


const props = defineProps({
  file: { type: Object, required: true },
  mode: { type: String, default: "view" }
})

const file = reactive({
  is_editing: false,
  name: props.file.name,
  path: props.file.path,
  icon: '',
  is_selected: false,
  dbc() { },
  start_rename() { },
  finish_rename() { },
  delete_file() { }
})

const input = ref(null);
file.start_rename = () => {
  file.is_editing = true
}

file.dbc = () => {
  open("/auth/open/" + file.path, "_blank")
}

file.finish_rename = () => {
  if (file.is_editing) {
    if (file.name == props.file.name) {
      return file.is_editing = false
    }
    rename_file(file.path, file.name).then((rst) => {
      if (!rst.ok) {
        file.name = props.file.name
        return add_msg({
          type: "warn",
          msg: lang.current.failed_rename
        })
      }
      add_msg({ type: "common", msg: lang.current.success_rename })
      load_current_dir(explorer_status.current_dir.path)
    })
    file.is_editing = false
  }
}

file.delete_file = () => {
  delete_file(file.path).then(rst => {
    if (!rst.ok) {
      return add_msg({ type: "warn", msg: lang.current.failed_delete })
    }
    add_msg({ msg: lang.current.success_delete })
    load_current_dir(explorer_status.current_dir.path)
  })
}

file.download = () => {
  const link = document.createElement('a');
  link.href = "/auth/open/" + file.path; // 替换成你的文件地址
  link.download = file.name; // 可以写自定义文件名，比如 'myfile.zip'
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
}

file.set_avator = () => {
  set_avator("/auth/open/" + file.path).then(rst => {
    if (!rst.ok) {
      return add_msg({ type: "warn", msg: lang.current.fail_set })
    }
    add_msg({ type: "common", msg: lang.current.success_set })
    user.info.avator = "/auth/open/" + file.path;
  })
}

file.set_as_bg = () => {
  set_as_bg("/auth/open/" + file.path).then(rst => {
    if (!rst.ok) {
      return add_msg({ type: "warn", msg: lang.current.fail_set })
    }
    add_msg({ type: "common", msg: lang.current.success_set })
    console.log(file.path);
    user.info.bg = "/auth/open/" + file.path;
  })
}

watch(input, (a) => {
  if (a) {
    a.focus()
  }
})

onMounted(() => {
  get_file_url(props.file).then(rst => {
    file.icon = rst
  })
})



function open_context_menu(e) {
  context_menu.visible = false
  context_menu.x = e.clientX;
  context_menu.y = e.clientY
  context_menu.visible = true
  context_menu.component = FileContextMenu
  context_menu.data = file
}
</script>

<template>
  <div @dblclick="file.dbc" class="file-item" v-if="!props.file.is_dir" :class="[mode]"
    @contextmenu.stop.prevent="open_context_menu">
    <div class="file-icon flex-row">
      <img :src="file.icon">
    </div>
    <div class="file-info">
      <p v-if="!file.is_editing" class="file-name">{{ file.name }}</p>
      <input v-else type="text" v-model="file.name" @keydown.enter="file.finish_rename" @blur="file.finish_rename"
        :ref="(e) => { input = e }">
    </div>
  </div>
</template>

<style scoped lang="scss">
.file-item {
  overflow: hidden;

  &:hover {
    background-color: rgb(231, 231, 231);
  }
}

.selected {
  border: 1px solid gray;
}

.view {
  display: flex;
  height: 100px;
  flex-direction: column;
  align-items: center;
  width: 100px;

  .file-icon {
    width: 100%;
    height: 80px;
    justify-content: center;

    img {
      width: 100%;
      height: 100%;
      object-fit: contain;
    }
  }

  .file-info {
    font-size: 12px;
    overflow: hidden;
    flex: 1;
    max-width: 100%;

    .file-name {
      height: 20px;
      font-size: 10px;
      line-height: 20px;
    }

    input {
      font-size: 10px;
      height: 20px;
      width: 100%;
    }
  }
}

.list {
  display: flex;
  width: 100%;
  flex-wrap: nowrap;
  flex: 0 0 25px;
  gap: 20px;

  .file-icon {
    height: 100%;
    width: 40px;

    img {
      height: 60%;
      object-fit: contain;
    }
  }

  .file-info {
    flex: 1;
    height: 100%;
    font-size: 12px;
    min-width: 0;
    overflow: hidden;

    .file-name {
      margin-top: 2.5px;
      height: 20px;
      line-height: 20px;
      overflow: auto;
    }

    input {
      margin-top: 2.5px;
      line-height: 20px;
      font-size: 12px;
      height: 20px;
      width: 100%;
    }
  }
}
</style>
