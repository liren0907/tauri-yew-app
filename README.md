# Tauri + Yew

Tauri 2 desktop app with a Yew (CSR) frontend built by Trunk, styled with Tailwind + DaisyUI.

## Prerequisites

- Rust toolchain (stable)
- wasm32 target: `rustup target add wasm32-unknown-unknown`
- Tauri CLI
- Trunk
- Node.js (Yarn) for Tailwind/DaisyUI

## Project Structure

- src/ Frontend (Yew) source
- src-tauri/ Tauri backend (Rust)
- public/ Static assets for Trunk
- tailwind.css Tailwind entry
- style.css Generated CSS (Trunk loads this)
- Trunk.toml Trunk build config
- src-tauri/tauri.conf.json Tauri app config

## Setup

Install CSS tooling:

```bash
yarn install
```

## Development

Run Tauri dev (starts Trunk and the desktop app, and builds CSS):

```bash
cargo tauri dev
```

Run only the frontend dev server:

```bash
trunk serve
```

Optionally watch CSS in a separate terminal:

```bash
yarn watch:css
```

Frontend dev server default:

- http://127.0.0.1:1420/

## Build

Build the desktop app:

```bash
cargo tauri build
```

Build only the frontend:

```bash
trunk build
```

## How It Works

- src/main.rs mounts the Yew app
- src/app.rs defines layout and routing
- src/routes.rs declares the route table
- tailwind.css builds into style.css via yarn scripts

## Common Issues

If `cargo tauri dev` fails with stale cache paths, clean the build cache:

```bash
cargo clean
```
