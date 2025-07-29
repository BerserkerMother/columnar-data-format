# Columnar Data Format

## What is it?
A data format for purpose of learning. It's purpose is to be fast and simple.

## Mistakes
- By the time of writing this paragraph, I have been rewriting the project four times. I can not get a clean interface for things. I have diagnose the problem however; I am trying to start from metadata! I should have started building the database it self and then getting to a good interface for metadata and serializing/deserializing would be more apparent!

## To read
- [ ] When to use columnar data format. [link](https://www.tinybird.co/blog-posts/when-to-use-columnar-database)
- [ ] Log-structured merge-tree([LSTM](https://en.wikipedia.org/wiki/Log-structured_merge-tree))

## To do
- [x] ~Meta data creation~
- [ ] Make a bitvec kinda of thing for storeing nulls e.g. 0 or 1 for each record in a column
- [ ] Serialize numeric and text types

