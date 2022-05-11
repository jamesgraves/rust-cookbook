## Examine the date and time

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Gets the current UTC [`DateTime`] and its hour/minute/second via [`Timelike`]
and its year/month/day/weekday via [`Datelike`].

```rust
{{#include examples/current_utc.rs}}
```

[`Datelike`]: https://docs.rs/chrono/*/chrono/trait.Datelike.html
[`DateTime`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html
[`Timelike`]: https://docs.rs/chrono/*/chrono/trait.Timelike.html
