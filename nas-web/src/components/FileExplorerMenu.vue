<script setup lang="js">
import { delete_file, mkdir } from '@/util/func';
import { add_msg, explorer_status, lang } from '@/util/globel';
function mkdir_under_current() {
  let dir_name = prompt(lang.current.dir_name);
  if (!dir_name) {
    return add_msg({ type: "warn", msg: lang.current.cancel_mkdir })
  }
  mkdir(explorer_status.current_dir.path + "\\" + dir_name).then(rst => {
    if (!rst.ok) {
      return add_msg({ msg: lang.current.fail_mkdir })
    }
    add_msg({ msg: lang.current.success_mkdir })
  })
}

function delete_current_dir() {
  let r = confirm(lang.current.confirm_delete);
  if (!r) {
    return add_msg({ type: "common", msg: lang.current.cancel })
  }
  delete_file(explorer_status.current_dir.path).then(rst => {
    if (!rst.ok) {
      return add_msg({ type: "warn", msg: lang.current.failed_delete })
    }
    add_msg({ msg: lang.current.success_delete })
  })
}
</script>

<template>
  <div class="flex-col">
    <div class="flex-row context-section" @click="mkdir_under_current">
      {{ lang.current.mkdir }}
    </div>
    <div class="flex-row context-section" @click="delete_current_dir">
      {{ lang.current.delete }}</div>
  </div>
</template>