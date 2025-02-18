# Based on flutter_rust_bridge minimal dart example

Will hopefully hold bindings from dart to locka99/opcua

## Developing

```
~/.cargo/bin/flutter_rust_bridge_codegen generate
dart --enable-experiment=native-assets test
dart --enable-experiment=native-assets lib/main.dart
```