# Audio Chat App

Audio-only macOS chat app with push-to-talk, using Whisper-Zero base (ASR), Gemma 3n (Ollama), and MeloTTS Mini (TTS).

## Features

- 🎙️ **Push-to-Talk Interface**: Hold the microphone button to record audio
- 🗣️ **Speech Recognition**: Powered by Whisper base.en model (~142MB)
- 🤖 **AI Conversation**: Uses Gemma 3n via Ollama for intelligent responses
- 🔊 **Text-to-Speech**: MeloTTS-English for natural voice synthesis
- 💾 **Conversation History**: SQLite database stores all conversations
- 🎨 **Beautiful UI**: Gradient background with waveform animations
- 📱 **Collapsible Sidebar**: View and manage conversation history

## Prerequisites

1. **Ollama**: Install and run Gemma 3n model
   ```bash
   # Install Ollama (if not already installed)
   curl -fsSL https://ollama.ai/install.sh | sh
   
   # Run the Gemma 3n model
   ollama run gemma3n:latest
   ```

2. **Python 3 & MeloTTS**: Required for text-to-speech
   ```bash
   # Install MeCab (required for MeloTTS)
   brew install mecab
   
   # MeloTTS will be installed automatically in the project's virtual environment
   ```

3. **Node.js & pnpm**: For frontend development
   ```bash
   # Install pnpm if not already installed
   npm install -g pnpm
   ```

## Setup

1. **Clone and install dependencies**:
   ```bash
   cd audio-chat-app
   pnpm install
   ```

2. **Install MeloTTS** (done automatically during first run):
   ```bash
   python3 -m venv venv
   source venv/bin/activate
   pip install git+https://github.com/myshell-ai/MeloTTS.git
   ```

3. **Run the development server**:
   ```bash
   pnpm tauri dev
   ```

4. **Allow microphone access** when prompted by macOS.

## Usage

1. **Start Ollama**: Make sure `ollama run gemma3n:latest` is running in a terminal
2. **Open the app**: The status bar will show if Ollama is connected
3. **Hold to record**: Press and hold the 🎙️ button to record your voice
4. **Release to send**: The app will transcribe, process with AI, and speak the response
5. **View history**: Click the hamburger menu to see past conversations

## Architecture

### Frontend (React 19.1.0 + TypeScript)
- **VoiceChat.tsx**: Main push-to-talk interface with waveform
- **HistorySidebar.tsx**: Collapsible conversation history
- **StatusBar.tsx**: Shows Ollama and microphone status
- **Waveform.tsx**: Animated audio visualization

### Backend (Tauri 2.0 + Rust)
- **main.rs**: Tauri app entry point and command handlers
- **asr.rs**: Whisper-based speech recognition
- **tts.rs**: MeloTTS text-to-speech synthesis
- **ollama.rs**: API integration with Ollama/Gemma 3n
- **db.rs**: SQLite conversation storage

### Models (~270MB total)
- **Whisper base.en**: `src-tauri/models/ggml-base.en.bin` (~142MB)
- **MeloTTS-English**: Downloaded automatically via Python package

## Database

Conversations are stored in SQLite at:
```
~/Library/Application Support/com.example.audiochat/conversations.db
```

Schema:
- `conversations`: id, created_at
- `messages`: id, conversation_id, role, content, timestamp

## Troubleshooting

### Ollama Issues
- **Error**: "Please run ollama run gemma3n:latest"
- **Solution**: Start Ollama in a terminal: `ollama run gemma3n:latest`

### Microphone Issues
- **Error**: "Microphone access denied"
- **Solution**: Enable microphone permissions in System Settings > Privacy & Security > Microphone

### Audio Processing Issues
- **Error**: "Transcription failed"
- **Solution**: Ensure Whisper model is bundled correctly in `src-tauri/models/`

### TTS Issues
- **Error**: "Audio synthesis failed"
- **Solution**: Ensure MeloTTS is installed: `pip install git+https://github.com/myshell-ai/MeloTTS.git`

## Development

### Build for Production
```bash
pnpm tauri build
```

### Model Information
- **Whisper**: `ggml-base.en.bin` from ggerganov/whisper.cpp
- **MeloTTS**: English model from myshell-ai/MeloTTS-English
- **LLM**: Gemma 3n (4B parameters) via Ollama

### Performance
- **Latency**: ~1-1.5s total (300ms ASR + 500ms LLM + 0.5-1s TTS)
- **Memory**: ~4GB RAM for Gemma 3n + ~500MB for models
- **Storage**: ~270MB for bundled models

## License

MIT License - see LICENSE file for details.

## Notes

- **Prototype**: This is a prototype focused on functionality over advanced features
- **macOS Only**: Designed and tested for macOS (Monterey 12+ recommended)
- **Offline-First**: All models run locally except for Ollama API calls
- **English Only**: Optimized for English language conversations