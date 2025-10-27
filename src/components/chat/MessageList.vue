<script setup lang="ts">
import MessageItem from './MessageItem.vue'

interface Message {
  id: string
  role: 'user' | 'assistant'
  content: string
  timestamp?: number
  sources?: string[]
}

const props = defineProps<{
  messages: Message[]
  isLoading?: boolean
}>()

// 滚动到底部
const messagesContainer = ref<HTMLElement | null>(null)

function scrollToBottom() {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
    }
  })
}

// 监听消息变化，自动滚动
watch(() => props.messages, () => {
  scrollToBottom()
}, { deep: true })

onMounted(() => {
  scrollToBottom()
})
</script>

<template>
  <div 
    ref="messagesContainer"
    class="flex-1 overflow-y-auto scroll-smooth"
  >
    <!-- 空状态 -->
    <div v-if="messages.length === 0" class="flex flex-col items-center justify-center h-full px-6 text-center">
      <div class="w-20 h-20 rounded-3xl bg-gradient-to-br from-primary/20 via-purple-500/20 to-pink-500/20 flex items-center justify-center mb-6 shadow-lg">
        <svg class="w-10 h-10 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
        </svg>
      </div>
      <h3 class="text-2xl font-bold mb-3 bg-gradient-to-r from-primary to-purple-600 bg-clip-text text-transparent">
        开始新对话
      </h3>
      <p class="text-muted-foreground max-w-md text-sm leading-relaxed">
        向 Wali AI 提问，我会基于您的知识库为您提供准确的答案
      </p>
      
      <!-- 建议问题 -->
      <div class="mt-8 grid grid-cols-1 md:grid-cols-2 gap-3 max-w-2xl w-full">
        <button class="p-4 rounded-xl border border-border/50 bg-muted/30 hover:bg-muted/60 hover:border-primary/50 transition-all text-left group">
          <p class="text-sm font-medium group-hover:text-primary transition-colors">✨ 如何使用知识库？</p>
        </button>
        <button class="p-4 rounded-xl border border-border/50 bg-muted/30 hover:bg-muted/60 hover:border-primary/50 transition-all text-left group">
          <p class="text-sm font-medium group-hover:text-primary transition-colors">📚 支持哪些文件格式？</p>
        </button>
      </div>
    </div>

    <!-- 消息列表 -->
    <div v-else class="py-4">
      <MessageItem
        v-for="message in messages"
        :key="message.id"
        :message="message"
      />

      <!-- 加载中 -->
      <div v-if="isLoading" class="flex px-6 py-4">
        <div class="flex gap-3 max-w-[85%]">
          <div class="flex-shrink-0 w-9 h-9 rounded-full bg-gradient-to-br from-purple-500 via-pink-500 to-rose-500 flex items-center justify-center shadow-md">
            <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
          </div>
          <div class="flex-1 space-y-2">
            <div class="flex items-center gap-2">
              <span class="text-sm font-semibold">Wali AI</span>
              <span class="text-xs text-muted-foreground">正在思考...</span>
            </div>
            <div class="flex gap-1.5">
              <div class="w-2 h-2 rounded-full bg-primary/70 animate-bounce [animation-delay:0ms]"></div>
              <div class="w-2 h-2 rounded-full bg-primary/70 animate-bounce [animation-delay:150ms]"></div>
              <div class="w-2 h-2 rounded-full bg-primary/70 animate-bounce [animation-delay:300ms]"></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

