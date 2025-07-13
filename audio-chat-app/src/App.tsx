import { useState, useEffect } from "react";
import VoiceChat from "./components/VoiceChat";
import HistorySidebar from "./components/HistorySidebar";
import StatusBar from "./components/StatusBar";
import { invoke } from "@tauri-apps/api/core";

export interface Message {
  id: string;
  conversation_id: string;
  role: "user" | "assistant";
  content: string;
  timestamp: string;
}

export interface Conversation {
  id: string;
  created_at: string;
}

function App() {
  const [currentConversationId, setCurrentConversationId] = useState<string | null>(null);
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [messages, setMessages] = useState<Message[]>([]);
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const [ollamaStatus, setOllamaStatus] = useState<boolean>(false);

  // Check Ollama status on startup
  useEffect(() => {
    checkOllamaStatus();
    loadConversations();
  }, []);

  const checkOllamaStatus = async () => {
    try {
      const status = await invoke<boolean>("check_ollama");
      setOllamaStatus(status);
    } catch (error) {
      console.error("Failed to check Ollama status:", error);
      setOllamaStatus(false);
    }
  };

  const loadConversations = async () => {
    try {
      const convs = await invoke<Conversation[]>("get_conversations");
      setConversations(convs);
    } catch (error) {
      console.error("Failed to load conversations:", error);
    }
  };

  const loadMessages = async (conversationId: string) => {
    try {
      const msgs = await invoke<Message[]>("get_messages", { conversationId });
      setMessages(msgs);
    } catch (error) {
      console.error("Failed to load messages:", error);
    }
  };

  const createNewConversation = async () => {
    try {
      const conversationId = await invoke<string>("create_conversation");
      setCurrentConversationId(conversationId);
      setMessages([]);
      await loadConversations();
      return conversationId;
    } catch (error) {
      console.error("Failed to create conversation:", error);
      throw error;
    }
  };

  const saveMessage = async (role: "user" | "assistant", content: string) => {
    if (!currentConversationId) {
      await createNewConversation();
    }
    
    try {
      await invoke("save_message", {
        conversationId: currentConversationId,
        role,
        content,
      });
      
      // Reload messages to show the new one
      if (currentConversationId) {
        await loadMessages(currentConversationId);
      }
    } catch (error) {
      console.error("Failed to save message:", error);
    }
  };

  const selectConversation = async (conversationId: string) => {
    setCurrentConversationId(conversationId);
    await loadMessages(conversationId);
    setSidebarOpen(false);
  };

  return (
    <div className="flex h-screen bg-gradient-to-br from-blue-900 via-purple-900 to-indigo-900">
      {/* Sidebar */}
      <HistorySidebar
        isOpen={sidebarOpen}
        onClose={() => setSidebarOpen(false)}
        conversations={conversations}
        currentConversationId={currentConversationId}
        onSelectConversation={selectConversation}
        onNewConversation={createNewConversation}
      />

      {/* Main Content */}
      <div className="flex-1 flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between p-4">
          <button
            onClick={() => setSidebarOpen(true)}
            className="text-white hover:text-gray-300 transition-colors"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </button>
          
          <h1 className="text-xl font-semibold text-white">Audio Chat</h1>
          
          <div className="w-6 h-6" /> {/* Spacer */}
        </div>

        {/* Voice Chat Component */}
        <div className="flex-1 flex items-center justify-center">
          <VoiceChat
            currentConversationId={currentConversationId}
            onSaveMessage={saveMessage}
            onCreateConversation={createNewConversation}
            messages={messages}
          />
        </div>

        {/* Status Bar */}
        <StatusBar ollamaStatus={ollamaStatus} onRefreshOllama={checkOllamaStatus} />
      </div>
    </div>
  );
}

export default App;