// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'node_id.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$Identifier {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) numeric,
    required TResult Function(UaString field0) string,
    required TResult Function(UaGuid field0) guid,
    required TResult Function(ByteString field0) byteString,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? numeric,
    TResult? Function(UaString field0)? string,
    TResult? Function(UaGuid field0)? guid,
    TResult? Function(ByteString field0)? byteString,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? numeric,
    TResult Function(UaString field0)? string,
    TResult Function(UaGuid field0)? guid,
    TResult Function(ByteString field0)? byteString,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Identifier_Numeric value) numeric,
    required TResult Function(Identifier_String value) string,
    required TResult Function(Identifier_Guid value) guid,
    required TResult Function(Identifier_ByteString value) byteString,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Identifier_Numeric value)? numeric,
    TResult? Function(Identifier_String value)? string,
    TResult? Function(Identifier_Guid value)? guid,
    TResult? Function(Identifier_ByteString value)? byteString,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Identifier_Numeric value)? numeric,
    TResult Function(Identifier_String value)? string,
    TResult Function(Identifier_Guid value)? guid,
    TResult Function(Identifier_ByteString value)? byteString,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $IdentifierCopyWith<$Res> {
  factory $IdentifierCopyWith(
          Identifier value, $Res Function(Identifier) then) =
      _$IdentifierCopyWithImpl<$Res, Identifier>;
}

/// @nodoc
class _$IdentifierCopyWithImpl<$Res, $Val extends Identifier>
    implements $IdentifierCopyWith<$Res> {
  _$IdentifierCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of Identifier
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$Identifier_NumericImplCopyWith<$Res> {
  factory _$$Identifier_NumericImplCopyWith(_$Identifier_NumericImpl value,
          $Res Function(_$Identifier_NumericImpl) then) =
      __$$Identifier_NumericImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$Identifier_NumericImplCopyWithImpl<$Res>
    extends _$IdentifierCopyWithImpl<$Res, _$Identifier_NumericImpl>
    implements _$$Identifier_NumericImplCopyWith<$Res> {
  __$$Identifier_NumericImplCopyWithImpl(_$Identifier_NumericImpl _value,
      $Res Function(_$Identifier_NumericImpl) _then)
      : super(_value, _then);

  /// Create a copy of Identifier
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Identifier_NumericImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$Identifier_NumericImpl extends Identifier_Numeric {
  const _$Identifier_NumericImpl(this.field0) : super._();

  @override
  final int field0;

  @override
  String toString() {
    return 'Identifier.numeric(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Identifier_NumericImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of Identifier
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Identifier_NumericImplCopyWith<_$Identifier_NumericImpl> get copyWith =>
      __$$Identifier_NumericImplCopyWithImpl<_$Identifier_NumericImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) numeric,
    required TResult Function(UaString field0) string,
    required TResult Function(UaGuid field0) guid,
    required TResult Function(ByteString field0) byteString,
  }) {
    return numeric(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? numeric,
    TResult? Function(UaString field0)? string,
    TResult? Function(UaGuid field0)? guid,
    TResult? Function(ByteString field0)? byteString,
  }) {
    return numeric?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? numeric,
    TResult Function(UaString field0)? string,
    TResult Function(UaGuid field0)? guid,
    TResult Function(ByteString field0)? byteString,
    required TResult orElse(),
  }) {
    if (numeric != null) {
      return numeric(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Identifier_Numeric value) numeric,
    required TResult Function(Identifier_String value) string,
    required TResult Function(Identifier_Guid value) guid,
    required TResult Function(Identifier_ByteString value) byteString,
  }) {
    return numeric(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Identifier_Numeric value)? numeric,
    TResult? Function(Identifier_String value)? string,
    TResult? Function(Identifier_Guid value)? guid,
    TResult? Function(Identifier_ByteString value)? byteString,
  }) {
    return numeric?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Identifier_Numeric value)? numeric,
    TResult Function(Identifier_String value)? string,
    TResult Function(Identifier_Guid value)? guid,
    TResult Function(Identifier_ByteString value)? byteString,
    required TResult orElse(),
  }) {
    if (numeric != null) {
      return numeric(this);
    }
    return orElse();
  }
}

abstract class Identifier_Numeric extends Identifier {
  const factory Identifier_Numeric(final int field0) = _$Identifier_NumericImpl;
  const Identifier_Numeric._() : super._();

  @override
  int get field0;

