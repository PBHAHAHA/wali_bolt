<script setup lang="ts">
import { User, Bot, Copy, Check } from 'lucide-vue-next'

interface Message {
  id: string
  role: 'user' | 'assistant'
  content: string
  timestamp?: number
  sources?: string[]
}

const props = defineProps<{
  message: Message
}>()

// 复制消息
const copied = ref(false)
async function copyMessage() {
  await navigator.clipboard.writeText(props.message.content)
  copied.value = true
  setTimeout(() => {
    copied.value = false
  }, 2000)
}

// 格式化时间
function formatTime(timestamp?: number) {
  if (!timestamp) return ''
  const date = new Date(timestamp)
  return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
}
</script>

<template>
  <!-- 用户消息 - 右对齐 -->
  <div 
    v-if="message.role === 'user'"
    class="flex justify-end px-6 py-4"
  >
    <div class="flex flex-row-reverse gap-3 max-w-[75%]">
      <!-- 头像 -->
      <div class="flex-shrink-0 w-9 h-9 rounded-full bg-gradient-to-br from-blue-500 to-primary flex items-center justify-center shadow-md">
        <User class="w-4.5 h-4.5 text-white" />
      </div>

      <!-- 消息气泡 -->
      <div class="flex flex-col items-end gap-2">
        <div class="bg-primary text-primary-foreground rounded-2xl rounded-tr-sm px-4 py-3 shadow-sm">
          <p class="text-sm whitespace-pre-wrap leading-relaxed">{{ message.content }}</p>
        </div>
        
        <!-- 时间戳 -->
        <span v-if="message.timestamp" class="text-xs text-muted-foreground px-1">
          {{ formatTime(message.timestamp) }}
        </span>
      </div>
    </div>
  </div>

  <!-- AI 消息 - 左对齐 -->
  <div 
    v-else
    class="group flex px-6 py-4 hover:bg-muted/20 transition-colors"
  >
    <div class="flex gap-3 max-w-[85%]">
      <!-- 头像 -->
      <div class="flex-shrink-0 w-9 h-9 rounded-full bg-gradient-to-br from-purple-500 via-pink-500 to-rose-500 flex items-center justify-center shadow-md">
        <Bot class="w-5 h-5 text-white" />
      </div>

      <!-- 消息内容 -->
      <div class="flex-1 min-w-0 space-y-2">
        <!-- 名称和操作 -->
        <div class="flex items-center justify-between gap-2">
          <span class="text-sm font-semibold text-foreground">Wali AI</span>
          
          <!-- 复制按钮 -->
          <button
            class="opacity-0 group-hover:opacity-100 transition-opacity p-1.5 rounded-lg hover:bg-muted"
            @click="copyMessage"
            title="复制"
          >
            <Check v-if="copied" class="w-4 h-4 text-green-500" />
            <Copy v-else class="w-4 h-4 text-muted-foreground" />
          </button>
        </div>

        <!-- 消息文本 -->
        <div class="prose prose-sm dark:prose-invert max-w-none">
          <p class="text-sm whitespace-pre-wrap leading-relaxed text-foreground/90">{{ message.content }}</p>
        </div>

        <!-- 来源引用 -->
        <div v-if="message.sources && message.sources.length > 0" class="flex flex-wrap gap-2 pt-1">
          <div
            v-for="source in message.sources"
            :key="source"
            class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg bg-muted/60 border border-border/50 text-xs text-muted-foreground hover:bg-muted transition-colors cursor-pointer"
          >
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            <span class="font-medium">{{ source }}</span>
          </div>
        </div>

        <!-- 时间戳 -->
        <span v-if="message.timestamp" class="text-xs text-muted-foreground">
          {{ formatTime(message.timestamp) }}
        </span>
      </div>
    </div>
  </div>
</template>

