<template>
  <div class="h-screen bg-white dark:bg-gray-900 flex flex-col overflow-hidden" style="-webkit-app-region: drag;">
    <!-- Header -->
    <header class="flex items-center justify-between px-6 flex-shrink-0" style="height: 64px;">
      <h1 class="text-2xl font-bold text-black dark:text-gray-100">Todo</h1>
      <div class="flex items-center gap-4" style="-webkit-app-region: no-drag;">
        <button class="text-gray-400 hover:text-gray-600 transition-colors" @click="showSettings = true">
          <Settings :size="20" />
        </button>
        <button class="text-gray-400 hover:text-gray-600 transition-colors" @click="toggleCollapse">
          <ChevronUp v-if="collapseAll" :size="20" />
          <ChevronDown v-else :size="20" />
        </button>
        <button class="text-gray-400 hover:text-gray-600 transition-colors" @click="handleClose">
          <X :size="20" />
        </button>
      </div>
    </header>

    <Transition name="content">
      <div v-if="!collapseAll" class="flex-1 overflow-y-auto px-6 scrollbar-hide" style="-webkit-app-region: no-drag;" @click="closeEdit">
        <div class="mb-8">
          <input
            ref="inputRef"
            v-model="newTodo"
            type="text"
            placeholder="输入内容，按enter完成"
            class="w-full px-5 py-3 rounded-xl border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 text-sm text-black dark:text-gray-100 placeholder-gray-400 dark:placeholder-gray-500 outline-none focus:border-gray-300 dark:focus:border-gray-600 transition-colors"
            @keyup.enter="addTodo"
          />
        </div>

        <div v-if="allEmpty" class="flex flex-col items-center justify-center py-20 text-center">
          <div class="text-4xl mb-4">📋</div>
          <p class="text-sm text-gray-400 dark:text-gray-500">还没有待办事项</p>
          <p class="text-xs text-gray-300 dark:text-gray-600 mt-1">在上方输入框添加你的第一个任务吧</p>
        </div>

        <!-- Active Todo List -->
        <div v-if="activeTodos.length > 0" class="mb-6">
        <TransitionGroup name="list" tag="div">
          <div v-for="item in activeTodos" :key="item.id"
            :class="{
              'opacity-30': dragState.id === item.id,
              'border-t-2 border-blue-400 dark:border-blue-500': dragState.id && dragState.id !== item.id && dragState.targetIdx === activeTodos.indexOf(item)
            }">
            <div
              class="flex items-center gap-3 py-3 cursor-grab group"
              style="min-height: 44px;"
              :class="{
                'px-2 -mx-2 rounded-lg bg-gray-50 dark:bg-gray-800': editingId === item.id
              }"
              @mousedown.prevent="onItemMouseDown($event, item)"
              @click.stop="enterEdit(item.id)"
            >
              <div class="flex items-center justify-center" style="width: 20px; height: 20px;">
                <button @click.stop="toggleDone(item.id)" class="p-0 bg-transparent border-0 leading-none" style="width: 20px; height: 20px; display: flex; align-items: center; justify-content: center;">
                  <CheckSquare v-if="completingIds.has(item.id)" :size="20" class="text-green-400" />
                  <Square v-else :size="20" class="text-gray-300 hover:text-gray-400 transition-colors" />
                </button>
              </div>
              <template v-if="editingId === item.id">
                <input ref="editInputRefs" v-model="item.text" class="text-sm text-black dark:text-gray-100 leading-5 bg-transparent outline-none" style="flex: 1; min-width: 0;" @click.stop @keyup.enter="closeEdit" @keyup.esc="closeEdit" />
              </template>
              <span v-else class="text-sm text-black dark:text-gray-100 leading-5 truncate" style="flex: 1; min-width: 0;">{{ item.text }}</span>

              <!-- Date picker button -->
              <button
                class="p-0 bg-transparent border-0 leading-none flex items-center justify-center text-gray-300 hover:text-gray-500 dark:hover:text-gray-400 transition-colors opacity-0 group-hover:opacity-100 flex-shrink-0"
                :class="{ 'opacity-100 text-blue-500': datePickerId === item.id }"
                style="width: 18px; height: 18px;"
                @click.stop="openDatePicker($event, item)"
              >
                <Calendar :size="15" />
              </button>
              <span
                v-if="getBadge(item) && editingId !== item.id"
                class="px-2 py-0.5 rounded-md text-xs font-medium bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-300 whitespace-nowrap"
              >{{ getBadge(item) }}</span>
              <template v-if="editingId === item.id">
                <button class="text-gray-400 hover:text-gray-600 transition-colors p-0 bg-transparent border-0 leading-none" :class="{ 'text-gray-600': expandedId === item.id }" style="width: 18px; height: 18px; display: flex; align-items: center; justify-content: center;" @click.stop="expandedId = expandedId === item.id ? null : item.id">
                  <ChevronDown :size="18" class="transition-transform" :class="{ 'rotate-180': expandedId === item.id }" />
                </button>
                <button @click.stop="deleteTodo(item.id)" class="text-gray-400 hover:text-red-500 transition-colors p-0 bg-transparent border-0 leading-none" style="width: 18px; height: 18px; display: flex; align-items: center; justify-content: center;">
                  <X :size="18" />
                </button>
              </template>
            </div>
            <!-- Expanded Notes -->
            <Transition name="notes">
              <div v-if="expandedId === item.id" class="pl-8 pb-3 pr-3">
                <textarea v-model="item.notes" placeholder="添加备注..." rows="2" class="w-full text-sm text-gray-600 dark:text-gray-300 bg-gray-50 dark:bg-gray-800 rounded-lg px-3 py-2 outline-none resize-none placeholder-gray-300 dark:placeholder-gray-600" @click.stop></textarea>
              </div>
            </Transition>
          </div>
        </TransitionGroup>
        </div>

        <!-- Done List -->
        <div v-if="doneTodos.length > 0" class="border-t border-gray-100 dark:border-gray-800 pt-4 pb-8">
          <button @click="showDone = !showDone" class="flex items-center gap-1 text-gray-400 hover:text-gray-500 transition-colors mb-3">
            <ChevronDown :size="16" class="transition-transform" :class="{ 'rotate-[-90deg]': !showDone }" />
            <span class="text-xs font-medium">Done</span>
          </button>
          <Transition name="done">
            <TransitionGroup v-if="showDone" name="list" tag="div">
              <div v-for="item in doneTodos" :key="item.id" class="flex items-center gap-3 py-2.5 group">
                <div class="flex items-center justify-center" style="width: 20px; height: 20px;">
                  <button @click="toggleDone(item.id)" class="p-0 bg-transparent border-0 leading-none" style="width: 20px; height: 20px; display: flex; align-items: center; justify-content: center;">
                    <CheckSquare :size="20" class="text-gray-300 hover:text-gray-400 transition-colors" />
                  </button>
                </div>
                <span class="text-sm text-gray-400 dark:text-gray-500 line-through leading-5 truncate" style="flex: 1; min-width: 0;">{{ item.text }}</span>
                <button @click="deleteTodo(item.id)" class="p-0 bg-transparent border-0 leading-none flex items-center justify-center text-gray-300 hover:text-red-400 transition-colors opacity-0 group-hover:opacity-100" style="width: 16px; height: 16px;">
                  <X :size="16" />
                </button>
              </div>
            </TransitionGroup>
          </Transition>
        </div>
      </div>
    </Transition>

    <!-- Undo Snackbar -->
    <Teleport to="body">
      <Transition name="toast">
        <div v-if="pendingDelete" class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 flex items-center gap-3 bg-gray-800 text-white text-sm rounded-xl px-5 py-3 shadow-lg">
          <span class="truncate max-w-48">已删除 "{{ pendingDelete.text }}"</span>
          <button class="flex-shrink-0 text-blue-300 hover:text-blue-200 font-medium transition-colors" @click="undoDelete">撤销</button>
        </div>
      </Transition>
    </Teleport>

    <!-- Settings Modal -->
    <Teleport to="body">
      <Transition name="modal">
        <div v-if="showSettings" class="fixed inset-0 bg-black/20 flex items-center justify-center z-50" @click.self="showSettings = false">
          <div class="bg-white dark:bg-gray-800 rounded-2xl shadow-lg p-6 w-72">
            <div class="flex items-center justify-between mb-5">
              <h2 class="text-base font-bold text-black dark:text-gray-100">设置</h2>
              <button class="text-gray-400 hover:text-gray-600 transition-colors" @click="showSettings = false"><X :size="18" /></button>
            </div>
            <div class="flex flex-col gap-4">
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-300">深色模式</span>
                <button class="relative w-10 h-5 rounded-full transition-colors" :class="darkMode ? 'bg-blue-500' : 'bg-gray-300 dark:bg-gray-600'" @click="darkMode = !darkMode">
                  <span class="absolute top-0.5 left-0.5 w-4 h-4 rounded-full bg-white shadow transition-transform" :class="darkMode ? 'translate-x-5' : ''"></span>
                </button>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-300">开启通知提醒</span>
                <button class="relative w-10 h-5 rounded-full transition-colors" :class="notifySettings.enabled ? 'bg-blue-500' : 'bg-gray-300 dark:bg-gray-600'" @click="notifySettings.enabled = !notifySettings.enabled">
                  <span class="absolute top-0.5 left-0.5 w-4 h-4 rounded-full bg-white shadow transition-transform" :class="notifySettings.enabled ? 'translate-x-5' : ''"></span>
                </button>
              </div>
              <div v-if="notifySettings.enabled" class="flex flex-col gap-1.5 pl-1">
                <label v-for="d in [0, 1, 2, 3]" :key="d" class="flex items-center gap-2 cursor-pointer">
                  <input type="checkbox" :checked="notifySettings.days.includes(d)" class="w-3.5 h-3.5 rounded border-gray-300 dark:border-gray-600 text-blue-500 focus:ring-0" @change="(e) => { if (e.target.checked) { notifySettings.days = [...notifySettings.days, d].sort() } else { notifySettings.days = notifySettings.days.filter(x => x !== d) } }" />
                  <span class="text-xs text-gray-500 dark:text-gray-400">{{ d === 0 ? '当天到期时提醒' : d === 1 ? '提前1天提醒' : `提前${d}天提醒` }}</span>
                </label>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-300">导出数据</span>
                <button class="px-3 py-1.5 text-xs rounded-lg bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-300 hover:bg-blue-100 dark:hover:bg-blue-900/50 transition-colors" @click="handleExport">导出</button>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-300">导入数据</span>
                <button class="px-3 py-1.5 text-xs rounded-lg bg-green-50 dark:bg-green-900/30 text-green-600 dark:text-green-300 hover:bg-green-100 dark:hover:bg-green-900/50 transition-colors" @click="handleImport">导入</button>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-300">清空所有数据</span>
                <button class="px-3 py-1.5 text-xs rounded-lg bg-red-50 text-red-600 hover:bg-red-100 transition-colors" @click="clearAll">清空</button>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-300">只清空已完成</span>
                <button class="px-3 py-1.5 text-xs rounded-lg bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors" @click="clearDone">清空</button>
              </div>
            </div>
            <div class="mt-5 pt-4 border-t border-gray-100 dark:border-gray-700">
              <p class="text-xs text-gray-400 dark:text-gray-500 mb-2 font-medium">快捷键</p>
              <div class="flex flex-col gap-1.5">
                <div class="flex items-center justify-between text-xs"><span class="text-gray-500 dark:text-gray-400">新建任务</span><kbd class="px-1.5 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 font-mono">Ctrl+N</kbd></div>
                <div class="flex items-center justify-between text-xs"><span class="text-gray-500 dark:text-gray-400">关闭面板</span><kbd class="px-1.5 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 font-mono">Esc</kbd></div>
                <div class="flex items-center justify-between text-xs"><span class="text-gray-500 dark:text-gray-400">确认编辑</span><kbd class="px-1.5 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 font-mono">Enter</kbd></div>
              </div>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <!-- Date Picker Popup (teleported to avoid overflow clipping) -->
    <Teleport to="body">
      <div
        v-if="datePickerId"
        class="fixed z-50 bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-3"
        :style="{ top: pickerPos.y + 'px', left: pickerPos.x + 'px' }"
        @click.stop
      >
        <input
          type="date"
          :value="pickerItem?.dueDate"
          :min="todayStr"
          class="text-sm w-36 text-black dark:text-gray-100 bg-gray-50 dark:bg-gray-700 rounded-lg px-3 py-1.5 outline-none appearance-none"
          @change="(e) => { if (pickerItem) pickerItem.dueDate = e.target.value; datePickerId = null }"
        />
        <button
          v-if="pickerItem?.dueDate"
          class="block mt-1 text-xs text-red-400 hover:text-red-500 transition-colors"
          @click="pickerItem.dueDate = null; datePickerId = null"
        >
          清除日期
        </button>
      </div>
    </Teleport>
  </div>
