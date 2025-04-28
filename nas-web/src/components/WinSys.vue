<!-- eslint-disable vue/multi-word-component-names -->
<template>
  <div id="flowin-container" style="overflow: hidden;">
    <TransitionGroup name="fadex" tag="div">
      <div class="flowin set-move-win" v-for="kv in map" :key="kv[0]" @click="set_index(kv[0])"
        :style="{ zIndex: kv[1].zIndex }" @mousedown="set_index(kv[0])">
        <div class="flowin-control set-move-win-trigger flex-row">
          <div class="flex-row app-info">
            <img :src="kv[1].icon"> <span>{{ lang.current[kv[1].name] }}</span>
          </div>
          <div @click.prevent="close_flowin(kv[0])" @mousedown.stop="null" class="flex-row close" style="height: 100%;">
            <img src="/close1.png">
          </div>
        </div>
        <div class="flowin-content">
          <component v-bind:is="kv[1].component">

          </component>
        </div>

      </div>
    </TransitionGroup>
  </div>
</template>
<style scoped lang="scss">
.flowin {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 900px;
  height: 600px;
  background-color: rgb(255, 255, 255);
  border-radius: 5px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border: 1px solid rgb(71, 71, 71);

  .flowin-control {
    height: 30px;
    background-color: #dbeaff;

    .app-info {
      padding-left: 5px;
      flex: 1;
      height: 100%;
      gap: 5px;

      img {
        height: 60%;
        object-fit: contain;
      }
    }

    .close {
      margin-left: auto;
      width: 30px;
      height: 100%;

      img {
        width: 60%;
        height: 60%;
      }
    }
  }

  .flowin-content {
    flex: 1;
    overflow-y: auto;
  }
}
</style>

<script setup>
import { set_move_win } from '@/util/func';
import { lang } from '@/util/globel';
import { onMounted } from 'vue';
const map = defineModel('map', { type: Map })
function close_flowin(key) {
  map.value.delete(key)
}

function set_index(key) {
  map.value.forEach((v, k) => {
    if (key != k) {
      v.zIndex = 1
    } else {
      v.zIndex = 9
    }
  })
}

onMounted(() => {
  set_move_win('set-move-win', 'set-move-win-trigger')
})
</script>