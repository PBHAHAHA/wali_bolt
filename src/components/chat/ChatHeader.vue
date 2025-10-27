<script setup lang="ts">
import { Plus, MoreVertical } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'

const emit = defineEmits<{
  newChat: []
}>()
</script>

<template>
  <header class="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
    <div class="flex items-center justify-between h-14 px-6">
      <!-- 标题 -->
      <div class="flex items-center gap-3">
        <h1 class="text-lg font-semibold">AI 对话</h1>
        <span class="text-xs px-2 py-1 rounded-md bg-muted text-muted-foreground">
          Beta
        </span>
      </div>

      <!-- 操作按钮 -->
      <div class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          @click="emit('newChat')"
        >
          <Plus class="w-4 h-4 mr-2" />
          新建对话
        </Button>

        <Button
          variant="ghost"
          size="icon"
        >
          <MoreVertical class="w-4 h-4" />
        </Button>
      </div>
    </div>
  </header>
</template>

