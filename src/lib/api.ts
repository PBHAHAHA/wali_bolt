import { invoke } from '@tauri-apps/api/core'

// ==================== 类型定义 ====================

export interface Message {
  id: string
  role: 'user' | 'assistant'
  content: string
  timestamp?: number
  sources?: string[]
}

export interface Conversation {
  id: string
  title: string
  created_at: number
  updated_at: number
}

export interface Document {
  id: string
  name: string
  content: string
  file_type?: string
  file_size: number
  created_at: number
  updated_at: number
}

export interface AskQuestionRequest {
  question: string
  conversation_id?: string
}

export interface AskQuestionResponse {
  success: boolean
  answer: string
  sources: string[]
  conversation_id: string
}

export interface UploadDocumentRequest {
  name: string
  content: string
  file_type?: string
}

export interface UploadDocumentResponse {
  success: boolean
  message: string
  document_id?: string
}

// ==================== API 函数 ====================

/**
 * 设置 API Key
 */
export async function setApiKey(apiKey: string): Promise<void> {
  await invoke('set_api_key', { apiKey })
}

/**
 * 获取 API Key 状态
 */
export async function getApiKeyStatus(): Promise<boolean> {
  return await invoke('get_api_key_status')
}

/**
 * 提问（RAG 问答）
 */
export async function askQuestion(request: AskQuestionRequest): Promise<AskQuestionResponse> {
  return await invoke('ask_question', { request })
}

/**
 * 获取所有对话列表
 */
export async function getConversations(): Promise<Conversation[]> {
  return await invoke('get_conversations')
}

/**
 * 获取对话消息列表
 */
export async function getMessages(conversationId: string): Promise<Message[]> {
  return await invoke('get_messages', { conversationId })
}

/**
 * 删除对话
 */
export async function deleteConversation(conversationId: string): Promise<boolean> {
  return await invoke('delete_conversation', { conversationId })
}

/**
 * 上传文档
 */
export async function uploadDocument(request: UploadDocumentRequest): Promise<UploadDocumentResponse> {
  return await invoke('upload_document', { request })
}

/**
 * 从文件路径上传文档（支持 PDF）
 */
export async function uploadDocumentFromPath(filePath: string): Promise<UploadDocumentResponse> {
  return await invoke('upload_document_from_path', { filePath })
}

/**
 * 获取所有文档列表
 */
export async function getDocuments(): Promise<Document[]> {
  return await invoke('get_documents')
}

/**
 * 删除文档
 */
export async function deleteDocument(documentId: string): Promise<boolean> {
  return await invoke('delete_document', { documentId })
}

/**
 * 读取文件内容
 */
export async function readFileContent(filePath: string): Promise<string> {
  return await invoke('read_file_content', { filePath })
}

/**
 * 获取文件信息
 */
export async function getFileInfo(filePath: string): Promise<[string, number]> {
  return await invoke('get_file_info', { filePath })
}