</template>

<script setup>
import { ref, computed, watch, nextTick, onMounted, onUnmounted } from 'vue'
import { Settings, ChevronUp, ChevronDown, X, Square, CheckSquare, Calendar } from '@lucide/vue'
import { invoke } from '@tauri-apps/api/core'
import { sendNotification } from '@tauri-apps/plugin-notification'

const STORAGE_KEY = 'todo-app-data'

const newTodo = ref('')
const collapseAll = ref(false)
const showDone = ref(true)
const editingId = ref(null)
const expandedId = ref(null)
const datePickerId = ref(null)
const pickerPos = ref({ x: 0, y: 0 })
const pickerItem = computed(() => activeTodos.value.find(t => t.id === datePickerId.value))
const showSettings = ref(false)
const inputRef = ref(null)
const editInputRefs = ref([])
const pendingDelete = ref(null)
const completingIds = ref(new Set())
const dragState = ref({ id: null, startY: 0, idx: -1, targetIdx: -1 })
const darkMode = ref(localStorage.getItem('todo-dark-mode') === 'true')
const notifySettings = ref(JSON.parse(localStorage.getItem('todo-notify-settings') || '{"enabled":true,"days":[1]}'))
const todayRef = ref(new Date().toDateString())
let deleteTimer = null
let midnightTimer = null
let notifyTimer = null

