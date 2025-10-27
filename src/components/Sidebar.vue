<script setup lang="ts">
import { MessageSquare, BookOpen, Settings } from 'lucide-vue-next'

// Props
defineProps<{
  collapsed?: boolean
}>()

// Emits
const emit = defineEmits<{
  toggle: []
  navigate: [view: 'chat' | 'knowledge' | 'settings']
}>()

// 当前激活的视图
const activeView = ref<'chat' | 'knowledge' | 'settings'>('chat')

// 菜单项配置
const menuItems = [
  {
    id: 'chat' as const,
    label: 'AI对话',
    icon: MessageSquare,
  },
  {
    id: 'knowledge' as const,
    label: '知识库',
    icon: BookOpen,
  },
  {
    id: 'settings' as const,
    label: '设置',
    icon: Settings,
  },
]

// 切换视图
function navigateTo(view: 'chat' | 'knowledge' | 'settings') {
  activeView.value = view
  emit('navigate', view)
}
</script>

<template>
  <aside
    :class="[
      'flex flex-col h-screen transition-all duration-300',
      collapsed ? 'w-16' : 'w-44'
    ]"
  >
    <!-- Logo 区域 - 点击可收起/展开 -->
    <!-- <div 
      :class="[
        'flex items-center border-b px-4 h-16 transition-all duration-300 cursor-pointer hover:bg-accent/50',
        collapsed ? 'justify-center' : 'justify-start'
      ]"
      @click="emit('toggle')"
    >
      <AppLogo :icon-only="collapsed" size="default" />
    </div> -->

    <!-- 导航菜单 -->
    <nav class="flex-1 p-2 space-y-1">
      <Button
        v-for="item in menuItems"
        :key="item.id"
        variant="ghost"
        size="icon"
        :class="[
          'w-full transition-all duration-200',
          collapsed ? 'justify-center px-2' : 'justify-start px-3',
          activeView === item.id && 'bg-accent text-accent-foreground'
        ]"
        @click="navigateTo(item.id)"
      >
        <component 
          :is="item.icon" 
          :class="[
            'flex-shrink-0 text-muted-foreground',
            collapsed ? 'w-5 h-5' : 'w-5 h-5'
          ]" 
        />
        
        <!-- 菜单文字 - 收起时隐藏 -->
        <transition
          enter-active-class="transition-opacity duration-200"
          leave-active-class="transition-opacity duration-200"
          enter-from-class="opacity-0"
          leave-to-class="opacity-0"
        >
          <span v-show="!collapsed" class="whitespace-nowrap text-sm text-muted-foreground">
            {{ item.label }}
          </span>
        </transition>
      </Button>
    </nav>

  </aside>
</template>

