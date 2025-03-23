# 11 - Packages & module system

## Packages

- Packages are marked with a kedi.toml file at the root.
- All files under a package is exposed. 
- Underscore in front of a name denotes a private module name / private top-level values.  It's mostly a convention - but they are excluded from auto-completion, not suggested by error messages etc.

## Dependencies

kedi.toml denotates three forms of dependencies:

- `exposed-dependencies`: Dependencies that are exposed to the public interface. Exposed dependencies of all "direct" dependencies are resolved to a single version.
- `internal-dependencies`: Internal dependencies are not exposed to the public interface - so the dependency resolver can resolve them to different versions. As an optimisation they are shared when the version bounds allow.
- `build-tools`: Dependencies that are only used in the build process. They are not included in the final binary.

### Versions

- No meanings encoded in the version numbers. Dot separated alphanumeric strings.
- Specify ranges with min and max versions.

```
[package]
name = "my-package"
version = "1.0.0"
description = "My package"
license = "MIT"
author = "..."

[exposed-dependencies]
some_dep = "1.2"
another_dep = "[1.2, 2.0), [2.2, 3.0)"

[internal-dependencies]
yet_another_dep = { version = "1.2.3.devel", as = "y_a_d" }
path_dep = "../another-package"

[build-tools]
build_dep = "1.2.3"
```

## Imports

No unqualified imports. All unqualified values are either local variables or from prelude.

```kedi
import somedep
import anotherdep as ad

..
somedep.foo()
ad.foo()
ad.bar.baz()
```

## Inline dependencies

I think this will be very useful for copy-pasted/auto-generated code snippets.

```
fn foo {
    import ad inline "1.2.3.devel";
    ad.foo()
}
```