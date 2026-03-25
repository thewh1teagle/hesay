# hesay

Speaking Hebrew using [renikud-rs](https://github.com/thewh1teagle/renikud) for Hebrew G2P and [piper-rs](https://github.com/thewh1teagle/piper-rs) for TTS.

## Features

- Self-contained ~200MB binary, no external files or dependencies at runtime
- Hebrew text to speech via neural G2P + TTS
- Mixed Hebrew/English input — English words are phonemized via eSpeak
- ONNX Runtime embedded

## Usage

```sh
hesay "שלום עולם"
hesay "אני אוהב machine learning" out.wav
```

## Build

```console
wget https://huggingface.co/thewh1teagle/renikud/resolve/main/model.onnx -O g2p.onnx
wget https://huggingface.co/thewh1teagle/phonikud-tts-checkpoints/resolve/main/michael.onnx
wget https://huggingface.co/thewh1teagle/phonikud-tts-checkpoints/resolve/main/model.config.json -O michael.onnx.json
cargo build --release
```
