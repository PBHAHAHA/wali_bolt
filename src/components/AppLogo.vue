<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    // 是否只显示图标（不显示文字）
    iconOnly?: boolean
    // 尺寸：small | default | large
    size?: 'small' | 'default' | 'large'
  }>(),
  {
    iconOnly: false,
    size: 'default'
  }
)

// 根据尺寸返回对应的类名
const sizeClasses = computed(() => {
  return {
    icon: {
      small: 'w-6 h-6',
      default: 'w-8 h-8',
      large: 'w-10 h-10'
    }[props.size],
    text: {
      small: 'text-sm',
      default: 'text-lg',
      large: 'text-xl'
    }[props.size]
  }
})
</script>

<template>
  <div class="flex items-center gap-2">
    <!-- Logo 图标 -->
    <div 
      :class="[
        'rounded-lg bg-primary flex items-center justify-center flex-shrink-0',
        sizeClasses.icon
      ]"
    >
      <span :class="['text-primary-foreground font-bold', sizeClasses.text]">
        W
      </span>
    </div>
    
    <!-- 应用名称 -->
    <transition
      enter-active-class="transition-opacity duration-200"
      leave-active-class="transition-opacity duration-200"
      enter-from-class="opacity-0"
      leave-to-class="opacity-0"
    >
      <h1 
        v-show="!props.iconOnly" 
        :class="['font-bold whitespace-nowrap', sizeClasses.text]"
      >
       
      </h1>
    </transition>
  </div>
</template>

