<script setup>
import { context_menu, lang, set_index } from '@/util/globel';
import CurrentTime from './CurrentTime.vue';
import TaskMenu from './TaskMenu.vue';

const map = defineModel('map', { type: Map });
const is_show = defineProps({ is_show: Boolean })
function open_task_menu(e, k, v) {
  context_menu.visible = false
  context_menu.visible = true
  context_menu.x = e.clientX
  context_menu.y = e.clientY - 30
  context_menu.component = TaskMenu
  context_menu.data = [k, v]
}
</script>

<template>

  <div id="task-bar" class="flex-row">
    <div class="logo flex-row"><img src="/logo3.png" style="background-color: white;"></div>
    <TransitionGroup name="fadex" tag="div" class="tasks-list flex-row">
      <div v-for="kv in map" :key="kv[0]" class="task-item flex-row" @click="set_index(kv[0])" @contextmenu="(e) => {
        open_task_menu(e, kv[0], kv[1])
      }">
        <img :src="kv[1].icon">
        <span v-if="is_show">{{ lang.current[kv[1].name] }}</span>
      </div>
    </TransitionGroup>
    <div class="task-bar-status flex-row">
      <CurrentTime></CurrentTime>
    </div>
  </div>

</template>
<style lang="scss" scoped>
#task-bar {
  width: 100vw;
  position: fixed;
  bottom: 0px;
  height: 40px;
  background-color: rgba(255, 255, 255, 0.274);
  gap: 5px;

  .logo {
    align-items: center;
    width: 40px;
    height: 40px;
    justify-content: center;
    transition: 0.2s;

    &:hover {
      background-color: rgba(255, 255, 255, 0.719);
    }

    img {
      width: 35px;
      height: 35px;
      border-radius: 50%;
    }
  }

  .tasks-list {
    flex: 1;
    height: 100%;

    .task-item {
      padding-left: 5px;
      padding-right: 5px;
      height: 40px;
      line-height: 40px;
      gap: 5px;
      font-size: 12px;
      transition: 0.2s;

      img {
        height: 20px;
        max-width: 30px;
      }

      &:hover {
        background-color: rgba(255, 255, 255, 0.6);
      }
    }

  }

  .task-bar-status {
    height: 100%;
    margin-left: auto;
    padding-right: 5px;
  }
}
</style>