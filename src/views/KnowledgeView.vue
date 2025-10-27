<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Upload, FileText, Trash2, Plus, AlertCircle, FileUp } from 'lucide-vue-next'
import * as api from '@/lib/api'
import { open } from '@tauri-apps/plugin-dialog'

// 文档列表
const documents = ref<api.Document[]>([])

// 加载状态
const isLoading = ref(false)
const isUploading = ref(false)

// 上传相关
const uploadName = ref('')
const uploadContent = ref('')
const showUploadModal = ref(false)
const fileInput = ref<HTMLInputElement | null>(null)
const selectedFile = ref<File | null>(null)
const selectedFilePath = ref<string | null>(null)

// 加载文档列表
async function loadDocuments() {
  isLoading.value = true
  try {
    documents.value = await api.getDocuments()
  } catch (error) {
    console.error('加载文档失败:', error)
    alert(`加载文档失败：${error instanceof Error ? error.message : String(error)}`)
  } finally {
    isLoading.value = false
  }
}

// 上传文档
async function handleUpload() {
  if (!uploadName.value.trim()) {
    alert('请输入文档名称')
    return
  }
  
  // 如果选择了文件路径（PDF 或其他），使用文件路径上传
  if (selectedFilePath.value) {
    isUploading.value = true
    try {
      const response = await api.uploadDocumentFromPath(selectedFilePath.value)
      
      if (response.success) {
        alert(response.message)
        showUploadModal.value = false
        uploadName.value = ''
        uploadContent.value = ''
        selectedFile.value = null
        selectedFilePath.value = null
        await loadDocuments()
      } else {
        alert(response.message)
      }
    } catch (error) {
      console.error('上传文档失败:', error)
      alert(`上传失败：${error instanceof Error ? error.message : String(error)}`)
    } finally {
      isUploading.value = false
    }
    return
  }
  
  // 否则使用文本内容上传
  if (!uploadContent.value.trim()) {
    alert('请输入文档内容或选择文件')
    return
  }

  isUploading.value = true
  try {
    const response = await api.uploadDocument({
      name: uploadName.value.trim(),
      content: uploadContent.value.trim(),
      file_type: 'text'
    })
    
    if (response.success) {
      alert(response.message)
      showUploadModal.value = false
      uploadName.value = ''
      uploadContent.value = ''
      selectedFile.value = null
      await loadDocuments()
    } else {
      alert(response.message)
    }
  } catch (error) {
    console.error('上传文档失败:', error)
    alert(`上传失败：${error instanceof Error ? error.message : String(error)}`)
  } finally {
    isUploading.value = false
  }
}

// 删除文档
async function handleDelete(documentId: string, documentName: string) {
  if (!confirm(`确定要删除文档 "${documentName}" 吗？`)) {
    return
  }

  try {
    await api.deleteDocument(documentId)
    alert('删除成功')
    await loadDocuments()
  } catch (error) {
    console.error('删除文档失败:', error)
    alert(`删除失败：${error instanceof Error ? error.message : String(error)}`)
  }
}

// 格式化文件大小
function formatFileSize(bytes: number): string {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(2) + ' MB'
}

// 格式化时间
function formatTime(timestamp: number): string {
  const date = new Date(timestamp * 1000)
  return date.toLocaleString('zh-CN')
}

// 处理文件选择（使用 Tauri 对话框）
async function handleFileSelect() {
  try {
    const filePath = await open({
      multiple: false,
      filters: [{
        name: '文档文件',
        extensions: ['txt', 'md', 'csv', 'json', 'pdf', 'doc', 'docx']
      }]
    })
    
    if (!filePath) {
      console.log('用户取消了文件选择')
      return
    }
    
    // 获取文件名
    const fileName = filePath.split(/[\\/]/).pop() || '未知文件'
    const fileExtension = fileName.split('.').pop()?.toLowerCase()
    
    uploadName.value = fileName
    selectedFilePath.value = filePath
    
    // 对于 PDF 文件，显示提示
    if (fileExtension === 'pdf') {
      uploadContent.value = '✅ PDF 文件将在上传时自动解析\n\n📄 文件：' + fileName
      console.log(`已选择 PDF 文件: ${fileName}`)
    } else {
      // 对于文本文件，可以预览内容
      try {
        const content = await api.readFileContent(filePath)
        uploadContent.value = content
        console.log(`已读取文件内容: ${fileName}, 长度: ${content.length} 字符`)
      } catch (error) {
        console.error('读取文件内容失败:', error)
        uploadContent.value = '⚠️ 文件已选择，将在上传时读取\n\n📄 文件：' + fileName
      }
    }
  } catch (error) {
    console.error('选择文件失败:', error)
    uploadContent.value = '❌ 选择文件失败，请重试'
  }
}

// 文件选择变化（旧的 input 方式，保留作为备用）
async function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  
  if (!file) return
  
  selectedFile.value = file
  
  // 读取文件内容
  try {
    // 检查文件类型
    const fileExtension = file.name.split('.').pop()?.toLowerCase()
    
    if (fileExtension === 'pdf') {
      // PDF 文件：标记需要后端处理
      uploadName.value = file.name
      uploadContent.value = '⚠️ PDF 文件需要使用"选择文件"按钮（Tauri 对话框）来选择\n\n请点击上方的"选择文件"按钮重新选择。'
      console.warn('PDF 文件应使用 Tauri 对话框选择')
    } else {
      // 其他文本文件：直接读取
      const text = await file.text()
      uploadContent.value = text
      uploadName.value = file.name
      console.log(`通过浏览器 input 读取文件: ${file.name}`)
    }
  } catch (error) {
    console.error('读取文件失败:', error)
    uploadContent.value = '❌ 读取文件失败，请重试'
  }
}

