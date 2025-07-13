interface WaveformProps {
  isActive: boolean;
}

export default function Waveform({ isActive }: WaveformProps) {
  const bars = Array.from({ length: 5 }, (_, i) => i);

  return (
    <div className="flex items-center justify-center space-x-1 h-16">
      {bars.map((bar) => (
        <div
          key={bar}
          className={`
            w-2 bg-gradient-to-t from-blue-500 to-purple-500 rounded-full
            transition-all duration-300 ease-in-out
            ${isActive 
              ? `h-8 animate-pulse` 
              : 'h-4'
            }
          `}
          style={{
            animationDelay: isActive ? `${bar * 0.1}s` : '0s',
            height: isActive ? `${Math.random() * 32 + 16}px` : '16px',
          }}
        />
      ))}
    </div>
  );
}