function loadTodos() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (raw) {
      const data = JSON.parse(raw)
      return data
    }
  } catch (e) { /* ignore */ }
  return [
    { id: 1, text: '参加团队周例会', done: false, notes: '', dueDate: null },
    { id: 2, text: '小组聚会', done: false, notes: '', dueDate: null },
    { id: 3, text: '整理好PPT', done: false, notes: '', dueDate: null },
    { id: 4, text: '晨跑5公里', done: true, notes: '', dueDate: null },
    { id: 5, text: '看个电影', done: true, notes: '', dueDate: null },
    { id: 6, text: '打球', done: true, notes: '', dueDate: null },
  ]
}

const todos = ref(loadTodos())

const activeTodos = computed(() => todos.value.filter(t => !t.done))
const doneTodos = computed(() => todos.value.filter(t => t.done))
const allEmpty = computed(() => activeTodos.value.length === 0 && doneTodos.value.length === 0)
const todayStr = computed(() => new Date().toISOString().split('T')[0])

watch(todos, () => {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(todos.value))
}, { deep: true })

watch(darkMode, (val) => {
  localStorage.setItem('todo-dark-mode', val)
  document.documentElement.classList.toggle('dark', val)
}, { immediate: true })

watch(notifySettings, (val) => {
  localStorage.setItem('todo-notify-settings', JSON.stringify(val))
}, { deep: true })

