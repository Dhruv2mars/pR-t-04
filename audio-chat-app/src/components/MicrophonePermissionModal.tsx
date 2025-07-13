import { useState } from "react";

interface MicrophonePermissionModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export default function MicrophonePermissionModal({ isOpen, onClose }: MicrophonePermissionModalProps) {
  const [currentStep, setCurrentStep] = useState(0);

  const steps = [
    {
      title: "Step 1: Open System Settings",
      description: "Click the Apple menu (🍎) in the top-left corner of your screen",
      image: "🍎",
      instruction: "Then select 'System Settings' from the dropdown menu"
    },
    {
      title: "Step 2: Navigate to Privacy & Security",
      description: "In the System Settings window, look for 'Privacy & Security' in the sidebar",
      image: "🔒",
      instruction: "Click on 'Privacy & Security' (you may need to scroll down)"
    },
    {
      title: "Step 3: Select Microphone",
      description: "In the Privacy & Security section, find and click 'Microphone'",
      image: "🎙️",
      instruction: "This will show you all apps that have requested microphone access"
    },
    {
      title: "Step 4: Enable Audio Chat App",
      description: "Find 'Audio Chat App' in the list and toggle it ON",
      image: "✅",
      instruction: "The toggle should turn blue when enabled. If you don't see the app, try using the microphone button first to trigger the permission request."
    }
  ];

  const nextStep = () => {
    if (currentStep < steps.length - 1) {
      setCurrentStep(currentStep + 1);
    }
  };

  const prevStep = () => {
    if (currentStep > 0) {
      setCurrentStep(currentStep - 1);
    }
  };

  const openSystemSettings = () => {
    // This will attempt to open System Settings directly to Privacy & Security
    window.open("x-apple.systempreferences:com.apple.preference.security?Privacy_Microphone");
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-xl max-w-md w-full mx-4 max-h-[90vh] overflow-y-auto">
        {/* Header */}
        <div className="bg-gradient-to-r from-blue-600 to-purple-600 text-white p-6 rounded-t-lg">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-bold">Enable Microphone Access</h2>
            <button
              onClick={onClose}
              className="text-white hover:text-gray-200 text-2xl font-bold"
            >
              ×
            </button>
          </div>
          <p className="text-blue-100 mt-2">
            Follow these steps to allow Audio Chat App to access your microphone
          </p>
        </div>

        {/* Content */}
        <div className="p-6">
          {/* Progress Bar */}
          <div className="mb-6">
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm text-gray-600">Progress</span>
              <span className="text-sm text-gray-600">{currentStep + 1} of {steps.length}</span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2">
              <div 
                className="bg-gradient-to-r from-blue-600 to-purple-600 h-2 rounded-full transition-all duration-300"
                style={{ width: `${((currentStep + 1) / steps.length) * 100}%` }}
              ></div>
            </div>
          </div>

          {/* Current Step */}
          <div className="text-center mb-6">
            <div className="text-6xl mb-4">{steps[currentStep].image}</div>
            <h3 className="text-lg font-semibold text-gray-800 mb-2">
              {steps[currentStep].title}
            </h3>
            <p className="text-gray-600 mb-2">
              {steps[currentStep].description}
            </p>
            <p className="text-sm text-gray-500 italic">
              {steps[currentStep].instruction}
            </p>
          </div>

          {/* Quick Access Button */}
          {currentStep === 0 && (
            <div className="mb-6 p-4 bg-blue-50 rounded-lg border border-blue-200">
              <p className="text-sm text-blue-800 mb-3">
                <strong>Quick shortcut:</strong> Click the button below to open System Settings directly
              </p>
              <button
                onClick={openSystemSettings}
                className="w-full bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded-lg transition-colors"
              >
                Open System Settings
              </button>
            </div>
          )}

          {/* Navigation Buttons */}
          <div className="flex justify-between">
            <button
              onClick={prevStep}
              disabled={currentStep === 0}
              className={`px-4 py-2 rounded-lg transition-colors ${
                currentStep === 0
                  ? "bg-gray-200 text-gray-400 cursor-not-allowed"
                  : "bg-gray-300 hover:bg-gray-400 text-gray-700"
              }`}
            >
              Previous
            </button>

            {currentStep < steps.length - 1 ? (
              <button
                onClick={nextStep}
                className="px-4 py-2 bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white rounded-lg transition-colors"
              >
                Next Step
              </button>
            ) : (
              <button
                onClick={onClose}
                className="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors"
              >
                Done
              </button>
            )}
          </div>

          {/* Additional Help */}
          <div className="mt-6 p-4 bg-gray-50 rounded-lg">
            <h4 className="font-semibold text-gray-800 mb-2">Need more help?</h4>
            <ul className="text-sm text-gray-600 space-y-1">
              <li>• Make sure you're running macOS Monterey (12.0) or later</li>
              <li>• You may need administrator privileges to change these settings</li>
              <li>• <strong>Important:</strong> If the app doesn't appear in the list, click the microphone button first</li>
              <li>• The app will only appear in Privacy Settings after requesting microphone access</li>
              <li>• Restart the Audio Chat App after enabling permissions</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}