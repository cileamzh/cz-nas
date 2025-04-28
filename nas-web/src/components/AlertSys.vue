<template>
  <div class="flex-col alert-sys">
    <transition-group name="fade" tag="div">
      <div v-for="alert in alert_map" :key="alert[1].idx" class="alert flex-row" :class="alert[1].type || 'common'">
        {{ alert[1].msg }}
        <img src="/close2.png" @click="rm_msg(alert[0])">
      </div>
    </transition-group>
  </div>
</template>
<script setup>
const alert_map = defineModel('map', {
  type: Map
});

function rm_msg(key) {
  alert_map.value.delete(key)
}
</script>
<style lang="scss" scoped>
.alert-sys {
  top: 10px;
  left: 50%;
  width: 400px;
  transform: translateX(-50%);
  height: 0px;
  z-index: 999;
  position: fixed;
}

.alert {
  height: 30px;
  border-radius: 3px;
  gap: 10px;
  padding: 5px;
  margin-top: 10px;

  img {
    margin-left: auto;
    height: 10px;
    width: 10px;
  }
}

.warn {
  background-color: rgb(255, 129, 129);
}

.common {
  background-color: rgb(137, 210, 146);
}

.msg {
  background-color: white;
}
</style>