function toggleCollapse() {
  collapseAll.value = !collapseAll.value
  nextTick(() => {
    if (!collapseAll.value) inputRef.value?.focus()
    invoke('resize_window', { width: 390, height: collapseAll.value ? 64 : 790 })
  })
}

function addTodo() {
  const text = newTodo.value.trim()
  if (!text) return
  todos.value.unshift({ id: Date.now(), text, done: false, notes: '', dueDate: null })
  newTodo.value = ''
}

function toggleDone(id) {
  const idx = todos.value.findIndex(t => t.id === id)
  if (idx === -1) return
  const todo = todos.value[idx]

  if (!todo.done) {
    completingIds.value = new Set([...completingIds.value, id])
    todos.value.splice(idx, 1, { ...todo, done: true })
    completingIds.value = new Set([...completingIds.value].filter(x => x !== id))
  } else {
    todo.done = false
  }
}

function enterEdit(id) {
  editingId.value = id
  expandedId.value = null
  nextTick(() => {
    const inputs = editInputRefs.value
    if (inputs && inputs.length) {
      const el = Array.isArray(inputs) ? inputs[inputs.length - 1] : inputs
      el?.focus()
    }
  })
}

function closeEdit() {
  editingId.value = null
  expandedId.value = null
  datePickerId.value = null
}

