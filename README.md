# Columnar Data Format

## What is it?
A data format for purpose of learning. It's purpose is to be fast and simple.

## Mistakes
- By the time of writing this paragraph, I have been rewriting the project four times. I can not get a clean interface for things. I have diagnose the problem however; I am trying to start from metadata! I should have started building the database it self and then getting to a good interface for metadata and serializing/deserializing would be more apparent!

## Insight
- CPU cache lines is 64 bytes meaning it can read 64 bytes from memory at once. If we align things 64 bytes, we would get max performance since the SIMD register is also 64byte.
- We don't actually care about the memory null values take because it is a part of user data! Memory layout will be there for nulls values(I mad the mistake of omitting it before). To know if something is null or not, we simply store a bitmap where each bit indicates if the values at that index is null(0) or not(1).
- A very useful insight is to notice we actually can make our custom types and structs for certain features. To demonstrate further, imagine a Fixed<T> struct. What if we want to iterate over its elements? Should we just return the underlying Vec as ref? If we do so, we soon realize we can't know which element is null because we need the bitmap too. So we have two options(I guess):
  - return a Vec<Option<&T>> `pub fn get_records(&self) -> Vec<Option<&T>>`
  - return another struct like FixedViewer<T> which provides functionalities we need. For example, it could implement Iterator trait.
The problem with the first approach is three fold. First, we are paying for 8 more bytes(Option). Second, we are allocating and filling up a vector of number of records on every call to get_records. And Third, if our requirements change, we may not be able to handle the use case with Vec<Option<&T>>. On the other hand, FixedViewer<T> can be anything we want. It would just save a pointer to the original data and expose some functionalities that we want(more control == win);

## Tips
- When haivng a struct with generic interface like Fixed<T>, It is feasible to define concrete types like `type IntArray = Fixed<i32>` or `type FloatArray = Fixed<f32>`. I believe this makes a cleaner api for the consumer.
- Macro rules can make tedious tasks easier. For example, if we want to type alias as the above, we can define a macro which takes type name, native rust type, and docs if any to alias type automatically. It looks like `typedef!(UInt8Array, u8, "8-bit unsigned interger)` and of course it can be encapsulated more information or do more things depending on needs.

## To read
- [x] When to use columnar data format. [link](https://www.tinybird.co/blog-posts/when-to-use-columnar-database)
- [ ] Log-structured merge-tree([LSTM](https://en.wikipedia.org/wiki/Log-structured_merge-tree))

## To do
- [x] ~Meta data creation~
- [ ] Read [Apache Arrow](https://arrow.apache.org/docs/format/Columnar.html) guide to columnar format implementation.
- [x] Make a bitvec kinda of thing for storeing nulls e.g. 0 or 1 for each record in a column.
- [ ] Serialize numeric and text types.