  /// Create a copy of Identifier
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Identifier_NumericImplCopyWith<_$Identifier_NumericImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Identifier_StringImplCopyWith<$Res> {
  factory _$$Identifier_StringImplCopyWith(_$Identifier_StringImpl value,
          $Res Function(_$Identifier_StringImpl) then) =
      __$$Identifier_StringImplCopyWithImpl<$Res>;
  @useResult
  $Res call({UaString field0});
}

/// @nodoc
class __$$Identifier_StringImplCopyWithImpl<$Res>
    extends _$IdentifierCopyWithImpl<$Res, _$Identifier_StringImpl>
    implements _$$Identifier_StringImplCopyWith<$Res> {
  __$$Identifier_StringImplCopyWithImpl(_$Identifier_StringImpl _value,
      $Res Function(_$Identifier_StringImpl) _then)
      : super(_value, _then);

  /// Create a copy of Identifier
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Identifier_StringImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as UaString,
    ));
  }
}

/// @nodoc

class _$Identifier_StringImpl extends Identifier_String {
  const _$Identifier_StringImpl(this.field0) : super._();

  @override
  final UaString field0;

  @override
  String toString() {
    return 'Identifier.string(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Identifier_StringImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of Identifier
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Identifier_StringImplCopyWith<_$Identifier_StringImpl> get copyWith =>
      __$$Identifier_StringImplCopyWithImpl<_$Identifier_StringImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) numeric,
    required TResult Function(UaString field0) string,
    required TResult Function(UaGuid field0) guid,
    required TResult Function(ByteString field0) byteString,
  }) {
    return string(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? numeric,
    TResult? Function(UaString field0)? string,
    TResult? Function(UaGuid field0)? guid,
    TResult? Function(ByteString field0)? byteString,
  }) {
    return string?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? numeric,
    TResult Function(UaString field0)? string,
    TResult Function(UaGuid field0)? guid,
    TResult Function(ByteString field0)? byteString,
    required TResult orElse(),
  }) {
    if (string != null) {
      return string(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Identifier_Numeric value) numeric,
    required TResult Function(Identifier_String value) string,
    required TResult Function(Identifier_Guid value) guid,
    required TResult Function(Identifier_ByteString value) byteString,
  }) {
    return string(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Identifier_Numeric value)? numeric,
    TResult? Function(Identifier_String value)? string,
    TResult? Function(Identifier_Guid value)? guid,
    TResult? Function(Identifier_ByteString value)? byteString,
  }) {
    return string?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Identifier_Numeric value)? numeric,
    TResult Function(Identifier_String value)? string,
    TResult Function(Identifier_Guid value)? guid,
    TResult Function(Identifier_ByteString value)? byteString,
    required TResult orElse(),
  }) {
    if (string != null) {
      return string(this);
    }
    return orElse();
  }
}

abstract class Identifier_String extends Identifier {
  const factory Identifier_String(final UaString field0) =
      _$Identifier_StringImpl;
  const Identifier_String._() : super._();

  @override
  UaString get field0;

  /// Create a copy of Identifier
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Identifier_StringImplCopyWith<_$Identifier_StringImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Identifier_GuidImplCopyWith<$Res> {
  factory _$$Identifier_GuidImplCopyWith(_$Identifier_GuidImpl value,
          $Res Function(_$Identifier_GuidImpl) then) =
      __$$Identifier_GuidImplCopyWithImpl<$Res>;
  @useResult
  $Res call({UaGuid field0});
}

/// @nodoc
class __$$Identifier_GuidImplCopyWithImpl<$Res>
    extends _$IdentifierCopyWithImpl<$Res, _$Identifier_GuidImpl>
    implements _$$Identifier_GuidImplCopyWith<$Res> {
  __$$Identifier_GuidImplCopyWithImpl(
      _$Identifier_GuidImpl _value, $Res Function(_$Identifier_GuidImpl) _then)
      : super(_value, _then);

  /// Create a copy of Identifier
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Identifier_GuidImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as UaGuid,
    ));
  }
}

/// @nodoc

class _$Identifier_GuidImpl extends Identifier_Guid {
  const _$Identifier_GuidImpl(this.field0) : super._();

  @override
  final UaGuid field0;

