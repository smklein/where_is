# where_is

[![Build status](https://github.com/smklein/where_is/workflows/ci/badge.svg)](https://github.com/smklein/where_is/actions)
[![](http://meritbadge.herokuapp.com/where_is)](https://crates.io/crates/where_is)

where_is provides tools for finding files within a filesystem hierarchy.

The primary entry point is the
[Finder](https://docs.rs/where_is/latest/where_is/struct.Finder.html) interface, which
converts to an iterator, returning all matching file entries.
