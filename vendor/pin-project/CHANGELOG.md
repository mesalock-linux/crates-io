# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

## [Unreleased]

## [0.4.8] - 2020-01-27

* [Ensured that users cannot implement `PinnedDrop` without proper attribute argument.][180]

[180]: https://github.com/taiki-e/pin-project/pull/180

## [0.4.7] - 2020-01-20

* [Fixed support for lifetime bounds.][176]

[176]: https://github.com/taiki-e/pin-project/pull/176

## [0.4.6] - 2019-11-20

* [Fixed compile error when there is `Self` in the where clause.][169]

[169]: https://github.com/taiki-e/pin-project/pull/169

## [0.4.5] - 2019-10-21

* [Fixed compile error with `dyn` types.][158]

[158]: https://github.com/taiki-e/pin-project/pull/158

## [0.4.4] - 2019-10-17

* [Fixed an issue where `PinnedDrop` implementations can call unsafe code without an unsafe block.][149]

[149]: https://github.com/taiki-e/pin-project/pull/149

## [0.4.3] - 2019-10-15 - YANKED

* [`#[pin_project]` can now interoperate with `#[cfg_attr()]`.][135]

* [`#[pin_project]` can now interoperate with `#[cfg()]` on tuple structs and tuple variants.][135]

* [Fixed support for DSTs(Dynamically Sized Types) on `#[pin_project(UnsafeUnpin)]`][120]

[120]: https://github.com/taiki-e/pin-project/pull/120
[135]: https://github.com/taiki-e/pin-project/pull/135

## [0.4.2] - 2019-09-29 - YANKED

* [Fixed support for DSTs(Dynamically Sized Types).][113]

[113]: https://github.com/taiki-e/pin-project/pull/113

## [0.4.1] - 2019-09-26 - YANKED

* [Fixed an issue that caused an error when using `#[pin_project]` on a type that has `#[pin]` + `!Unpin` field with no generics or lifetime.][111]

[111]: https://github.com/taiki-e/pin-project/pull/111

## [0.4.0] - 2019-09-25 - YANKED

* [**Pin projection has become a safe operation.**][18] In the absence of other unsafe code that you write, it is impossible to cause undefined behavior.

