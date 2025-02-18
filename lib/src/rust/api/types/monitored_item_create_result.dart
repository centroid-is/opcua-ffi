// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.8.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'status_code.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `from`, `from`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<MonitoredItemCreateResult>>
abstract class MonitoredItemCreateResult implements RustOpaqueInterface {
  int get monitoredItemId;

  int get revisedQueueSize;

  double get revisedSamplingInterval;

  StatusCode get statusCode;

  set monitoredItemId(int monitoredItemId);

  set revisedQueueSize(int revisedQueueSize);

  set revisedSamplingInterval(double revisedSamplingInterval);

  set statusCode(StatusCode statusCode);
}
