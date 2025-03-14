## Display formatted date and time

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Gets and displays the current time in UTC using [`Utc::now`]. Formats the
current time in the well-known formats [RFC 2822] using [`DateTime::to_rfc2822`]
and [RFC 3339] using [`DateTime::to_rfc3339`], and in a custom format using
[`DateTime::format`].

```rust
{{#include examples/current_rfc_2822_3339.rs}}
```

[`DateTime::format`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.format
[`DateTime::to_rfc2822`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.to_rfc2822
[`DateTime::to_rfc3339`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.to_rfc3339
[`Utc::now`]: https://docs.rs/chrono/*/chrono/offset/struct.Utc.html#method.now

[RFC 2822]: https://www.ietf.org/rfc/rfc2822.txt
[RFC 3339]: https://www.ietf.org/rfc/rfc3339.txt
