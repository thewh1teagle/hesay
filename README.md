# hesay

~200MB self-contained binary for speaking Hebrew. Uses [renikud-rs](https://github.com/thewh1teagle/renikud) for Hebrew G2P and [piper-rs](https://github.com/thewh1teagle/piper-rs) for TTS, with ONNX Runtime embedded. No external dependencies, no model files needed.

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
