<!-- eslint-disable vue/multi-word-component-names -->
<template>
  <div class="player">
    <div v-if="mediaUrl">
      <!-- 根据链接的格式判断是否是音频或视频 -->
      <audio v-if="isAudio" :src="mediaUrl" controls>
        Your browser does not support the audio element.
      </audio>

      <video v-if="isVideo" :src="mediaUrl" controls width="600">
        Your browser does not support the video element.
      </video>

      <!-- 如果不是音频或视频格式，则显示错误提示 -->
      <div v-if="!isAudio && !isVideo" class="error">
        Unsupported media format or invalid URL.
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    mediaUrl: {
      type: String,
      required: true, // 确保父组件传递了 mediaUrl
    },
  },
  computed: {
    // 判断链接是否是音频文件
    isAudio() {
      return this.mediaUrl && /\.(mp3|wav|ogg)$/i.test(this.mediaUrl);
    },
    // 判断链接是否是视频文件
    isVideo() {
      return this.mediaUrl && /\.(mp4|webm|ogg)$/i.test(this.mediaUrl);
    },
  },
};
</script>

<style scoped>
.player {
  text-align: center;
  padding: 20px;
}

audio,
video {
  margin-top: 20px;
}

.error {
  color: red;
  margin-top: 20px;
}
</style>
