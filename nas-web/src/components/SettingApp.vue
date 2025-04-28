<!-- eslint-disable vue/multi-word-component-names -->
<template>
  <div id="setting-component" class="flex-col">
    <div id="user-setting" class="flex-col">
      <div id="avatar">
        <img :src="user.info.avator" alt="Avatar" />
      </div>
      <h2>{{ user.info.username }}</h2>

      <div class="setting-row flex-row">
        <span>{{ lang.current["file_manager_mode"] }}:</span>
        <WrapSection :list="explorer_status.mode.list" v-model:selected="explorer_status.mode.current"
          @change="change_mode" class="wrap-section" />
      </div>

      <div class="setting-row">
        <span>Language:</span>
        <WrapSection :list="lang.list" v-model:selected="lang.selected" @change="change_lang" class="wrap-section" />
      </div>
    </div>

    <div class="quit-btn">
      <button @click="quit">Exit User</button>
    </div>
  </div>
</template>

<script setup>
import { add_msg, explorer_status, lang, tasks, user } from '@/util/globel';
import { useRouter } from 'vue-router';
import WrapSection from './WrapSection.vue';

const router = useRouter();

function quit() {
  document.cookie = "token=; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT";
  add_msg({ type: "common", msg: "Quit Successfully" });
  router.push({ name: "login" });
  tasks.clear()
}

function change_mode(v) {
  explorer_status.mode.current = v;
  localStorage.setItem("explorer-mode", v);
}

function change_lang(v) {
  lang.current = lang[v.value];
  localStorage.setItem("lang", JSON.stringify(v));
}
</script>

<style scoped lang="scss">
#setting-component {
  width: 100%;
  height: 100%;
  padding: 20px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 30px;
  background-color: #f9f9f9;
}

#user-setting {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 15px;

  #avatar {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    overflow: hidden;
    background-color: #eee;

    img {
      width: 100%;
      height: 100%;
      object-fit: cover;
    }
  }

  h2 {
    font-size: 20px;
    font-weight: bold;
    margin: 0;
  }

  .setting-row {

    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    flex-wrap: nowrap;
  }
}

.wrap-section {
  width: 120px;
  border-radius: 6px;
  background-color: rgba(243, 243, 243, 0.8);
  overflow: hidden;
}

.quit-btn {
  margin-top: 20px;

  button {
    padding: 8px 16px;
    background-color: #d9534f;
    border: none;
    border-radius: 5px;
    color: white;
    cursor: pointer;
    font-size: 16px;

    &:hover {
      background-color: #c9302c;
    }
  }
}
</style>
