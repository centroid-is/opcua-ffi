// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `from`, `from`

Future<void> wrapdatetime({required WrapDateTime a}) =>
    RustLib.instance.api.crateApiTypesDateTimeWrapdatetime(a: a);

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Box < WrapDateTime >>>
abstract class BoxWrapDateTime implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<WrapDateTime>>
abstract class WrapDateTime implements RustOpaqueInterface {
  /// Time as chrono
  DateTime asChrono();

  /// To checked ticks. Function returns 0 or MAX_INT64
  /// if date exceeds valid OPC UA range
  PlatformInt64 checkedTicks();

  /// Constructs a date time for the endtimes
  static WrapDateTime endtimes() =>
      RustLib.instance.api.crateApiTypesDateTimeWrapDateTimeEndtimes();

  /// Returns the maximum tick value, corresponding to the end of time
  static PlatformInt64 endtimesTicks() =>
      RustLib.instance.api.crateApiTypesDateTimeWrapDateTimeEndtimesTicks();

  /// Constructs a date time for the epoch
  static WrapDateTime epoch() =>
      RustLib.instance.api.crateApiTypesDateTimeWrapDateTimeEpoch();

  /// Tests if the date time is null (i.e. equal to epoch)
  bool isNull();

  /// Constructs from the current time
  static WrapDateTime now() =>
      RustLib.instance.api.crateApiTypesDateTimeWrapDateTimeNow();

  /// Constructs from the current time with an offset
  static WrapDateTime nowWithOffset({required Duration offset}) =>
      RustLib.instance.api
          .crateApiTypesDateTimeWrapDateTimeNowWithOffset(offset: offset);

  /// Creates a null date time (i.e. the epoch)
  static WrapDateTime null_() =>
      RustLib.instance.api.crateApiTypesDateTimeWrapDateTimeNull();

  /// Returns the time in ticks, of 100 nanosecond intervals
  PlatformInt64 ticks();

  /// Returns an RFC 3339 and ISO 8601 date and time string such as 1996-12-19T16:39:57-08:00.
  String toRfc3339();

  /// Constructs from a year, month, day
  static WrapDateTime ymd(
          {required int year, required int month, required int day}) =>
      RustLib.instance.api.crateApiTypesDateTimeWrapDateTimeYmd(
          year: year, month: month, day: day);

  /// Constructs from a year, month, day, hour, minute, second
  static WrapDateTime ymdHms(
          {required int year,
          required int month,
          required int day,
          required int hour,
          required int minute,
          required int second}) =>
      RustLib.instance.api.crateApiTypesDateTimeWrapDateTimeYmdHms(
          year: year,
          month: month,
          day: day,
          hour: hour,
          minute: minute,
          second: second);

  /// Constructs from a year, month, day, hour, minute, second, nanosecond
  static WrapDateTime ymdHmsNano(
          {required int year,
          required int month,
          required int day,
          required int hour,
          required int minute,
          required int second,
          required int nanos}) =>
      RustLib.instance.api.crateApiTypesDateTimeWrapDateTimeYmdHmsNano(
          year: year,
          month: month,
          day: day,
          hour: hour,
          minute: minute,
          second: second,
          nanos: nanos);
}