onMounted(() => {
  loadDocuments()
})
</script>

<template>
  <div class="flex flex-col h-full">
    <!-- 顶部操作栏 -->
    <div class="flex items-center justify-between p-6 border-b">
      <div>
        <h1 class="text-2xl font-bold">知识库</h1>
        <p class="text-sm text-muted-foreground mt-1">
          上传文档，让 AI 理解您的知识
        </p>
      </div>
      <button
        @click="showUploadModal = true"
        class="flex items-center gap-2 px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-colors"
      >
        <Plus class="w-4 h-4" />
        <span>上传文档</span>
      </button>
    </div>

    <!-- 文档列表 -->
    <div class="flex-1 overflow-y-auto p-6">
      <div v-if="isLoading" class="flex items-center justify-center h-64">
        <div class="text-muted-foreground">加载中...</div>
      </div>

      <div v-else-if="documents.length === 0" class="flex flex-col items-center justify-center h-64 text-center">
        <AlertCircle class="w-16 h-16 text-muted-foreground mb-4" />
        <h3 class="text-lg font-semibold mb-2">还没有文档</h3>
        <p class="text-muted-foreground mb-4">上传您的第一个文档开始使用知识库</p>
        <button
          @click="showUploadModal = true"
          class="flex items-center gap-2 px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-colors"
        >
          <Upload class="w-4 h-4" />
          <span>上传文档</span>
        </button>
      </div>

      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div
          v-for="doc in documents"
          :key="doc.id"
          class="p-4 border rounded-lg hover:shadow-md transition-shadow"
        >
          <div class="flex items-start gap-3">
            <div class="p-2 bg-primary/10 rounded-lg">
              <FileText class="w-5 h-5 text-primary" />
            </div>
            <div class="flex-1 min-w-0">
              <h3 class="font-semibold truncate" :title="doc.name">
                {{ doc.name }}
              </h3>
              <p class="text-sm text-muted-foreground mt-1">
                {{ formatFileSize(doc.file_size) }}
              </p>
              <p class="text-xs text-muted-foreground mt-1">
                {{ formatTime(doc.created_at) }}
              </p>
            </div>
            <button
              @click="handleDelete(doc.id, doc.name)"
              class="p-2 text-muted-foreground hover:text-red-600 transition-colors"
              title="删除文档"
            >
              <Trash2 class="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 上传文档弹窗 -->
    <div
      v-if="showUploadModal"
      class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
      @click.self="showUploadModal = false"
    >
      <div class="bg-background rounded-lg shadow-xl w-full max-w-2xl mx-4">
        <div class="p-6 border-b">
          <h2 class="text-xl font-semibold">上传文档</h2>
        </div>
        
        <div class="p-6 space-y-4">
          <div>
            <label class="block text-sm font-medium mb-2">文档名称</label>
            <input
              v-model="uploadName"
              type="text"
              placeholder="例如：产品手册"
              class="w-full px-4 py-2 border border-border rounded-lg bg-background text-foreground"
            />
          </div>
          
          <!-- 文件选择 -->
          <div>
            <label class="block text-sm font-medium mb-2">选择文件（可选）</label>
            <input
              ref="fileInput"
              type="file"
              accept=".txt,.md,.csv,.json,.pdf,.doc,.docx"
              class="hidden"
              @change="handleFileChange"
            />
            <button
              @click="handleFileSelect"
              class="flex items-center gap-2 px-4 py-2 border border-border rounded-lg hover:bg-muted transition-colors"
            >
              <FileUp class="w-4 h-4" />
              <span>{{ selectedFile ? selectedFile.name : '选择文件' }}</span>
            </button>
            <p class="text-xs text-muted-foreground mt-1">
              支持格式：txt, md, csv, json, pdf, doc, docx
            </p>
          </div>
          
          <div>
            <label class="block text-sm font-medium mb-2">文档内容</label>
            <div class="relative">
              <textarea
                v-model="uploadContent"
                rows="10"
                placeholder="在此输入或粘贴文档内容..."
                class="w-full px-4 py-2 border border-border rounded-lg bg-background text-foreground resize-none"
              />
              <div class="absolute bottom-2 right-2 text-xs text-muted-foreground">
                {{ uploadContent.length }} 字符
              </div>
            </div>
            <p class="text-xs text-muted-foreground mt-1">
              💡 提示：支持复制粘贴大段文本，系统会自动分块处理
            </p>
          </div>
        </div>
        
        <div class="p-6 border-t flex justify-end gap-3">
          <button
            @click="showUploadModal = false"
            class="px-4 py-2 border border-border rounded-lg hover:bg-muted transition-colors"
            :disabled="isUploading"
          >
            取消
          </button>
          <button
            @click="handleUpload"
            class="px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-colors"
            :disabled="isUploading"
          >
            {{ isUploading ? '上传中...' : '上传' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

