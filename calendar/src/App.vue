<script setup>
import Calendar from './components/Calendar.vue'
import GitHubLogo from '../public/favicon.png';
import { getGitHubUrl } from './utils/getGitHubUrl'


function generateRandomState() {
  const state = Math.random().toString(36).substring(7);
  return state;
}

const from = generateRandomState();
</script>


<template>
  <div>
    <h1> Triage Schedule </h1>
      <Calendar />
  </div>

  <div class="container">
    <div class="social-auth">
      <!-- GitHub OAuth -->
      <a :href="getGitHubUrl(from)" class="auth-btn github-auth">
        <img :src="GitHubLogo" alt="GitHub Logo" />
        <span>GitHub</span>
      </a>
    </div>
  </div>
</template>


<style scoped>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
a {
  text-decoration: none;
  color: inherit;
}

html {
  font-size: 62.5%;
}

body {
  font-family: Roboto, sans-serif;
  color: #222;
  font-size: 1.6rem;
}

.container {
  background-color: #2363eb;
  height: 100vh;
  width: 100vw;
  display: flex;
  align-items: center;
  justify-content: center;
}

.social-auth {
  max-width: 27rem;
  width: 100%;
  display: flex;
  align-items: center;
  flex-direction: column;
}

.auth-btn {
  background-color: #fff;
  border-radius: 5px;
  padding: 0.6rem 0;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease-in-out;
}

.auth-btn img {
  height: 4rem;
  margin-right: 1rem;
}
.auth-btn span {
  font-size: 1.8rem;
}

.auth-btn:hover {
  box-shadow: 0 1px 13px 0 rgb(0 0 0 / 15%);
}

.auth-btn.github-auth {
  margin-bottom: 1.5rem;
}
</style>
