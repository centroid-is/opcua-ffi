// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.8.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`, `from`, `into`

Future<void> monitoreditemcreaterequest(
        {required MonitoredItemCreateRequest a}) =>
    RustLib.instance.api
        .crateApiTypesMonitoredItemCreateRequestMonitoreditemcreaterequest(
            a: a);

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<MonitoredItemCreateRequest>>
abstract class MonitoredItemCreateRequest implements RustOpaqueInterface {}
