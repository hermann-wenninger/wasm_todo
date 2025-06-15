
import os
import subprocess
from pathlib import Path

PROJECT_ROOT = Path("C:/Users/Dell/Documents/a_work/wasm_todo/webassembly_standart/wasm_react_demo")
RUST_DIR = PROJECT_ROOT / "rust-wasm"
FRONTEND_DIR = PROJECT_ROOT / "frontend"
PKG_DIR = FRONTEND_DIR / "pkg"

def create_dirs():
    os.makedirs(RUST_DIR / "src", exist_ok=True)
    os.makedirs(FRONTEND_DIR / "src", exist_ok=True)

def write_rust_files():
    (RUST_DIR / "Cargo.toml").write_text("""\
[package]
name = "rust_wasm"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
""")

    (RUST_DIR / "src/lib.rs").write_text("""\
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hallo, {name} aus Rust 🦀")
}
""")

def create_react_vite_app():
    subprocess.run(["npm", "create", "vite@latest", FRONTEND_DIR.name, "--", "--template", "react-ts"], cwd=PROJECT_ROOT, check=True)

def install_frontend_deps():
    subprocess.run(["npm", "install"], cwd=FRONTEND_DIR, check=True)
    subprocess.run(["npm", "install", "--save-dev", "@vitejs/plugin-react"], cwd=FRONTEND_DIR, check=True)

def update_vite_config():
    vite_config = FRONTEND_DIR / "vite.config.ts"
    vite_config.write_text("""\
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
})
""")

def write_react_files():
    (FRONTEND_DIR / "src/App.tsx").write_text("""\
import React, { useEffect, useState } from 'react'
import init, { greet } from '../pkg/rust_wasm'

function App() {
  const [output, setOutput] = useState('')

  useEffect(() => {
    init()
  }, [])

  const handleClick = () => {
    const result = greet("React")
    setOutput(result)
  }

  return (
    <div style={{ padding: 20 }}>
      <h1>Rust + WebAssembly + React</h1>
      <button onClick={handleClick}>Sag Hallo!</button>
      <p>{output}</p>
    </div>
  )
}

export default App
""")

def build_wasm():
    subprocess.run(["wasm-pack", "build", "--target", "web", "--out-dir", str(PKG_DIR)], cwd=RUST_DIR, check=True)

def main():
    print("🔧 Erstelle Verzeichnisse und Dateien ...")
    create_dirs()
    write_rust_files()
    
    print("⚛️ Erstelle Vite React App ...")
    create_react_vite_app()
    
    print("📦 Installiere npm-Abhängigkeiten ...")
    install_frontend_deps()
    
    print("📝 Ersetze Konfigurations- und React-Dateien ...")
    update_vite_config()
    write_react_files()

    print("🔨 Baue Rust → WebAssembly mit wasm-pack ...")
    build_wasm()

    print("✅ Fertig! Starte mit:\n  cd wasm_react_demo/frontend\n  npm run dev")

if __name__ == "__main__":
    main()
