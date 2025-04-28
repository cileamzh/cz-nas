<template>
  <div id="main">
    <router-view>
    </router-view>
    <AlertSys v-model:map="msg_map" class="flex-col alert-sys"></AlertSys>
    <ContextMenu v-model:option="context_menu"></ContextMenu>
  </div>
</template>
<style lang="scss" scoped></style>
<script setup>
import { onMounted } from 'vue';
import { RouterView, useRouter } from 'vue-router';
import AlertSys from './components/AlertSys.vue';
import { context_menu, lang, msg_map } from './util/globel';
import ContextMenu from './components/ContextMenu.vue';
function getCookie(name) {
  const value = `; ${document.cookie}`
  const parts = value.split(`; ${name}=`)
  if (parts.length === 2) return parts.pop().split(';').shift()
}
let router = useRouter()
onMounted(() => {
  lang.init();
  let tkb = getCookie("token");
  if (!tkb) {
    router.push({ name: "login" })
  }
  if (tkb) {
    return ""
  }
})
</script>