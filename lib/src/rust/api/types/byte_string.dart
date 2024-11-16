// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.1.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These functions are ignored (category: IgnoreBecauseExplicitAttribute): `from`
// These functions are ignored (category: IgnoreBecauseNotAllowedOwner): `from`

Future<void> wrapbytestring({required WrapByteString a}) =>
    RustLib.instance.api.crateApiTypesByteStringWrapbytestring(a: a);

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<WrapByteString>>
abstract class WrapByteString implements RustOpaqueInterface {
  /// Encodes the bytestring as a Base64 encoded string
  String asBase64();

  /// Creates a byte string from a Base64 encoded string
  static WrapByteString? fromBase64({required String data}) =>
      RustLib.instance.api
          .crateApiTypesByteStringWrapByteStringFromBase64(data: data);

  bool isEmpty();

  /// Test if the string is null
  bool isNull();

  /// Test if the string is null or empty
  bool isNullOrEmpty();

  /// Create a null string (not the same as an empty string)
  static WrapByteString null_() =>
      RustLib.instance.api.crateApiTypesByteStringWrapByteStringNull();

  /// This function is meant for use with NumericRange. It creates a substring from this string
  /// from min up to and inclusive of max. Note that min must have an index within the string
  /// but max is allowed to be beyond the end in which case the remainder of the string is
  /// returned (see docs for NumericRange).
  WrapByteString substring({required BigInt min, required BigInt max});
}
