<script setup>
import { register,unregister } from '@tauri-apps/api/globalShortcut';
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/tauri";

let closeReader = ref(localStorage.getItem('short-cut-closeReader'))
let toggleReader = ref(localStorage.getItem('short-cut-toggleReader'))
let prevPage = ref(localStorage.getItem('short-cut-prevPage'))
let nextPage = ref(localStorage.getItem('short-cut-nextPage'))
let toggleAuto = ref(localStorage.getItem('short-cut-toggleAuto'))

const isWin = (navigator.platform === "Win32") || (navigator.platform === "Windows")
const isMac = (navigator.platform === "Mac68K") || (navigator.platform === "MacPPC") || (navigator.platform === "Macintosh") || (navigator.platform === "MacIntel")

let getCommandStr = () => {
  let btn = "Meta+"
  if(isMac){
    btn = 'Command+'
  } else if (isWin){
    btn = 'Win+'
  }
  return btn;
}
let getOptionStr = () => {
  let btn = "Alt+"
  if(isMac){
    btn = 'Option+'
  }
  return btn;
}



let focusId = ''
let beforeValue = ''
let keyFunc = (e) => {
  console.log('keydown',focusId)
  let ctrl = e.ctrlKey,shift = e.shiftKey,alt = e.altKey,meta = e.metaKey;
  let connect = (meta ? getCommandStr() : "" ) +  (ctrl ? "Control+" : "" )
      + (alt ? getOptionStr() : "" ) + (shift ? "Shift+" : "" )
  if(e.key !== 'Meta' && e.key !== 'Control' && e.key !== 'Alt' && e.key !== 'Shift'){
    let code = e.code
    if (code.indexOf('Key') !== -1) {
      code = code.replace('Key','')
    }
    connect = connect + code;
  }
  if (focusId === 'closeReader') {
    closeReader.value = connect
  } else if (focusId === 'toggleReader') {
    toggleReader.value = connect
  } else if (focusId === 'prevPage') {
    prevPage.value = connect
  } else if (focusId === 'nextPage') {
    nextPage.value = connect
  } else if (focusId === 'toggleAuto') {
    toggleAuto.value = connect
  }
  e.preventDefault()
}

function inputFocus (e)  {
  console.log('focus', e)
  setTimeout(() => {
    focusId = e.target.id
    beforeValue = e.target._value
    document.addEventListener('keydown', keyFunc)
  },40)
}
async function inputBlur(e) {
  console.log('blur',e)
  document.removeEventListener('keydown', keyFunc)
  let tmpFocusId = focusId
  let tmpBeforeValue = beforeValue
  focusId = ''
  beforeValue = ''
  let curValue = e.target._value
  if (!curValue || curValue.endsWith('+')) {
    if (tmpFocusId === 'closeReader') {
      closeReader.value = tmpBeforeValue
    } else if (tmpFocusId === 'toggleReader') {
      toggleReader.value = tmpBeforeValue
    } else if (tmpFocusId === 'prevPage') {
      prevPage.value = tmpBeforeValue
    } else if (tmpFocusId === 'nextPage') {
      nextPage.value = tmpBeforeValue
    } else if (tmpFocusId === 'toggleAuto') {
      toggleAuto.value = tmpBeforeValue
    }
  } else if (curValue === tmpBeforeValue) {

  } else {
    let key = 'short-cut-' + tmpFocusId
    let shortCut = localStorage.getItem(key)
    if (shortCut) {
      await unregister(shortCut)
    }
    let sendMsg = 'type='
    if (tmpFocusId === 'closeReader') {
      sendMsg += '1'
    } else if (tmpFocusId === 'toggleReader') {
      sendMsg += '2'
    } else if (tmpFocusId === 'prevPage') {
      sendMsg += '4&content=prev'
    } else if (tmpFocusId === 'nextPage') {
      sendMsg += '4&content=next'
    } else if (tmpFocusId === 'toggleAuto') {
      sendMsg += '3'
    }
    await register(curValue, async () => {
      console.log("key trigger:",sendMsg)
      await invoke("send_tcp_msg", {msg: sendMsg})
    });
    localStorage.setItem(key, curValue)
  }

}

</script>

<template>
  <div class="container">
    <a-alert
        message="使用说明"
        description="本软件为uTools插件《摸鱼阅读》的辅助工具，致力于解决在使用摸鱼阅读的全局快捷键时会闪uTools输入框的问题。
        只需要在本软件设置好相应的快捷键，就可以在《摸鱼阅读》插件的阅读窗口打开时，使用本软件的快捷键来进行阅读窗口的显示、隐藏、翻页等操作。"
        type="info" show-icon
    />

    <a-row style="margin-top: 2rem;">
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="关闭阅读器" /></a-col>
      <a-col :span="8">
        <a-input v-model:value="closeReader" placeholder="请点击后录入快捷键" class="key-input" id="closeReader"
                 @focus="inputFocus" @blur="inputBlur" readOnly />
      </a-col>
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="显示/隐藏阅读器" /></a-col>
      <a-col :span="8">
        <a-input v-model:value="toggleReader" placeholder="请点击后录入快捷键" class="key-input" id="toggleReader"
                 @focus="inputFocus" @blur="inputBlur" readOnly />
      </a-col>
    </a-row>

    <a-row style="margin-top: 2rem;">
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="上一页" @focus="inputFocus" /></a-col>
      <a-col :span="8">
        <a-input v-model:value="prevPage" placeholder="请点击后录入快捷键" class="key-input" id="prevPage"
                 @focus="inputFocus" @blur="inputBlur" readOnly />
      </a-col>
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="下一页" @focus="inputFocus" /></a-col>
      <a-col :span="8">
        <a-input v-model:value="nextPage" placeholder="请点击后录入快捷键" class="key-input" id="nextPage"
                 @focus="inputFocus" @blur="inputBlur" readOnly />
      </a-col>
    </a-row>

    <a-row style="margin-top: 2rem;">
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="启/停自动翻页" /></a-col>
      <a-col :span="8">
        <a-input v-model:value="toggleAuto" placeholder="请点击后录入快捷键" class="key-input" id="toggleAuto"
                 @focus="inputFocus" @blur="inputBlur" readOnly />
      </a-col>
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
