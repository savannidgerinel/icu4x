// generated by diplomat-tool

part of 'lib.g.dart';

/// See the [Rust documentation for `Collator`](https://docs.rs/icu/latest/icu/collator/struct.Collator.html) for more information.
final class Collator implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  Collator._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XCollator_destroy));

  /// Construct a new Collator instance.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/collator/struct.Collator.html#method.try_new) for more information.
  ///
  /// Throws [Error] on failure.
  factory Collator(DataProvider provider, Locale locale, CollatorOptions options) {
    final temp = ffi2.Arena();
    final result = _ICU4XCollator_create_v1(provider._ffi, locale._ffi, options._toFfi(temp));
    temp.releaseAll();
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return Collator._fromFfi(result.union.ok, []);
  }

  /// Compare two strings.
  ///
  /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
  /// to the WHATWG Encoding Standard.
  ///
  /// See the [Rust documentation for `compare_utf16`](https://docs.rs/icu/latest/icu/collator/struct.Collator.html#method.compare_utf16) for more information.
  int compare(String left, String right) {
    final temp = ffi2.Arena();
    final leftView = left.utf16View;
    final rightView = right.utf16View;
    final result = _ICU4XCollator_compare_utf16_(_ffi, leftView.allocIn(temp), leftView.length, rightView.allocIn(temp), rightView.length);
    temp.releaseAll();
    return result;
  }

  /// The resolved options showing how the default options, the requested options,
  /// and the options from locale data were combined. None of the struct fields
  /// will have `Auto` as the value.
  ///
  /// See the [Rust documentation for `resolved_options`](https://docs.rs/icu/latest/icu/collator/struct.Collator.html#method.resolved_options) for more information.
  ResolvedCollatorOptions get resolvedOptions {
    final result = _ICU4XCollator_resolved_options(_ffi);
    return ResolvedCollatorOptions._fromFfi(result);
  }
}

@meta.ResourceIdentifier('ICU4XCollator_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XCollator_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XCollator_destroy(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier('ICU4XCollator_create_v1')
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, _CollatorOptionsFfi)>(isLeaf: true, symbol: 'ICU4XCollator_create_v1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XCollator_create_v1(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale, _CollatorOptionsFfi options);

@meta.ResourceIdentifier('ICU4XCollator_compare_utf16_')
@ffi.Native<ffi.Int8 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint16>, ffi.Size, ffi.Pointer<ffi.Uint16>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XCollator_compare_utf16_')
// ignore: non_constant_identifier_names
external int _ICU4XCollator_compare_utf16_(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint16> leftData, int leftLength, ffi.Pointer<ffi.Uint16> rightData, int rightLength);

@meta.ResourceIdentifier('ICU4XCollator_resolved_options')
@ffi.Native<_ResolvedCollatorOptionsFfi Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCollator_resolved_options')
// ignore: non_constant_identifier_names
external _ResolvedCollatorOptionsFfi _ICU4XCollator_resolved_options(ffi.Pointer<ffi.Opaque> self);
