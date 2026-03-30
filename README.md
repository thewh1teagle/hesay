# hesay

Speaking Hebrew using [renikud-rs](https://github.com/thewh1teagle/renikud) for Hebrew G2P and [piper-rs](https://github.com/thewh1teagle/piper-rs) for TTS.

## Features

- Self-contained ~100MB binary, no external files or dependencies at runtime
- Hebrew text to speech via neural G2P + TTS
- Mixed Hebrew/English input — English words are phonemized via eSpeak
- ONNX Runtime embedded
- Drop-in ready for AI agents and pipelines — just call the binary

## Usage

```sh
hesay "שלום עולם"
hesay "אני אוהב machine learning" out.wav
```

## Build

See [BUILDING.md](BUILDING.md)