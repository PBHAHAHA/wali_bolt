<script setup lang="ts">
import MainLayout from '@/layouts/MainLayout.vue'
import ChatView from '@/views/ChatView.vue'
import KnowledgeView from '@/views/KnowledgeView.vue'
import { ref, onMounted } from 'vue'
import * as api from '@/lib/api'

const apiKeyConfigured = ref(false)
const apiKey = ref('')

// 检查 API Key 状态
async function checkApiKeyStatus() {
  try {
    apiKeyConfigured.value = await api.getApiKeyStatus()
  } catch (error) {
    console.error('检查 API Key 状态失败:', error)
  }
}

// 保存 API Key
async function saveApiKey() {
  if (!apiKey.value.trim()) {
    alert('请输入 API Key')
    return
  }

  try {
    await api.setApiKey(apiKey.value.trim())
    apiKeyConfigured.value = true
    alert('API Key 设置成功！')
    apiKey.value = ''
  } catch (error) {
    console.error('设置 API Key 失败:', error)
    alert(`设置失败：${error instanceof Error ? error.message : String(error)}`)
  }
}

onMounted(() => {
  checkApiKeyStatus()
})
</script>

<template>
  <MainLayout v-slot="{ currentView }">
    <!-- AI 对话视图 -->
    <ChatView v-if="currentView === 'chat'" />

    <!-- 知识库视图 -->
    <KnowledgeView v-else-if="currentView === 'knowledge'" />

    <!-- 设置视图 -->
    <div v-else-if="currentView === 'settings'" class="p-8 max-w-2xl mx-auto">
      <h1 class="text-3xl font-bold mb-6">设置</h1>
      
      <!-- API Key 配置 -->
      <div class="space-y-4">
        <div>
          <h2 class="text-xl font-semibold mb-2">API Key 配置</h2>
          <p class="text-muted-foreground mb-4">
            请配置您的通义千问 API Key 以使用 AI 功能
          </p>
          
          <!-- 获取 API Key 指南 -->
          <div class="mb-4 p-4 bg-blue-500/10 border border-blue-500/20 rounded-lg">
            <h3 class="text-sm font-semibold mb-2 text-blue-600 dark:text-blue-400">如何获取 API Key？</h3>
            <ol class="text-sm text-muted-foreground space-y-1 list-decimal list-inside">
              <li>访问 <a href="https://dashscope.console.aliyun.com/" target="_blank" class="text-blue-600 dark:text-blue-400 hover:underline">阿里云 DashScope 控制台</a></li>
              <li>登录您的阿里云账号（如果没有，请先注册）</li>
              <li>在控制台中创建或查看 API Key</li>
              <li>复制 API Key 并粘贴到下方输入框</li>
            </ol>
            <p class="text-xs text-muted-foreground mt-2">
              💡 提示：API Key 通常以 <code class="bg-background px-1 rounded">sk-</code> 开头
            </p>
          </div>
          
          <div v-if="apiKeyConfigured" class="mb-4 p-4 bg-green-500/10 border border-green-500/20 rounded-lg">
            <p class="text-green-600 dark:text-green-400">✓ API Key 已配置</p>
          </div>
          
          <div class="flex gap-2">
            <input
              v-model="apiKey"
              type="password"
              placeholder="请输入您的 API Key（sk-xxxxxxxx）"
              class="flex-1 px-4 py-2 border border-border rounded-lg bg-background text-foreground"
            />
            <button
              @click="saveApiKey"
              class="px-6 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-colors"
            >
              保存
            </button>
          </div>
        </div>
      </div>
    </div>
  </MainLayout>
</template>
