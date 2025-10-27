<script setup lang="ts">
import ChatHeader from '@/components/chat/ChatHeader.vue'
import MessageList from '@/components/chat/MessageList.vue'
import ChatInput from '@/components/chat/ChatInput.vue'
import * as api from '@/lib/api'

// 消息列表
const messages = ref<api.Message[]>([])

// 加载状态
const isLoading = ref(false)

// 当前对话 ID
const currentConversationId = ref<string | undefined>(undefined)

// 发送消息
async function handleSend(content: string) {
  // 添加用户消息
  const userMessage: api.Message = {
    id: Date.now().toString(),
    role: 'user',
    content,
    timestamp: Date.now()
  }
  messages.value.push(userMessage)

  // 开始加载
  isLoading.value = true

  try {
    // 调用后端 API
    const response = await api.askQuestion({
      question: content,
      conversation_id: currentConversationId.value
    })

    // 设置或更新对话 ID
    currentConversationId.value = response.conversation_id

    // 添加 AI 回复
    const aiMessage: api.Message = {
      id: (Date.now() + 1).toString(),
      role: 'assistant',
      content: response.answer,
      timestamp: Date.now(),
      sources: response.sources
    }
    messages.value.push(aiMessage)
  } catch (error) {
    console.error('发送消息失败:', error)
    
    // 显示错误消息
    const errorMessage: api.Message = {
      id: (Date.now() + 1).toString(),
      role: 'assistant',
      content: `抱歉，发生了错误：${error instanceof Error ? error.message : String(error)}`,
      timestamp: Date.now()
    }
    messages.value.push(errorMessage)
  } finally {
    isLoading.value = false
  }
}

// 停止生成
function handleStop() {
  isLoading.value = false
  console.log('停止生成')
}

// 新建对话
function handleNewChat() {
  if (messages.value.length > 0) {
    const confirmed = confirm('确定要开始新对话吗？当前对话将被清空。')
    if (confirmed) {
      messages.value = []
      currentConversationId.value = undefined
    }
  }
}
</script>

<template>
  <div class="flex flex-col h-full">
    <!-- 顶部栏 -->
    <ChatHeader @new-chat="handleNewChat" />

    <!-- 消息列表 -->
    <MessageList 
      :messages="messages" 
      :is-loading="isLoading"
    />

    <!-- 输入框 -->
    <ChatInput 
      :is-loading="isLoading"
      @send="handleSend"
      @stop="handleStop"
    />
  </div>
</template>

