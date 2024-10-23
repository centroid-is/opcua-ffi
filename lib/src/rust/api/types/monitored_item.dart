// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `from`, `from`

Future<void> monitoreditem({required WrapMonitoredItem a}) =>
    RustLib.instance.api.crateApiTypesMonitoredItemMonitoreditem(a: a);

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<WrapMonitoredItem>>
abstract class WrapMonitoredItem implements RustOpaqueInterface {
  /// Client assigned handle for the monitored item.
  int clientHandle();

  /// Whether the oldest values are discarded on queue overflow on the server.
  bool discardOldest();

  /// Server assigned ID of the monitored item.
  int id();

  factory WrapMonitoredItem({required int clientHandle}) =>
      RustLib.instance.api.crateApiTypesMonitoredItemWrapMonitoredItemNew(
          clientHandle: clientHandle);

  /// Queue size on the server.
  BigInt queueSize();

  /// Sampling interval.
  double samplingInterval();
}
