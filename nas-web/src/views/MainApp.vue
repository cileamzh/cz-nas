<template>
  <div id="main" @contextmenu.prevent="main_menu" :style="{ backgroundImage: `url(${user.info.bg})` }">
    <div v-for="app in apps" :key="app.name" @click="open_app(app)" class="section">
      <img :src="app.icon" :alt="app.name">
    </div>
    <Flowin v-model:map="tasks"></Flowin>
    <TaskBar v-model:map="tasks"></TaskBar>
  </div>
</template>
<style lang="scss" scoped>
#main {
  width: 100vw;
  height: 100vh;
  background-size: cover;
  display: flex;
  flex-direction: column;
  z-index: 1;
  overflow: hidden;
}

.section {
  width: 50px;
  height: 50px;
  background-color: rgba(255, 255, 255, 0.429);
  border-radius: 5px;
  overflow: hidden;
  user-select: none;
  margin: 10px 0px 0px 6px;

  img {
    margin: 12.5px;
    width: 25px;
    height: 25px;
  }
}

.section:hover {
  filter: brightness(90%);
}
</style>
<script setup>
import Flowin from '@/components/WinSys.vue';
import TaskBar from '@/components/TaskBar.vue';
import { add_task, apps, explorer_status, tasks, user } from '@/util/globel';
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';
const router = useRouter()

onMounted(() => {
  explorer_status.init();
  fetch("/auth/userinfo").then(res => res.json()).then(rst => {
    user.info = rst.data;
  })
    .catch((e) => {
      console.error(e);
      router.push({ name: "login" })
    })
  fetch("/auth/user_dir").then(res => res.json()).then((rst) => {
    user.user_dir = rst.user_dir;
  })


})

function open_app(app) {
  add_task(app);
}
</script>
