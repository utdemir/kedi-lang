# kedi-lang

## Syntax

```
fn sort arr
    : array? arr
    : arr.all? number?
    : sorted? @return
{
     range(0, arr.length).for_each |i| {
         range(i, arr.length).for_each |j| {
            if arr[i] > arr[j] {
                let temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
    }
}

: sort x == sort (sort x)
```

## Module system

```
import math;
import some.lib;
```

* Every top-level value is exported.
  * An '_' prefix is a convention for private values.
* Resolution order:
  * Directory of the file
  * KEDI_PATH
  * Compiler builtin modules
* Pattern:
  * Dots end up separate directories
  * It searches for files with `kedi.toml` extension.
  * If not, searches for directories with a `mod.kedi` file.