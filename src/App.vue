<script setup>
// import Greet from "./components/Greet.vue";

let getCommandStr = () => {
  let btn = "Meta+"
  if(window.platform.isMacOs){
    btn = 'Command+'
  } else if (window.platform.isWindows){
    btn = 'Win+'
  }
  return btn;
}
let getOptionStr = () => {
  let btn = "Alt+"
  if(window.platform.isMacOs){
    btn = 'Option+'
  }
  return btn;
}

let keyFunc = (e) => {
  let ctrl = e.ctrlKey,shift = e.shiftKey,alt = e.altKey,meta = e.metaKey;
  let connect = (meta ? getCommandStr() : "" ) +  (ctrl ? "Control+" : "" ) + (alt ? getOptionStr() : "" ) + (shift ? "Shift+" : "" );
  if(e.key !== 'Meta' && e.key !== 'Control' && e.key !== 'Alt' && e.key !== 'Shift'){
    connect = connect + e.key;
  }
  return connect;
}

function inputFocus (e)  {
  console.log(e)
}

</script>

<template>
  <div class="container">
    <a-alert
        message="使用说明"
        description="本软件为uTools插件《摸鱼阅读》的辅助工具，致力于解决在使用摸鱼阅读的全局快捷键时会闪uTools输入框的问题。
        只需要在本软件设置好相应的快捷键，就可以在《摸鱼阅读》插件的阅读窗口打开的，使用本软件的快捷键来进行阅读窗口的显示、隐藏、翻页等操作。"
        type="info" show-icon
    />

    <a-row style="margin-top: 2rem;">
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="关闭阅读器" /></a-col>
      <a-col :span="8"><a-input v-model:value="value" placeholder="Basic usage" class="key-input" @focus="inputFocus" /></a-col>
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="显示/隐藏阅读器" /></a-col>
      <a-col :span="8"><a-input v-model:value="value" placeholder="Basic usage" class="key-input" /></a-col>
    </a-row>

    <a-row style="margin-top: 2rem;">
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="上一页" /></a-col>
      <a-col :span="8"><a-input v-model:value="value" placeholder="Basic usage" class="key-input" /></a-col>
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="下一页" /></a-col>
      <a-col :span="8"><a-input v-model:value="value" placeholder="Basic usage" class="key-input" /></a-col>
    </a-row>

    <a-row style="margin-top: 2rem;">
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="启/停自动翻页" /></a-col>
      <a-col :span="8"><a-input v-model:value="value" placeholder="Basic usage" class="key-input" /></a-col>
    </a-row>
  </div>
</template>

<style scoped>
.container{
  padding: 2rem;
}
.container .label{
  line-height: 32px;
  text-indent: 6px;
}
.container .key-input{
  background:unset;
}
</style>
