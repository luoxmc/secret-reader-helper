<script setup>
import { register,unregister } from '@tauri-apps/api/globalShortcut';
import {h, ref} from "vue"
import { invoke } from "@tauri-apps/api/tauri";
import {ClearOutlined} from "@ant-design/icons-vue";

let closeReader = ref(localStorage.getItem('short-cut-closeReader'))
let toggleReader = ref(localStorage.getItem('short-cut-toggleReader'))
let prevPage = ref(localStorage.getItem('short-cut-prevPage'))
let nextPage = ref(localStorage.getItem('short-cut-nextPage'))
let toggleAuto = ref(localStorage.getItem('short-cut-toggleAuto'))
let toggleMove = ref(localStorage.getItem('short-cut-toggleMove'))

const isWin = (navigator.platform === "Win32") || (navigator.platform === "Windows")
const isMac = (navigator.platform === "Mac68K") || (navigator.platform === "MacPPC") || (navigator.platform === "Macintosh") || (navigator.platform === "MacIntel")

initKeyListener()

async function initKeyListener() {
  let storage = localStorage
  if (storage && storage.length > 0) {
    for (let i = 0; i < storage.length; i++) {
      let keyName = storage.key(i)
      if (keyName.startsWith('short-cut')) {
        let keyValue = localStorage.getItem(keyName)
        if (!keyValue) {
          continue
        }
        let sendMsg = 'type='
        if (keyName === 'short-cut-closeReader') {
          sendMsg += '1'
        } else if (keyName === 'short-cut-toggleReader') {
          sendMsg += '2'
        } else if (keyName === 'short-cut-prevPage') {
          sendMsg += '4&content=prev'
        } else if (keyName === 'short-cut-nextPage') {
          sendMsg += '4&content=next'
        } else if (keyName === 'short-cut-toggleAuto') {
          sendMsg += '3'
        } else if (keyName === 'short-cut-toggleMove') {
          sendMsg += '5'
        }
        if (sendMsg === 'type=') {
          continue
        }
        await register(keyValue, async () => {
          console.log("key trigger:",sendMsg)
          await invoke("send_tcp_msg", {msg: sendMsg})
        });
      }
    }
  }
}

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
  } else if (focusId === 'toggleMove') {
    toggleMove.value = connect
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
    } else if (tmpFocusId === 'toggleMove') {
      toggleMove.value = tmpBeforeValue
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
    } else if (tmpFocusId === 'toggleMove') {
      sendMsg += '5'
    }
    await register(curValue, async () => {
      console.log("key trigger:",sendMsg)
      await invoke("send_tcp_msg", {msg: sendMsg})
    });
    localStorage.setItem(key, curValue)
  }
}

// 清除快捷键
async function clearKey(checkId) {
  let key = 'short-cut-' + checkId
  let shortCut = localStorage.getItem(key)
  if (shortCut) {
    await unregister(shortCut)
    localStorage.removeItem(key)
    if (checkId === 'closeReader') {
      closeReader.value = ''
    } else if (checkId === 'toggleReader') {
      toggleReader.value = ''
    } else if (checkId === 'prevPage') {
      prevPage.value = ''
    } else if (checkId === 'nextPage') {
      nextPage.value = ''
    } else if (checkId === 'toggleAuto') {
      toggleAuto.value = ''
    } else if (checkId === 'toggleMove') {
      toggleMove.value = ''
    }
  }
}

function  clearAllKey() {
  let ids = ['closeReader','toggleReader','prevPage','nextPage','toggleAuto', 'toggleMove']
  for (let i = 0; i < ids.length; i++) {
    clearKey(ids[i])
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

    <div style="margin-top: 2rem;">
      <a-button style="float: right; margin-right: 2rem" :icon="h(ClearOutlined)" @click="clearAllKey">清除所有快捷键</a-button>
      <div style="clear: both"></div>
    </div>

    <a-row style="margin-top: 2rem;">
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="关闭阅读器" /></a-col>
      <a-col :span="6">
        <a-input v-model:value="closeReader" placeholder="请点击后录入快捷键" class="key-input" id="closeReader"
                 @focus="inputFocus" @blur="inputBlur" readOnly />
      </a-col>
      <a-col :span="2"><a-button :icon="h(ClearOutlined)" @click="clearKey('closeReader')"></a-button></a-col>
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="显示/隐藏阅读器" /></a-col>
      <a-col :span="6">
        <a-input v-model:value="toggleReader" placeholder="请点击后录入快捷键" class="key-input" id="toggleReader"
                 @focus="inputFocus" @blur="inputBlur" readOnly />
      </a-col>
      <a-col :span="2"><a-button :icon="h(ClearOutlined)" @click="clearKey('toggleReader')"></a-button></a-col>
    </a-row>

    <a-row style="margin-top: 2rem;">
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="上一页" @focus="inputFocus" /></a-col>
      <a-col :span="6">
        <a-input v-model:value="prevPage" placeholder="请点击后录入快捷键" class="key-input" id="prevPage"
                 @focus="inputFocus" @blur="inputBlur" readOnly />
      </a-col>
      <a-col :span="2"><a-button :icon="h(ClearOutlined)" @click="clearKey('prevPage')"></a-button></a-col>
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="下一页" @focus="inputFocus" /></a-col>
      <a-col :span="6">
        <a-input v-model:value="nextPage" placeholder="请点击后录入快捷键" class="key-input" id="nextPage"
                 @focus="inputFocus" @blur="inputBlur" readOnly />
      </a-col>
      <a-col :span="2"><a-button :icon="h(ClearOutlined)" @click="clearKey('nextPage')"></a-button></a-col>
    </a-row>

    <a-row style="margin-top: 2rem;">
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="启/停自动翻页" /></a-col>
      <a-col :span="6">
        <a-input v-model:value="toggleAuto" placeholder="请点击后录入快捷键" class="key-input" id="toggleAuto"
                 @focus="inputFocus" @blur="inputBlur" readOnly />
      </a-col>
      <a-col :span="2"><a-button :icon="h(ClearOutlined)" @click="clearKey('toggleAuto')"></a-button></a-col>
      <a-col :span="4" class="label"><a-typography-text strong :ellipsis="true" content="开/关窗口移动" /></a-col>
      <a-col :span="6">
        <a-input v-model:value="toggleMove" placeholder="请点击后录入快捷键" class="key-input" id="toggleMove"
                 @focus="inputFocus" @blur="inputBlur" readOnly />
      </a-col>
      <a-col :span="2"><a-button :icon="h(ClearOutlined)" @click="clearKey('toggleMove')"></a-button></a-col>
    </a-row>

  </div>
</template>

<style scoped>
.container{
  padding: 2rem;
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  -khtml-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}
.container .label{
  line-height: 32px;
  text-indent: 3px;
  font-size: 12px;
}
.container .key-input{
  background:unset;
  width: 90%;
}
</style>
