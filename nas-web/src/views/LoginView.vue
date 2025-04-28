<template>
  <div id="main">
    <form @submit.prevent="submit_login">
      <div id="login-box">
        <h3>{{ lang.current.login_header }}</h3>
        <div class="input-box">
          <p>{{ lang.current.username }}</p>
          <input type="text" name="uname" v-model="login_form.username">
        </div>
        <div class="input-box">
          <p>{{ lang.current.password }}</p>
          <input type="password" name="upassword" v-model="login_form.password">
        </div>
        <button type="submit">{{ lang.current.login }}</button>
        <div>
          <h6>{{ lang.current.login_help }}</h6>
        </div>
      </div>
    </form>
  </div>
</template>
<style lang="scss" scoped>
#main {
  height: 100vh;
  width: 100vw;
  background-image: url("/bg.png");
  background-size: cover;
  display: flex;
  justify-content: center;
  flex-direction: column;
}

#login-box {
  width: 400px;
  height: 370px;
  display: flex;
  gap: 15px;
  margin-left: 50px;
  flex-direction: column;
  align-items: center;
  border-radius: 20px;
  background-color: #e9f7ffaf;

  h3 {
    margin-top: 30px;
  }

  .input-box {
    height: 75px;
    width: 90%;
    border: none;
    overflow: hidden;



    p {
      font-size: 12px;
      height: 12px;
      margin-bottom: 12px;
      margin-left: 2px;
    }

    input {
      width: 100%;
      border-radius: 10px;
      height: 50px;
      outline: none;
      box-shadow: none;
      margin-bottom: 2px;
    }
  }

  button {
    width: 90%;
    height: 50px;
    margin-top: 10px;
    border-radius: 10px;
    outline: none;
    box-shadow: none;
  }
}
</style>
<script setup>
import { add_msg, lang } from '@/util/globel';
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const login_form = ref({
  username: "",
  password: ""
})
let router = useRouter()
function submit_login() {
  fetch("/unauth/login", {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({
      username: login_form.value.username,
      password: login_form.value.password
    }) // You can add login data here if needed
  })
    .then(response => {
      if (response.status == 400) {
        add_msg({
          type: "warn",
          msg: "Error"
        })
      }
      if (response.ok) {
        add_msg({
          type: "common",
          msg: "Login Successfully"
        })
        router.replace({ name: "main" })
      }

    })
}
</script>
