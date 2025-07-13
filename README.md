# Audio Chat App

A voice-first conversation application built with Tauri and React that enables natural audio conversations with AI. This app provides a seamless push-to-talk experience for macOS users.

## 🎯 Features

- **Voice-First Interface**: Push-to-talk audio recording with real-time waveform visualization
- **Speech-to-Text**: Powered by OpenAI's Whisper model for accurate transcription
- **AI Conversations**: Integration with Ollama for intelligent responses using Gemma3N model
- **Text-to-Speech**: macOS native TTS for natural voice responses
- **Conversation History**: Persistent storage of all conversations and messages
- **Modern UI**: Beautiful gradient interface with responsive design
- **Permission Management**: Proper microphone permission handling for macOS

## 🛠️ Tech Stack

### Frontend
- **React 19** with TypeScript
- **Tailwind CSS** for styling
- **Vite** for build tooling
- **Tauri API** for native integration

### Backend
- **Rust** with Tauri 2.0
- **Whisper-rs** for speech recognition
- **Ollama** for AI chat (Gemma3N model)
- **SQLite** for data persistence
- **cpal** for audio processing
- **macOS native TTS** for speech synthesis

## 📋 Prerequisites

Before running this application, ensure you have:

1. **Node.js** (v18 or higher)
2. **pnpm** package manager
3. **Rust** toolchain (latest stable)
4. **Ollama** installed and running
5. **macOS** (for native TTS and permissions)

## 🚀 Installation

### 1. Clone the Repository
```bash
git clone https://github.com/Dhruv2mars/pR-t-04.git
cd pR-t-04/audio-chat-app
```

### 2. Install Dependencies
```bash
pnpm install
```

### 3. Set Up Ollama
Install Ollama and run the required model:
```bash
# Install Ollama (if not already installed)
curl -fsSL https://ollama.ai/install.sh | sh

# Pull and run the Gemma3N model
ollama pull gemma3n:latest
ollama run gemma3n:latest
```

### 4. Download Whisper Model
Download the Whisper model for speech recognition:
```bash
# Create models directory
mkdir -p src-tauri/models

# Download the Whisper model (you'll need to download this manually)
# Place ggml-base.en.bin in src-tauri/models/
```

### 5. Build and Run
```bash
# Development mode
pnpm dev

# Build for production
pnpm build
```

## 🎤 Usage

1. **Launch the App**: Start the application and grant microphone permissions when prompted
2. **Start Conversation**: Click and hold the microphone button to record your message
3. **Release to Send**: Release the button to send your audio message
4. **Listen to Response**: The AI will respond with synthesized speech
5. **View History**: Access conversation history from the sidebar

## 📁 Project Structure

```
audio-chat-app/
├── src/                    # React frontend
│   ├── components/         # React components
│   │   ├── VoiceChat.tsx  # Main voice interface
│   │   ├── HistorySidebar.tsx
│   │   ├── StatusBar.tsx
│   │   ├── Waveform.tsx
│   │   └── MicrophonePermissionModal.tsx
│   ├── App.tsx            # Main app component
│   └── main.tsx           # Entry point
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs        # Tauri app setup
│   │   ├── asr.rs         # Speech recognition
│   │   ├── tts.rs         # Text-to-speech
│   │   ├── ollama.rs      # AI chat integration
│   │   ├── db.rs          # Database operations
│   │   ├── microphone.rs  # Microphone handling
│   │   └── audio_processing.rs
│   └── Cargo.toml         # Rust dependencies
└── package.json           # Node.js dependencies
```

## 🔧 Configuration

### Environment Variables
The app uses default configurations, but you can customize:

- **Ollama URL**: Defaults to `http://localhost:11434`
- **Whisper Model**: Uses `ggml-base.en.bin`
- **TTS**: Uses macOS native `say` command

### Database
Conversations are stored in SQLite database located in the app's data directory.

## 🐛 Troubleshooting

### Microphone Permissions
If microphone access is denied:
1. Go to System Preferences > Security & Privacy > Privacy > Microphone
2. Add the app to the list of allowed applications
3. Restart the application

### Ollama Connection Issues
- Ensure Ollama is running: `ollama run gemma3n:latest`
- Check if port 11434 is accessible
- Verify the model is downloaded: `ollama list`

### Whisper Model Issues
- Ensure `ggml-base.en.bin` is in `src-tauri/models/`
- Check file permissions and size
- Verify the model file is not corrupted

### Audio Processing Issues
- Check microphone input levels
- Ensure recording duration is at least 0.5 seconds
- Verify audio format compatibility

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- **OpenAI** for the Whisper speech recognition model
- **Ollama** for the local AI inference
- **Tauri** for the cross-platform framework
- **React** and **Rust** communities

## 📞 Support

For issues and questions:
- Create an issue on GitHub
- Check the troubleshooting section above
- Ensure all prerequisites are met

---

**Note**: This application requires macOS for optimal functionality due to native TTS and permission handling. 