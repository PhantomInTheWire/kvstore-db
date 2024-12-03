# ActionKV: High-Performance Key-Value Storage

## Overview

`actionkv` is a robust key-value storage library that implements the Bitcask storage format, a cutting-edge approach to persistent data storage renowned for its performance, reliability, and simplicity.

## Background: The Bitcask Storage Format

### Origins in Riak
Bitcask was originally developed for Riak, a pioneering NoSQL database that emerged during the early 2010s NoSQL movement. While contemporaries like MongoDB and Apache CouchDB focused on speed, Riak distinguished itself with an unwavering commitment to data durability and resilience.

### What Makes Bitcask Special?

**Core Characteristics:**
- **Log-Structured Hash Table (LSHT)**: A storage paradigm optimized for high-performance writes and rapid key lookups
- **Append-Only Design**: Ensures data integrity and enables fast, sequential writes
- **Predictable Performance**: Provides consistent read and write latencies
- **Simple Recovery**: Straightforward mechanism for data reconstruction in case of system failure

### Technical Details

#### Record Layout
Bitcask stores each record with a precise, well-defined structure:

![Bitcask Record Format](https://github.com/user-attachments/assets/c9c4a5f2-639d-4775-b029-071a1ae1fa36)
