<script setup lang="ts">
import { Paperclip, X, Sparkles, Mic, Send } from 'lucide-vue-next'

const props = defineProps<{
  isLoading?: boolean
  disabled?: boolean
}>()

const emit = defineEmits<{
  send: [message: string]
  stop: []
}>()

const inputText = ref('')
const textarea = ref<HTMLTextAreaElement | null>(null)
const isFocused = ref(false)

// 发送消息
function handleSend() {
  const message = inputText.value.trim()
  if (!message || props.isLoading) return
  
  emit('send', message)
  inputText.value = ''
  
  // 重置高度
  nextTick(() => {
    if (textarea.value) {
      textarea.value.style.height = 'auto'
    }
  })
}

// 停止生成
function handleStop() {
  emit('stop')
}

// 自动调整高度
function adjustHeight() {
  if (textarea.value) {
    textarea.value.style.height = 'auto'
    textarea.value.style.height = Math.min(textarea.value.scrollHeight, 120) + 'px'
  }
}

// 键盘事件
function handleKeydown(e: KeyboardEvent) {
  // Enter 发送，Shift+Enter 换行
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault()
    handleSend()
  }
}

// 焦点事件
function handleFocus() {
  isFocused.value = true
}

function handleBlur() {
  isFocused.value = false
}
</script>

<template>
  <div class="bg-background px-4 py-6">
    <div class="max-w-4xl mx-auto">
      <!-- 输入框容器 -->
      <div 
        :class="[
          'relative flex items-end gap-3 rounded-2xl border-2 bg-muted/30 px-4 py-3 transition-all duration-200',
          isFocused ? 'border-primary/50 shadow-lg shadow-primary/10' : 'border-border/50'
        ]"
      >
        <!-- 左侧附件按钮 -->
        <button
          type="button"
          class="flex-shrink-0 w-9 h-9 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted transition-colors"
          title="添加附件"
        >
          <Paperclip class="w-5 h-5" />
        </button>

        <!-- 输入框 -->
        <div class="flex-1 min-w-0">
          <textarea
            ref="textarea"
            v-model="inputText"
            :disabled="disabled"
            :placeholder="isLoading ? 'AI 正在思考中...' : '发消息或输入 / 选择技能'"
            class="w-full resize-none bg-transparent text-sm placeholder:text-muted-foreground focus:outline-none disabled:cursor-not-allowed disabled:opacity-50 min-h-[24px] max-h-[120px]"
            rows="1"
            @input="adjustHeight"
            @keydown="handleKeydown"
            @focus="handleFocus"
            @blur="handleBlur"
          />
        </div>

        <!-- 右侧操作按钮组 -->
        <div class="flex items-center gap-1.5 flex-shrink-0">
          <!-- 深度思考按钮 -->
          <button
            v-if="inputText.trim()"
            type="button"
            class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-background border border-border text-foreground hover:bg-muted transition-colors"
            title="深度思考"
          >
            <X class="w-3.5 h-3.5" />
            <span>深度思考</span>
          </button>

          <!-- AI 功能按钮 -->
          <button
            type="button"
            class="flex-shrink-0 w-9 h-9 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted transition-colors"
            title="AI 功能"
          >
            <Sparkles class="w-5 h-5" />
          </button>

          <!-- 语音输入按钮 -->
          <button
            type="button"
            class="flex-shrink-0 w-9 h-9 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted transition-colors"
            title="语音输入"
          >
            <Mic class="w-5 h-5" />
          </button>

          <!-- 发送/停止按钮 -->
          <button
            v-if="!isLoading"
            type="button"
            :disabled="!inputText.trim() || disabled"
            :class="[
              'flex-shrink-0 w-9 h-9 flex items-center justify-center rounded-lg transition-all duration-200',
              inputText.trim() && !disabled
                ? 'bg-primary text-primary-foreground hover:bg-primary/90 shadow-md'
                : 'bg-muted/50 text-muted-foreground cursor-not-allowed'
            ]"
            @click="handleSend"
          >
            <Send class="w-4.5 h-4.5" />
          </button>
          <button
            v-else
            type="button"
            class="flex-shrink-0 w-9 h-9 flex items-center justify-center rounded-lg bg-red-500 text-white hover:bg-red-600 transition-colors"
            @click="handleStop"
          >
            <X class="w-4.5 h-4.5" />
          </button>
        </div>
      </div>

    </div>
  </div>
</template>

