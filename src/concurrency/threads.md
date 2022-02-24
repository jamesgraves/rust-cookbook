# Threads

Add these crates to your own project:

```
cargo add crossbeam crossbeam_channel
```

Other crates needed for examples:

```
cargo add image lazy_static num num_cpus ring threadpool walkdir
```

{{#include thread/crossbeam-spawn.md}}

{{#include thread/crossbeam-complex.md}}

{{#include thread/crossbeam-spsc.md}}

{{#include thread/global-mut-state.md}}

{{#include thread/threadpool-walk.md}}

{{#include thread/threadpool-fractal.md}}

{{#include ../links.md}}
