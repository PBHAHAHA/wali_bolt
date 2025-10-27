<script setup lang="ts">
import Sidebar from '@/components/Sidebar.vue'

// 侧边栏是否收起
const isCollapsed = ref(false)

// 当前激活的视图
const currentView = ref<'chat' | 'knowledge' | 'settings'>('chat')

// 切换侧边栏
function toggleSidebar() {
  isCollapsed.value = !isCollapsed.value
}

// 切换视图
function handleNavigate(view: 'chat' | 'knowledge' | 'settings') {
  currentView.value = view
}
</script>

<template>
  <div class="bg-[#eee] flex h-screen overflow-hidden p-1">
    <!-- 左侧边栏 -->
    <Sidebar 
      :collapsed="isCollapsed"
      @toggle="toggleSidebar"
      @navigate="handleNavigate"
    />
    
    <!-- 右侧内容区 -->
    <main class="flex-1 overflow-auto bg-white">
      <!-- 通过插槽传入内容，并传递当前视图 -->
      <slot :current-view="currentView" />
    </main>
  </div>
</template>