function openDatePicker(event, item) {
  if (datePickerId.value === item.id) {
    datePickerId.value = null
    return
  }
  const rect = event.currentTarget.getBoundingClientRect()
  pickerPos.value = { x: rect.right - 200, y: rect.bottom + 4 }
  datePickerId.value = item.id
}

function onItemMouseDown(e, item) {
  const list = activeTodos.value
  const idx = list.indexOf(item)
  if (idx === -1) return
  dragState.value = { id: item.id, startY: e.clientY, idx, targetIdx: idx }
  document.addEventListener('mousemove', onGlobalMouseMove)
  document.addEventListener('mouseup', onGlobalMouseUp)
}

function onGlobalMouseMove(e) {
  const s = dragState.value
  if (!s.id) return
  const dy = e.clientY - s.startY
  const rowHeight = 56 // item 44px + gap 12px
  const newIdx = Math.round(s.idx + dy / rowHeight)
  const clamped = Math.max(0, Math.min(activeTodos.value.length - 1, newIdx))
  if (clamped !== s.targetIdx) {
    dragState.value = { ...s, targetIdx: clamped }
  }
}

function onGlobalMouseUp() {
  document.removeEventListener('mousemove', onGlobalMouseMove)
  document.removeEventListener('mouseup', onGlobalMouseUp)
  const s = dragState.value
  if (!s.id || s.targetIdx === s.idx) {
    dragState.value = { id: null, startY: 0, idx: -1, targetIdx: -1 }
    return
  }
  const fromAll = todos.value.findIndex(t => t.id === s.id)
  if (fromAll === -1) { dragState.value = { id: null, startY: 0, idx: -1, targetIdx: -1 }; return }
  const [moved] = todos.value.splice(fromAll, 1)
  const targetItem = activeTodos.value[s.targetIdx]
  const toAll = targetItem ? todos.value.indexOf(targetItem) : todos.value.length
  todos.value.splice(toAll, 0, moved)
  dragState.value = { id: null, startY: 0, idx: -1, targetIdx: -1 }
}

function onKeydown(e) {
  if (e.key === 'Escape') { closeEdit(); showSettings.value = false }
  if ((e.ctrlKey || e.metaKey) && (e.key === 'n' || e.key === 'N')) {
    e.preventDefault()
    if (collapseAll.value) toggleCollapse()
    inputRef.value?.focus()
  }
}

function scheduleMidnightRefresh() {
  const now = new Date()
  const tomorrow = new Date(now.getFullYear(), now.getMonth(), now.getDate() + 1)
  const ms = tomorrow.getTime() - now.getTime()
  midnightTimer = setTimeout(() => { todayRef.value = new Date().toDateString(); scheduleMidnightRefresh() }, ms)
}

function getDaysLeft(item) {
  if (!item.dueDate) return -1
  void todayRef.value
  const target = new Date(item.dueDate + 'T00:00:00')
  const today = new Date(new Date().toISOString().split('T')[0] + 'T00:00:00')
  return Math.ceil((target.getTime() - today.getTime()) / (1000 * 60 * 60 * 24))
}

function getBadge(item) {
  if (!item.dueDate) return null
  void todayRef.value
  const diff = getDaysLeft(item)
  if (diff < 0) return '已过期'
  if (diff === 0) return '今天'
  if (diff === 1) return '明天'
  return `还有${diff}天`
}

