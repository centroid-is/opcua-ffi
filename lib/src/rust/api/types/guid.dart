// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.6.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `from`, `from`

Future<void> wrapguid({required UaGuid a}) =>
    RustLib.instance.api.crateApiTypesGuidWrapguid(a: a);

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Box < UAGuid >>>
abstract class BoxUaGuid implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<UAGuid>>
abstract class UaGuid implements RustOpaqueInterface {
  /// Returns the bytes of the Guid
  U8Array16 asBytes();

  static Future<UaGuid> fromBytes({required U8Array16 bytes}) =>
      RustLib.instance.api.crateApiTypesGuidUaGuidFromBytes(bytes: bytes);

  /// Creates a random Guid
  factory UaGuid() => RustLib.instance.api.crateApiTypesGuidUaGuidNew();

  /// Return a null guid, i.e. 00000000-0000-0000-0000-000000000000
  static UaGuid null_() => RustLib.instance.api.crateApiTypesGuidUaGuidNull();
}

class U8Array16 extends NonGrowableListView<int> {
  static const arraySize = 16;

  @internal
  Uint8List get inner => _inner;
  final Uint8List _inner;

  U8Array16(this._inner)
      : assert(_inner.length == arraySize),
        super(_inner);

  U8Array16.init() : this(Uint8List(arraySize));
}
