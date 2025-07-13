import { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Message } from "../App";
import Waveform from "./Waveform";
import MicrophonePermissionModal from "./MicrophonePermissionModal";

interface VoiceChatProps {
  currentConversationId: string | null;
  onSaveMessage: (role: "user" | "assistant", content: string) => Promise<void>;
  onCreateConversation: () => Promise<string>;
  messages: Message[];
}

export default function VoiceChat({
  currentConversationId,
  onSaveMessage,
  onCreateConversation,
  messages,
}: VoiceChatProps) {
  const [isRecording, setIsRecording] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const [transcribedText, setTranscribedText] = useState("");
  const [responseText, setResponseText] = useState("");
  const [isPlaying, setIsPlaying] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showPermissionModal, setShowPermissionModal] = useState(false);
  const [recordingStartTime, setRecordingStartTime] = useState<number | null>(null);

  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const audioChunksRef = useRef<Blob[]>([]);
  const audioRef = useRef<HTMLAudioElement | null>(null);

  // Get the latest messages for display
  const latestMessages = messages.slice(-2);

  // Check microphone permission on component mount
  useEffect(() => {
    const checkMicPermission = async () => {
      try {
        const hasPermission = await invoke<boolean>("check_microphone_permission");
        if (!hasPermission) {
          console.log("Microphone permission not granted yet");
        }
      } catch (error) {
        console.error("Failed to check microphone permission:", error);
      }
    };
    
    checkMicPermission();
  }, []);

  const startRecording = async () => {
    try {
      setError(null);
      
      // First, request microphone permission through Tauri (this will make the app appear in Privacy Settings)
      try {
        const hasPermission = await invoke<boolean>("request_microphone_permission");
        if (!hasPermission) {
          setError("Microphone access denied. Please enable microphone permissions in System Settings.");
          return;
        }
      } catch (permissionError) {
        console.error("Permission request failed:", permissionError);
        setError("Microphone access denied. Please enable microphone permissions in System Settings.");
        return;
      }
      
      // Now try to get the media stream
      const stream = await navigator.mediaDevices.getUserMedia({ 
        audio: {
          sampleRate: 48000,
          channelCount: 1,
          echoCancellation: true,
          noiseSuppression: true
        } 
      });
      
      // Try to use WAV format if supported, otherwise fall back to WebM
      const options = { mimeType: 'audio/webm;codecs=opus' };
      if (!MediaRecorder.isTypeSupported(options.mimeType)) {
        console.log("WebM not supported, trying WAV");
        options.mimeType = 'audio/wav';
      }
      
      mediaRecorderRef.current = new MediaRecorder(stream, options);
      audioChunksRef.current = [];

      mediaRecorderRef.current.ondataavailable = (event) => {
        audioChunksRef.current.push(event.data);
      };

      mediaRecorderRef.current.onstop = async () => {
        const actualMimeType = mediaRecorderRef.current?.mimeType || "audio/webm";
        const audioBlob = new Blob(audioChunksRef.current, { type: actualMimeType });
        console.log("Audio blob created:", audioBlob.size, "bytes, type:", audioBlob.type);
        await processAudio(audioBlob);
        
        // Stop all tracks to release microphone
        stream.getTracks().forEach(track => track.stop());
      };

      mediaRecorderRef.current.start();
      setIsRecording(true);
      setRecordingStartTime(Date.now());
    } catch (error) {
      console.error("Failed to start recording:", error);
      setError("Microphone access denied. Please enable microphone permissions in System Settings.");
    }
  };

  const stopRecording = () => {
    if (mediaRecorderRef.current && isRecording) {
      const recordingDuration = recordingStartTime ? Date.now() - recordingStartTime : 0;
      
      if (recordingDuration < 500) { // Less than 0.5 seconds
        setError("Recording too short. Please hold the button for at least 0.5 seconds.");
        setIsRecording(false);
        setRecordingStartTime(null);
        return;
      }
      
      mediaRecorderRef.current.stop();
      setIsRecording(false);
      setRecordingStartTime(null);
    }
  };

  const processAudio = async (audioBlob: Blob) => {
    setIsProcessing(true);
    setTranscribedText("");
    setResponseText("");

    try {
      // Convert blob to array buffer
      const arrayBuffer = await audioBlob.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);
      
      console.log("Sending audio data to backend:", uint8Array.length, "bytes");
      
      // Step 1: Process audio and transcribe
      const transcription = await invoke<string>("process_audio_blob", {
        audioData: Array.from(uint8Array),
      });
      
      setTranscribedText(transcription);
      
      // Save user message
      if (!currentConversationId) {
        await onCreateConversation();
      }
      await onSaveMessage("user", transcription);

      // Step 2: Send to Ollama
      const response = await invoke<string>("send_prompt", {
        prompt: transcription,
      });
      
      setResponseText(response);
      
      // Save assistant message
      await onSaveMessage("assistant", response);

      // Step 3: Synthesize speech
      const audioPath = await invoke<string>("synthesize_speech", {
        text: response,
      });
      
      // Play the synthesized audio
      await playAudio(audioPath);

    } catch (error) {
      console.error("Failed to process audio:", error);
      setError(error as string);
    } finally {
      setIsProcessing(false);
    }
  };

  const playAudio = async (audioPath: string) => {
    try {
      setIsPlaying(true);
      console.log("Attempting to play audio from:", audioPath);
      
      // Try multiple approaches for audio playback
      let audio: HTMLAudioElement;
      
      try {
        // Method 1: Read file and create blob URL
        const audioData = await invoke<number[]>("read_audio_file", { path: audioPath });
        console.log("Read audio data:", audioData.length, "bytes");
        
        const uint8Array = new Uint8Array(audioData);
        const blob = new Blob([uint8Array], { type: "audio/wav" });
        const blobUrl = URL.createObjectURL(blob);
        
        console.log("Created blob URL:", blobUrl, "Blob size:", blob.size);
        
        audio = new Audio(blobUrl);
        
        // Clean up blob URL when done
        const cleanup = () => URL.revokeObjectURL(blobUrl);
        audio.addEventListener('ended', cleanup);
        audio.addEventListener('error', cleanup);
        
      } catch (blobError) {
        console.warn("Blob method failed, trying asset protocol:", blobError);
        
        // Method 2: Try using Tauri's asset protocol
        const assetUrl = `asset://localhost/${audioPath.replace(/^\//, '')}`;
        console.log("Trying asset URL:", assetUrl);
        audio = new Audio(assetUrl);
      }
      
      audioRef.current = audio;
      
      audio.onended = () => {
        setIsPlaying(false);
      };
      
      audio.onerror = (e) => {
        console.error("Audio playback error:", e);
        setIsPlaying(false);
        setError(`Failed to play audio response: ${e.type}`);
      };
      
      audio.onloadstart = () => {
        console.log("Audio loading started");
      };
      
      audio.oncanplay = () => {
        console.log("Audio can start playing");
      };
      
      await audio.play();
      console.log("Audio playback started successfully");
      
    } catch (error) {
      console.error("Failed to play audio:", error);
      setIsPlaying(false);
      setError(`Failed to play audio response: ${error}`);
    }
  };

  const handleMouseDown = () => {
    if (!isProcessing) {
      startRecording();
    }
  };

  const handleMouseUp = () => {
    if (isRecording) {
      stopRecording();
    }
  };

  // Prevent context menu on right click
  const handleContextMenu = (e: React.MouseEvent) => {
    e.preventDefault();
  };

  return (
    <div className="flex flex-col items-center space-y-8 p-8 max-w-2xl mx-auto">
      {/* Waveform Animation */}
      <Waveform isActive={isRecording || isPlaying} />

      {/* Microphone Button */}
      <div className="relative">
        <button
          onMouseDown={handleMouseDown}
          onMouseUp={handleMouseUp}
          onMouseLeave={handleMouseUp}
          onContextMenu={handleContextMenu}
          disabled={isProcessing}
          className={`
            w-24 h-24 rounded-full flex items-center justify-center text-white text-3xl
            transition-all duration-200 transform
            ${isRecording 
              ? "bg-red-500 scale-110 shadow-lg shadow-red-500/50" 
              : isProcessing
              ? "bg-yellow-500 animate-pulse"
              : "bg-blue-600 hover:bg-blue-700 hover:scale-105 shadow-lg"
            }
            ${isProcessing ? "cursor-not-allowed" : "cursor-pointer"}
            select-none
          `}
        >
          {isProcessing ? (
            <div className="animate-spin">⚙️</div>
          ) : (
            "🎙️"
          )}
        </button>
        
        {/* Recording indicator */}
        {isRecording && (
          <div className="absolute -top-2 -right-2 w-6 h-6 bg-red-500 rounded-full animate-pulse flex items-center justify-center">
            <div className="w-3 h-3 bg-white rounded-full"></div>
          </div>
        )}
      </div>

      {/* Instructions */}
      <div className="text-center text-gray-300">
        {isRecording ? (
          <p className="text-lg font-medium">🔴 Recording... Release to send</p>
        ) : isProcessing ? (
          <p className="text-lg font-medium">⚙️ Processing...</p>
        ) : isPlaying ? (
          <p className="text-lg font-medium">🔊 Playing response...</p>
        ) : (
          <p className="text-lg">Hold the microphone button to record</p>
        )}
      </div>

      {/* Current Transcription and Response */}
      {(transcribedText || responseText) && (
        <div className="w-full space-y-4">
          {transcribedText && (
            <div className="bg-blue-900/30 rounded-lg p-4 border border-blue-500/30">
              <p className="text-sm text-blue-300 mb-1">You said:</p>
              <p className="text-white">{transcribedText}</p>
            </div>
          )}
          
          {responseText && (
            <div className="bg-purple-900/30 rounded-lg p-4 border border-purple-500/30">
              <p className="text-sm text-purple-300 mb-1">Assistant:</p>
              <p className="text-white">{responseText}</p>
            </div>
          )}
        </div>
      )}

      {/* Recent Messages */}
      {latestMessages.length > 0 && !transcribedText && !responseText && (
        <div className="w-full space-y-4">
          <h3 className="text-lg font-medium text-gray-300 text-center">Recent Messages</h3>
          {latestMessages.map((message) => (
            <div
              key={message.id}
              className={`rounded-lg p-4 border ${
                message.role === "user"
                  ? "bg-blue-900/20 border-blue-500/20"
                  : "bg-purple-900/20 border-purple-500/20"
              }`}
            >
              <p className={`text-sm mb-1 ${
                message.role === "user" ? "text-blue-300" : "text-purple-300"
              }`}>
                {message.role === "user" ? "You:" : "Assistant:"}
              </p>
              <p className="text-white text-sm">{message.content}</p>
            </div>
          ))}
        </div>
      )}

      {/* Error Display */}
      {error && (
        <div className="w-full bg-red-900/30 border border-red-500/30 rounded-lg p-4">
          <div className="flex items-start justify-between">
            <div className="flex-1">
              <p className="text-red-300 text-sm">{error}</p>
              {error.includes("Microphone access denied") && (
                <button
                  onClick={() => setShowPermissionModal(true)}
                  className="mt-3 bg-blue-600 hover:bg-blue-700 text-white text-sm py-2 px-4 rounded-lg transition-colors inline-flex items-center space-x-2"
                >
                  <span>📖</span>
                  <span>Tell me how to fix this</span>
                </button>
              )}
            </div>
          </div>
        </div>
      )}

      {/* Microphone Permission Modal */}
      <MicrophonePermissionModal
        isOpen={showPermissionModal}
        onClose={() => setShowPermissionModal(false)}
      />
    </div>
  );
}