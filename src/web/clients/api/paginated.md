## Consume a paginated RESTful API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Wraps a paginated web API in a convenient Rust iterator. The iterator lazily
fetches the next page of results from the remote server as it arrives at the end
of each page.

```rust,no_run
{{#include examples/reqwest_paginated.rs}}
```
Try modifying the example to change the page size, and have it print out a
debug message upon every GET request.