function checkNotifications() {
  const settings = notifySettings.value
  if (!settings.enabled) return
  const groups = {}
  for (const item of activeTodos.value) {
    const days = getDaysLeft(item)
    if (days < 0 || !settings.days.includes(days)) continue
    if (!groups[days]) groups[days] = []
    groups[days].push(item.text)
  }
  const parts = []
  const sorted = Object.keys(groups).map(Number).sort()
  for (const d of sorted) {
    const label = d === 0 ? '今日' : d === 1 ? '明日' : `${d}天后`
    parts.push(`${label}：${groups[d].slice(0, 3).join('、')}${groups[d].length > 3 ? '等' + groups[d].length + '项' : ''}`)
  }
  if (parts.length > 0) {
    sendNotification({ title: 'Todo 提醒', body: parts.join('；') })
  }
}

function startNotifyTimer() {
  checkNotifications()
  notifyTimer = setInterval(checkNotifications, 30 * 60 * 1000)
}

onMounted(() => {
  document.addEventListener('keydown', onKeydown)
  scheduleMidnightRefresh()
  startNotifyTimer()
})

onUnmounted(() => {
  document.removeEventListener('keydown', onKeydown)
  clearTimeout(midnightTimer)
  clearInterval(notifyTimer)
  clearTimeout(deleteTimer)
})

function deleteTodo(id) {
  const item = todos.value.find(t => t.id === id)
  if (!item) return
  todos.value = todos.value.filter(t => t.id !== id)
  if (editingId.value === id) editingId.value = null
  if (expandedId.value === id) expandedId.value = null
  clearTimeout(deleteTimer)
  pendingDelete.value = { id: item.id, text: item.text, data: item }
  deleteTimer = setTimeout(() => { pendingDelete.value = null }, 3000)
}

function undoDelete() {
  if (!pendingDelete.value) return
  const restored = pendingDelete.value.data
  todos.value.unshift(restored)
  clearTimeout(deleteTimer)
  pendingDelete.value = null
}

function clearAll() { todos.value = []; showSettings.value = false }
function clearDone() { todos.value = todos.value.filter(t => !t.done); showSettings.value = false }

async function handleExport() {
  const json = JSON.stringify(todos.value, null, 2)
  await invoke('export_data', { data: json })
}

async function handleImport() {
  try {
    const data = await invoke('import_data')
    const parsed = JSON.parse(data)
    if (Array.isArray(parsed)) todos.value = parsed
  } catch (e) { /* ignore */ }
}

function handleClose() {
  invoke('hide_window')
}


</script>

<style scoped>
.content-enter-active,.content-leave-active { transition: opacity 0.2s ease, transform 0.2s ease; }
.content-enter-from { opacity: 0; transform: translateY(-8px); }
.content-leave-to { opacity: 0; transform: translateY(-8px); }
.list-enter-active,.list-leave-active { transition: all 0.25s ease; }
.list-enter-from { opacity: 0; transform: translateY(-12px); }
.list-leave-to { opacity: 0; transform: translateX(12px); }
.list-move { transition: transform 0.25s ease; }
.notes-enter-active,.notes-leave-active { transition: all 0.2s ease; }
.notes-enter-from,.notes-leave-to { opacity: 0; max-height: 0; padding-top: 0; padding-bottom: 0; overflow: hidden; }
.toast-enter-active,.toast-leave-active { transition: all 0.25s ease; }
.toast-enter-from,.toast-leave-to { opacity: 0; transform: translate(-50%, 12px); }
.modal-enter-active,.modal-leave-active { transition: opacity 0.2s ease; }
.modal-enter-from,.modal-leave-to { opacity: 0; }
.done-enter-active,.done-leave-active { transition: all 0.25s ease; }
.done-enter-from { opacity: 0; max-height: 0; overflow: hidden; }
.done-leave-to { opacity: 0; max-height: 0; overflow: hidden; }
.scrollbar-hide::-webkit-scrollbar { display: none; }
.scrollbar-hide { -ms-overflow-style: none; scrollbar-width: none; }
</style>
