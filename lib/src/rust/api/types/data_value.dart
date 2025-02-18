// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.8.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'byte_string.dart';
import 'date_time.dart';
import 'guid.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'status_code.dart';
import 'string.dart';
import 'variant.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `from`, `from`

Future<void> wrapdatavalue({required DataValue a}) =>
    RustLib.instance.api.crateApiTypesDataValueWrapdatavalue(a: a);

class DataValue {
  final Variant? value;

  /// The status associated with the value.
  /// Not present if the StatusCode bit in the EncodingMask is False
  final StatusCode? status;

  /// The source timestamp associated with the value.
  /// Not present if the SourceTimestamp bit in the EncodingMask is False.
  final UaDateTime? sourceTimestamp;

  /// The number of 10 picosecond intervals for the SourceTimestamp.
  /// Not present if the SourcePicoSeconds bit in the EncodingMask is False.
  /// If the source timestamp is missing the picoseconds are ignored.
  final int? sourcePicoseconds;

  /// The Server timestamp associated with the value.
  /// Not present if the ServerTimestamp bit in the EncodingMask is False.
  final UaDateTime? serverTimestamp;

  /// The number of 10 picosecond intervals for the ServerTimestamp.
  /// Not present if the ServerPicoSeconds bit in the EncodingMask is False.
  /// If the Server timestamp is missing the picoseconds are ignored.
  final int? serverPicoseconds;

  const DataValue({
    this.value,
    this.status,
    this.sourceTimestamp,
    this.sourcePicoseconds,
    this.serverTimestamp,
    this.serverPicoseconds,
  });

  @override
  int get hashCode =>
      value.hashCode ^
      status.hashCode ^
      sourceTimestamp.hashCode ^
      sourcePicoseconds.hashCode ^
      serverTimestamp.hashCode ^
      serverPicoseconds.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is DataValue &&
          runtimeType == other.runtimeType &&
          value == other.value &&
          status == other.status &&
          sourceTimestamp == other.sourceTimestamp &&
          sourcePicoseconds == other.sourcePicoseconds &&
          serverTimestamp == other.serverTimestamp &&
          serverPicoseconds == other.serverPicoseconds;
}
