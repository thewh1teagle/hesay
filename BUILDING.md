# Building

## Prerequisites

Download the model files into the project root:

```console
wget https://huggingface.co/thewh1teagle/renikud/resolve/main/model.onnx -O renikud.onnx
wget https://huggingface.co/thewh1teagle/phonikud-tts-checkpoints/resolve/main/michael.onnx -O michael.onnx
wget https://huggingface.co/thewh1teagle/phonikud-tts-checkpoints/resolve/main/model.config.json -O michael.onnx.json
```

## Build

```console
cargo build --release
```

## macOS

On macOS, `bindgen` may fail to find system headers (`stdio.h not found`). Pass the SDK path explicitly:

```console
export BINDGEN_EXTRA_CLANG_ARGS="-isysroot $(xcrun --show-sdk-path)"
cargo build --release
```
