interface StatusBarProps {
  ollamaStatus: boolean;
  onRefreshOllama: () => void;
}

export default function StatusBar({ ollamaStatus, onRefreshOllama }: StatusBarProps) {
  return (
    <div className="bg-gray-900/50 border-t border-gray-700 p-3">
      <div className="flex items-center justify-between text-sm">
        {/* Ollama Status */}
        <div className="flex items-center space-x-2">
          <div className={`w-2 h-2 rounded-full ${ollamaStatus ? 'bg-green-500' : 'bg-red-500'}`} />
          <span className="text-gray-300">
            Ollama: {ollamaStatus ? 'Connected' : 'Disconnected'}
          </span>
          {!ollamaStatus && (
            <button
              onClick={onRefreshOllama}
              className="text-blue-400 hover:text-blue-300 underline"
            >
              Retry
            </button>
          )}
        </div>

        {/* Microphone Status */}
        <div className="flex items-center space-x-2">
          <div className="w-2 h-2 rounded-full bg-green-500" />
          <span className="text-gray-300">Microphone Ready</span>
        </div>

        {/* Help Text */}
        {!ollamaStatus && (
          <div className="text-gray-400 text-xs">
            Run: <code className="bg-gray-800 px-1 rounded">ollama run gemma3n:latest</code>
          </div>
        )}
      </div>
    </div>
  );
}