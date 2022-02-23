# Threads

Add these crates to your own project:

```
cargo add crossbeam crossbeam_channel
```

Other crates needed for examples:

```
cargo add lazy_static threadpool num image walkdir ring num_cpus
```

{{#include thread/crossbeam-spawn.md}}

{{#include thread/crossbeam-complex.md}}

{{#include thread/crossbeam-spsc.md}}

{{#include thread/global-mut-state.md}}

{{#include thread/threadpool-walk.md}}

{{#include thread/threadpool-fractal.md}}

{{#include ../links.md}}
