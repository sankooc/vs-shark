// Mock VSCode API for development environment
const vscode = {
  postMessage: (message: any) => {
    console.log("VSCode postMessage:", message);
  },
  getState: () => {
    return {};
  },
  setState: (state: any) => {
    console.log("VSCode setState:", state);
  },
};

// Add vscode to window for development
declare global {
  interface Window {
    vscode: typeof vscode;
  }
}

if (import.meta.env.DEV) {
  window.vscode = vscode;
}

export default vscode;
