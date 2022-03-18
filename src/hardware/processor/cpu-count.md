## Check number of logical cpu cores

[![num_cpus-badge]][num_cpus] [![cat-hardware-support-badge]][cat-hardware-support]

Add the `num_cpus` crate to your own project:

```
cargo add num_cpus
```

Shows the number of logical CPU cores in current machine using [`num_cpus::get`].

```rust
{{#include examples/cpu-count.rs}}
```