* `#[unsafe_project]` attribute has been replaced with `#[pin_project]` attribute. ([#18][18], [#33][33])

* [The `Unpin` argument has been removed - an `Unpin` impl is now generated by default.][18]

* Drop impls must be specified with `#[pinned_drop]` instead of via a normal `Drop` impl. ([#18][18], [#33][33], [#86][86])

* [`Unpin` impls must be specified with an impl of `UnsafeUnpin`, instead of implementing the normal `Unpin` trait.][18]

* [`#[pin_project]` attribute now determines the visibility of the projection type/method is based on the original type.][96]

* [`#[pin_project]` can now be used for public type with private field types.][53]

* [`#[pin_project]` can now interoperate with `#[cfg()]`.][77]

* [Added `project_ref` method to `#[pin_project]` types.][93]

* [Added `#[project_ref]` attribute.][93]

* [Removed "project_attr" feature and always enable `#[project]` attribute.][94]

* [`#[project]` attribute can now be used for `impl` blocks.][46]

* [`#[project]` attribute can now be used for `use` statements.][85]

* [`#[project]` attribute now supports `match` expressions at the position of the initializer expression of `let` expressions.][51]

Changes since the 0.4.0-beta.1 release:

* [Fixed an issue that caused an error when using `#[pin_project(UnsafeUnpin)]` and not providing a manual `UnsafeUnpin` implementation on a type with no generics or lifetime.][107]

[18]: https://github.com/taiki-e/pin-project/pull/18
[33]: https://github.com/taiki-e/pin-project/pull/107
[107]: https://github.com/taiki-e/pin-project/pull/107

## [0.4.0-beta.1] - 2019-09-21

* [Changed the argument type of project method back to `self: Pin<&mut Self>`.][90]

* [Removed "project_attr" feature and always enable `#[project]` attribute.][94]

* [Removed "renamed" feature.][100]

* [`#[project]` attribute can now be used for `use` statements.][85]

* [Added `project_ref` method and `#[project_ref]` attribute.][93]

* [`#[pin_project]` attribute now determines the visibility of the projection type/method is based on the original type.][96]

[85]: https://github.com/taiki-e/pin-project/pull/85
[90]: https://github.com/taiki-e/pin-project/pull/90
[93]: https://github.com/taiki-e/pin-project/pull/93
[94]: https://github.com/taiki-e/pin-project/pull/94
[96]: https://github.com/taiki-e/pin-project/pull/96
[100]: https://github.com/taiki-e/pin-project/pull/100

## [0.4.0-alpha.11] - 2019-09-11

* [Changed #[pinned_drop] to trait implementation.][86]

  ```rust
  #[pinned_drop]
  impl<T> PinnedDrop for Foo<'_, T> {
      fn drop(mut self: Pin<&mut Self>) {
          **self.project().was_dropped = true;
      }
  }
  ```

* Added some examples and generated code.

* Improve error messages.

[86]: https://github.com/taiki-e/pin-project/pull/86

## [0.4.0-alpha.10] - 2019-09-07

* [`#[pin_project]` can now interoperate with `#[cfg()]`.][77]

* Improved documentation.

[77]: https://github.com/taiki-e/pin-project/pull/77

## [0.4.0-alpha.9] - 2019-09-05

* [Added 'project_into' method to #[pin_project] types][69]. This can be useful when returning a pin projection from a method.
  ```rust
  fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T> {
      self.project_into().pinned
  }
  ```

* [Prevented UnpinStruct from appearing in the document by default.][71] See [taiki-e/pin-project#71][71] for more details.

[69]: https://github.com/taiki-e/pin-project/pull/69
[71]: https://github.com/taiki-e/pin-project/pull/69

## [0.4.0-alpha.8] - 2019-09-03

* [Improved document of generated code.][62]. Also added an option to control the document of generated code. See [taiki-e/pin-project#62][62] for more details.

* [Improved error messages][61]

[61]: https://github.com/taiki-e/pin-project/pull/61
[62]: https://github.com/taiki-e/pin-project/pull/62

## [0.4.0-alpha.7] - 2019-09-02

* [Applied `#[allow(dead_code)]` to generated types.][57]

[57]: https://github.com/taiki-e/pin-project/pull/57

## [0.4.0-alpha.6] - 2019-09-01

* [Allowed using `#[pin_project]` type with private field types][53]

[53]: https://github.com/taiki-e/pin-project/pull/53

## [0.4.0-alpha.5] - 2019-08-24

* [`#[project]` attribute now supports `match` expressions at the position of the initializer expression of `let` expressions.][51]

[51]: https://github.com/taiki-e/pin-project/pull/51

## [0.4.0-alpha.4] - 2019-08-23

* Avoided clippy::drop_bounds lint in generated code.

## [0.4.0-alpha.3] - 2019-08-23

* [Changed `project` method generated by `#[pin_project]` attribute to take an `&mut Pin<&mut Self>` argument.][47]

* [`#[project]` attribute can now be used for impl blocks.][46]

* [`#[pin_project]` attribute can now detect that the type used does not have its own drop implementation without actually implementing drop.][48] This removed some restrictions.

[46]: https://github.com/taiki-e/pin-project/pull/46
[47]: https://github.com/taiki-e/pin-project/pull/47
[48]: https://github.com/taiki-e/pin-project/pull/48

## [0.4.0-alpha.2] - 2019-08-13

* Updated `proc-macro2`, `syn`, and `quote` to 1.0.

## [0.4.0-alpha.1] - 2019-08-11

* **Pin projection has become a safe operation.**

* `#[unsafe_project]` has been replaced with `#[pin_project]`.

* The `Unpin` argument has been removed - an `Unpin` impl is now generated by default.

* Drop impls must be specified with `#[pinned_drop]` instead of via a normal `Drop` impl.

* `Unpin` impls must be specified with an impl of `UnsafeUnpin`, instead of implementing the normal `Unpin` trait.

* Made `#[project]` attribute disabled by default.

See also [tracking issue for 0.4 release][21].

[21]: https://github.com/taiki-e/pin-project/issues/21

## [0.3.5] - 2019-08-14

* Updated `proc-macro2`, `syn`, and `quote` to 1.0.

## [0.3.4] - 2019-07-21

* Improved error messages.

## [0.3.3] - 2019-07-15 - YANKED

* Improved error messages.

## [0.3.2] - 2019-03-30

* Avoided suffixes on tuple index.

## [0.3.1] - 2019-03-02

* Improved documentation.

* Updated minimum `syn` version to 0.15.22.

## [0.3.0] - 2019-02-20

* Removed `unsafe_fields` attribute.

* Removed `unsafe_variants` attribute.

## [0.2.2] - 2019-02-20

* Fixed a bug that generates incorrect code for the some structures with trait bounds on type generics.

## [0.2.1] - 2019-02-20

* Fixed a bug that generates incorrect code for the structures with where clause and associated type fields.

## [0.2.0] - 2019-02-11

* Made `unsafe_fields` optional.

* Improved documentation.

## [0.1.8] - 2019-02-02

* Added the feature to create projected enums to `unsafe_project`.

* Added `project` attribute to support pattern matching.

## [0.1.7] - 2019-01-19

* Fixed documentation.

## [0.1.6] - 2019-01-19

* `unsafe_fields` can now opt-out.

* Added `unsafe_variants` attribute. This attribute is available if pin-project is built with the "unsafe_variants" feature.

## [0.1.5] - 2019-01-17

* Added support for tuple struct to `unsafe_project`.

## [0.1.4] - 2019-01-12

* Added options for automatically implementing `Unpin` to both `unsafe_project` and `unsafe_fields`.

## [0.1.3] - 2019-01-11

* Fixed dependencies.

* Added `unsafe_fields` attribute.

## [0.1.2] - 2019-01-09

* Improved documentation.

## [0.1.1] - 2019-01-08

* Renamed from `unsafe_pin_project` to `unsafe_project`.

## [0.1.0] - 2019-01-08 - YANKED

Initial release

[Unreleased]: https://github.com/taiki-e/pin-project/compare/v0.4.8...HEAD
[0.4.8]: https://github.com/taiki-e/pin-project/compare/v0.4.7...v0.4.8
[0.4.7]: https://github.com/taiki-e/pin-project/compare/v0.4.6...v0.4.7
[0.4.6]: https://github.com/taiki-e/pin-project/compare/v0.4.5...v0.4.6
[0.4.5]: https://github.com/taiki-e/pin-project/compare/v0.4.4...v0.4.5
[0.4.4]: https://github.com/taiki-e/pin-project/compare/v0.4.3...v0.4.4
[0.4.3]: https://github.com/taiki-e/pin-project/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/taiki-e/pin-project/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/taiki-e/pin-project/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/taiki-e/pin-project/compare/v0.4.0-beta.1...v0.4.0
[0.4.0-beta.1]: https://github.com/taiki-e/pin-project/compare/v0.4.0-alpha.11...v0.4.0-beta.1
[0.4.0-alpha.11]: https://github.com/taiki-e/pin-project/compare/v0.4.0-alpha.10...v0.4.0-alpha.11
[0.4.0-alpha.10]: https://github.com/taiki-e/pin-project/compare/v0.4.0-alpha.9...v0.4.0-alpha.10
[0.4.0-alpha.9]: https://github.com/taiki-e/pin-project/compare/v0.4.0-alpha.8...v0.4.0-alpha.9
[0.4.0-alpha.8]: https://github.com/taiki-e/pin-project/compare/v0.4.0-alpha.7...v0.4.0-alpha.8
[0.4.0-alpha.7]: https://github.com/taiki-e/pin-project/compare/v0.4.0-alpha.6...v0.4.0-alpha.7
[0.4.0-alpha.6]: https://github.com/taiki-e/pin-project/compare/v0.4.0-alpha.5...v0.4.0-alpha.6
[0.4.0-alpha.5]: https://github.com/taiki-e/pin-project/compare/v0.4.0-alpha.4...v0.4.0-alpha.5
[0.4.0-alpha.4]: https://github.com/taiki-e/pin-project/compare/v0.4.0-alpha.3...v0.4.0-alpha.4
[0.4.0-alpha.3]: https://github.com/taiki-e/pin-project/compare/v0.4.0-alpha.2...v0.4.0-alpha.3
[0.4.0-alpha.2]: https://github.com/taiki-e/pin-project/compare/v0.4.0-alpha.1...v0.4.0-alpha.2
[0.4.0-alpha.1]: https://github.com/taiki-e/pin-project/compare/v0.3.5...v0.4.0-alpha.1
[0.3.5]: https://github.com/taiki-e/pin-project/compare/v0.3.4...v0.3.5
[0.3.4]: https://github.com/taiki-e/pin-project/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/taiki-e/pin-project/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/taiki-e/pin-project/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/taiki-e/pin-project/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/taiki-e/pin-project/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/taiki-e/pin-project/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taiki-e/pin-project/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/pin-project/compare/v0.1.8...v0.2.0
[0.1.8]: https://github.com/taiki-e/pin-project/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/taiki-e/pin-project/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/taiki-e/pin-project/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/taiki-e/pin-project/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/taiki-e/pin-project/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/taiki-e/pin-project/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/taiki-e/pin-project/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/taiki-e/pin-project/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/pin-project/releases/tag/v0.1.0