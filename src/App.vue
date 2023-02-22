<script setup>
// import Greet from "./components/Greet.vue";

import { toggleTheme } from '@zougt/vite-plugin-theme-preprocessor/dist/browser-utils.js';

//切换主题
let theme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
toggleTheme({scopeName: 'theme-' + theme});
</script>

<template>
  <a-button type="primary">Primary Button</a-button>
  <a-button>Default Button</a-button>
  <a-button type="dashed">Dashed Button</a-button>
  <a-button type="text">Text Button</a-button>
  <a-button type="link">Link Button</a-button>
</template>

<style scoped>

</style>
