<template>
  <div class="flex-col" @click="() => {
    context_menu.data.is_selected = false
  }">
    <div class="flex-row context-section" @click="context_menu.data.start_rename">{{ lang.current.rename }}</div>
    <div class="flex-row context-section" @click="context_menu.data.delete_file">{{ lang.current.delete }}</div>
    <div class="flex-row context-section" @click="context_menu.data.download">{{ lang.current.download }}</div>
    <div class="flex-row context-section" @click="context_menu.data.set_avator" v-if="is_photo(context_menu.data.name)">
      {{ lang.current.set_avator }}
    </div>
    <div class="flex-row context-section" @click="context_menu.data.set_as_bg" v-if="is_photo(context_menu.data.name)">
      {{ lang.current.set_as_bg }}
    </div>
  </div>
</template>
<style lang="scss" scoped></style>
<script setup>
import { is_photo } from '@/util/func';
import { context_menu, lang } from '@/util/globel';

</script>