# kvstore-db: High-Performance Key-Value Storage(NoSQL DB)

## Overview

`kvstore-db` is a robust key-value storage library that implements the Bitcask storage format, a cutting-edge approach to persistent data storage renowned for its performance, reliability, and simplicity.

## Background: The Bitcask Storage Format

### Origins in Riak
Bitcask was originally developed for Riak.
Riak, a NoSQL database, was developed during the height of the NoSQL movement and competed against similar systems such as MongoDB, Apache CouchDB, and Tokyo Tyrant. It distinguished itself with its emphasis on resilience to failure. Although it was slower than its peers, it guaranteed that it never lost data. That guarantee was enabled in part because of its smart choice of a data format. 

### Record Layout
Bitcask stores each record with a precise, well-defined structure:

![Bitcask Record Format](https://github.com/user-attachments/assets/c9c4a5f2-639d-4775-b029-071a1ae1fa36)
