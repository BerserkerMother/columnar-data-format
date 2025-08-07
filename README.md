# Columnar Data Format

## What is it?
A data format for purpose of learning. It's purpose is to be fast and simple.

## Mistakes
- By the time of writing this paragraph, I have been rewriting the project four times. I can not get a clean interface for things. I have diagnose the problem however; I am trying to start from metadata! I should have started building the database it self and then getting to a good interface for metadata and serializing/deserializing would be more apparent!

## Insight
-  CPU cache lines is 64 bytes meaning it can read 64 bytes from memory at once. If we align things 64 bytes, we would get max performance since the SIMD register is also 64byte.
- We don't actually care about the memory null values take because it is a part of user data! Memory layout will be there for nulls values(I mad the mistake of omitting it before). To know if something is null or not, we simply store a bitmap where each bit indicates if the values at that index is null(0) or not(1).

## To read
- [x] When to use columnar data format. [link](https://www.tinybird.co/blog-posts/when-to-use-columnar-database)
- [ ] Log-structured merge-tree([LSTM](https://en.wikipedia.org/wiki/Log-structured_merge-tree))

## To do
- [x] ~Meta data creation~
- [ ] Read [Apache Arrow](https://arrow.apache.org/docs/format/Columnar.html) guide to columnar format implementation.
- [x] Make a bitvec kinda of thing for storeing nulls e.g. 0 or 1 for each record in a column.
- [ ] Serialize numeric and text types.