  @override
  String toString() {
    return 'Identifier.guid(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Identifier_GuidImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of Identifier
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Identifier_GuidImplCopyWith<_$Identifier_GuidImpl> get copyWith =>
      __$$Identifier_GuidImplCopyWithImpl<_$Identifier_GuidImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) numeric,
    required TResult Function(UaString field0) string,
    required TResult Function(UaGuid field0) guid,
    required TResult Function(ByteString field0) byteString,
  }) {
    return guid(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? numeric,
    TResult? Function(UaString field0)? string,
    TResult? Function(UaGuid field0)? guid,
    TResult? Function(ByteString field0)? byteString,
  }) {
    return guid?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? numeric,
    TResult Function(UaString field0)? string,
    TResult Function(UaGuid field0)? guid,
    TResult Function(ByteString field0)? byteString,
    required TResult orElse(),
  }) {
    if (guid != null) {
      return guid(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Identifier_Numeric value) numeric,
    required TResult Function(Identifier_String value) string,
    required TResult Function(Identifier_Guid value) guid,
    required TResult Function(Identifier_ByteString value) byteString,
  }) {
    return guid(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Identifier_Numeric value)? numeric,
    TResult? Function(Identifier_String value)? string,
    TResult? Function(Identifier_Guid value)? guid,
    TResult? Function(Identifier_ByteString value)? byteString,
  }) {
    return guid?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Identifier_Numeric value)? numeric,
    TResult Function(Identifier_String value)? string,
    TResult Function(Identifier_Guid value)? guid,
    TResult Function(Identifier_ByteString value)? byteString,
    required TResult orElse(),
  }) {
    if (guid != null) {
      return guid(this);
    }
    return orElse();
  }
}

abstract class Identifier_Guid extends Identifier {
  const factory Identifier_Guid(final UaGuid field0) = _$Identifier_GuidImpl;
  const Identifier_Guid._() : super._();

  @override
  UaGuid get field0;

  /// Create a copy of Identifier
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Identifier_GuidImplCopyWith<_$Identifier_GuidImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Identifier_ByteStringImplCopyWith<$Res> {
  factory _$$Identifier_ByteStringImplCopyWith(
          _$Identifier_ByteStringImpl value,
          $Res Function(_$Identifier_ByteStringImpl) then) =
      __$$Identifier_ByteStringImplCopyWithImpl<$Res>;
  @useResult
  $Res call({ByteString field0});
}

/// @nodoc
class __$$Identifier_ByteStringImplCopyWithImpl<$Res>
    extends _$IdentifierCopyWithImpl<$Res, _$Identifier_ByteStringImpl>
    implements _$$Identifier_ByteStringImplCopyWith<$Res> {
  __$$Identifier_ByteStringImplCopyWithImpl(_$Identifier_ByteStringImpl _value,
      $Res Function(_$Identifier_ByteStringImpl) _then)
      : super(_value, _then);

  /// Create a copy of Identifier
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Identifier_ByteStringImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as ByteString,
    ));
  }
}

/// @nodoc

class _$Identifier_ByteStringImpl extends Identifier_ByteString {
  const _$Identifier_ByteStringImpl(this.field0) : super._();

  @override
  final ByteString field0;

  @override
  String toString() {
    return 'Identifier.byteString(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Identifier_ByteStringImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of Identifier
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Identifier_ByteStringImplCopyWith<_$Identifier_ByteStringImpl>
      get copyWith => __$$Identifier_ByteStringImplCopyWithImpl<
          _$Identifier_ByteStringImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) numeric,
    required TResult Function(UaString field0) string,
    required TResult Function(UaGuid field0) guid,
    required TResult Function(ByteString field0) byteString,
  }) {
    return byteString(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? numeric,
    TResult? Function(UaString field0)? string,
    TResult? Function(UaGuid field0)? guid,
    TResult? Function(ByteString field0)? byteString,
  }) {
    return byteString?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? numeric,
    TResult Function(UaString field0)? string,
    TResult Function(UaGuid field0)? guid,
    TResult Function(ByteString field0)? byteString,
    required TResult orElse(),
  }) {
    if (byteString != null) {
      return byteString(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Identifier_Numeric value) numeric,
    required TResult Function(Identifier_String value) string,
    required TResult Function(Identifier_Guid value) guid,
    required TResult Function(Identifier_ByteString value) byteString,
  }) {
    return byteString(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Identifier_Numeric value)? numeric,
    TResult? Function(Identifier_String value)? string,
    TResult? Function(Identifier_Guid value)? guid,
    TResult? Function(Identifier_ByteString value)? byteString,
  }) {
    return byteString?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Identifier_Numeric value)? numeric,
    TResult Function(Identifier_String value)? string,
    TResult Function(Identifier_Guid value)? guid,
    TResult Function(Identifier_ByteString value)? byteString,
    required TResult orElse(),
  }) {
    if (byteString != null) {
      return byteString(this);
    }
    return orElse();
  }
}

abstract class Identifier_ByteString extends Identifier {
  const factory Identifier_ByteString(final ByteString field0) =
      _$Identifier_ByteStringImpl;
  const Identifier_ByteString._() : super._();

  @override
  ByteString get field0;

  /// Create a copy of Identifier
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Identifier_ByteStringImplCopyWith<_$Identifier_ByteStringImpl>
      get copyWith => throw _privateConstructorUsedError;
}
