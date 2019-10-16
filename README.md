# Algorithms + Data Structures = Rust Programs
This repository implements data structures and algorithms from **Niklaus Wirth's** book **[Algorithms + Data Structures = Programs](goodreads-main-link)**.

Code comments often include citations from this book. Such citation includes a passage from the book, author's name, year of the publishing and page number.
```rust
///
/// > An array is a homogeneous structure; it consists of components
///     which are all of the same type, called the _base type_.
///     \
///     \
///     Niklaus Wirth 1976, 11
///
```

Each data structure and algorithm (further referred to as topics) is contained in their own module. Besides algorithm commentary, the code comments explain implementation. Tests are included to elaborate on how to use the API. They also help us validate the topics.

Each module is prefixed with an index. For example `a_001_binary_search.rs`. This is purely to organize the code repository. There are special modules which don't cover a standalone topic. They focus on integrating data structures with algorithms together.

## List of topics
- [binary search algorithm](src/topics/a_001_binary_search.rs)
- [sorting by straight insertion algorithm](src/topics/a_002_straight_insertion.rs)


<!-- Invisible List of References -->
[goodreads-main-link]: https://www.goodreads.com/book/show/25860569-algorithms-data-structures-programs
