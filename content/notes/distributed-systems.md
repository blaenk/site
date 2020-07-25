+++
title = "Distributed Systems"
date = 2018-03-07
aliases = ["/notes/scalability"]

[note]
kind = "concept"
+++

Notes about the system design of scalable distributed systems from resources such as the amazingly wonderful book _Designing Data-Intensive Applications_ by Martin Kleppmann.

<nav id="toc"></nav>

# System Principles

## Reliability

Software can be considered to be reliable when it continues to work correctly even when things go wrong. The things that can go wrong are _faults_, so a system that anticipates faults and can cope with them is known as fault-tolerant or resilient. Faults can be caused by hardware, software, or humans.

## Scalability

Scalability is a system's ability to cope with increased load. In other words, if a system grows a particular way, what options are available to cope with that growth.

Load is often described by load parameters, such as requests per second to a webserver, ratio of reads to writes in a database, etc.

For example, Twitter's scalability challenge was with respect to its fan-out, where each user follows many other users, and each user is followed by many people. Requesting a user's home timeline can be accomplished in two main ways.

Look up all of the people they follow, find all of their recent tweets, and then merge them sorted by time.

``` postgresql
SELECT tweets.*, users.* FROM tweets
  JOIN users ON tweets.sender_id = users.id
  JOIN follows ON follows.followee_id = users.id
WHERE follows.follower_id = current_user;
```

Maintain a cache for each user's home timeline, like a queue. When a user posts a tweet, insert the new tweet into the home timeline cache of each user that follows them.

The second, queue approach is more efficient in Twitter's case because the average rate of published tweets is almost two orders of magnitude lower than the rate of home timeline reads, so it's preferable to do more work at write time than at read time.

The problem is that the number of followers per user highly varies, with an average of 75 followers, but others with more than 30 million followers, and there is a time constraint of delivering tweets to followers within 5 seconds. In this case, the distribution of followers per user, perhaps weighted by tweet frequency, is a useful load parameter since it corresponds to fan-out load. To compensate for this variability, Twitter is moving to a hybrid of both approaches, so that a small number of users with a very large number of followers are excepted from the queue approach. Instead, when a user requests their home timeline, the tweets from such celebrities followed by the user are fetched separately and merged with the regular queue approach.

Given load parameters, a system's scalability can be analyzed by increasing a load parameter and:

* with fixed system resources, and observing the system's performance
* determining how to increase system resources to maintain the same level of performance

Throughput is the number of records that can be processed per second, or the total time it takes to run a job on a dataset of a certain size.

A service's response time is the time between a client sending a request and receiving a response. The response time is what the client sees, which includes the time to process the request (service time) and network and queuing delays.

Response time should be thought of as a distribution of measurable values, not a single value, because individual response times may vary, either due to heavier requests, or random additional latency introduced by a context switch, TCP retransmission, garbage collection pause, etc.

To get an idea of a typical response time, it's better to use percentiles than an arithmetic mean.

Higher percentiles are even more important in backend services that are called multiple times as part of serving a single end-user request, since, even if they're called in parallel, the end-user has to wait for the slowest of the calls to complete. Even if only a small percentage of backend calls are slow, the chance of getting a slow call increases with an increase in backend calls, so a higher proportion of end-user requests end up being slow. This is known as _tail latency amplification_.

Latency is the duration that a request is waiting to be handled, during which it is latent (awaiting service).

An architecture is unlikely to cope with ten times its usual load, so it may be necessary to re-think an architecture of a fast-growing service at least on every order of magnitude load increase, if not more.

A shared nothing architecture is one in which load is distributed across multiple machines.

Elastic systems are ones that can automatically add computing resources when they detect a load increase, instead of manually.

It's important to avoid premature optimization for scaling to some hypothetical future load, since, if the assumptions for common operations and load parameters end up being wrong, the time will be wasted or counter-productive. Quicker iteration should be emphasized.

## Maintainability

Maintainability refers to making it easier to work on the system in the future.

Three design principles to keep in mind for software systems are operability, simplicity, and evolvability.

### Operability

Operability refers to the ease of keeping a system running smoothly.

An operations team may be responsible for:

* monitoring system health and restoration
* identifying causes of problems
* keeping software up to date, e.g. security patches
* taking note of how different systems affect each other, to prevent problems before they cause damage
* anticipating future problems, e.g. capacity planning
* establishing best practices, e.g. for deployment and configuration management
* performing maintenance tasks, e.g. moving application to another platform
* defining operations processes
* preserving system knowledge

Good operability is facilitated by:

* visibility into system internals and runtime behavior
* automation and standard tool integration
* avoiding machine dependencies, allowing any to be taken down
* documenting
* straightforward operational model, e.g. "If I do x, y will happen"
* good defaults, but freedom to override
* sensible self-healing with option for manual control of system state
* minimize surprises through predictable behavior

### Simplicity

Simplicity refers to the ease of understanding the system, achieved by removing as much complexity as possible.

### Evolvability

Evolvability refers to the ease of making changes in the future, such as to adapt to unanticipated use cases.

# Data Models and Query Languages

Applications are typically built by layering data models, where each layer is represented in terms of the next-lower layer. For example:

1. data structures, objects, and APIs
2. JSON, database tables, graph model
3. bytes in memory, disk, or network
4. electrical currents, pulses of light, magnetic fields

Each data model embodies assumptions about how it's used, facilitating certain use cases while not supporting others. As a result, it often determines what the software above it can do, so the choice of data model is application specific.

The use of traditional relational databases alongside non-relational (NoSQL) databases is sometimes called _polyglot persistence_.

There is an impedance mismatch between the relational model and mainstream object-oriented programming due to the perceived awkward translation layer between objects and tables, rows, and columns.

Data normalization has many benefits:

* consistent style and spelling of the data
* avoiding ambiguity between duplicates
* ease of updating
* localization support
* better search

Document stores like MongoDB, RethinkDB, CouchDB, and Espresso typically store self-contained denormalized JSON documents instead of storing normalized data and then joining the disparate data in queries as in traditional relational databases. In this case, JSON has better locality than multi-table schemas.

Normalizing data requires many-to-one relationships which aren't as natural to express in the document model. If the document store doesn't support joins, they have to be emulated in application code with multiple queries to the database. The intermediary results may not even fit in memory before they're filtered, so special care needs to be taken. It's possible that the initial data model fits well in a join-free document model, but eventually becomes more interconnected.

## Network Model

The network model as standardized by the Conference on Data Systems Languages (CODASYL) was a generalization of the hierarchical model. In the hierarchical model, every record has exactly one parent, forming a tree structure. In the network model, a record can have multiple parents, allowing many-to-one and many-to-many relationships.

The links between models aren't foreign keys, but more like data pointers. Accessing a record requires following an _access path_ from a root record. A query was performed by moving a cursor through the database, iterating over lists of records and following access paths. The application had to keep track of the relationships when a record had multiple parents.

Adding access paths required going through handwritten database query code and rewriting it to handle the new access paths.

## Relational Model

The relational model typical has a query optimizer that transparently decides the order and manner in which to execute a query, such as which indexes are used, effectively automatically constructing the "access path" that would be manually constructed in the network model.

Even though query optimizers are complicated, incurring many years of research and development, they only have to be written once and then all applications using the database can benefit from it.

## Document Model

Document databases are more like the hierarchical model in the sense that they store nested records (one-to-many) within their parent record rather than in a separate table. Document databases can be useful when data comes in self-contained documents and relationships between them are rare. Representing many-to-one or many-to-many relationships can still be done through a unique identifier known as a _document reference_---similar to a foreign key in the relational model---which is resolved at read-time by using a join or follow-up queries.

The document model claims better schema flexibility, better performance due to locality, and a closer match to the application data structures.

The document model can be a good fit if an application's data has a document-like structure where the entire tree is loaded at once.

A limitation of the document model is the inability to refer directly to a nested item within a document. Instead, it is referred by saying for example "the second item in the list of positions for user 251."

Another limitation of the document model is when the application data uses many-to-many relationships. Joins may be reduced by denormalization but that increases the complexity of keeping the denormalized data consistent. Joins can be emulated in application code through multiple requests but that increases application complexity and is often slower than a join performed by the database. Overall these issues can lead to more complexity and decreased performance.

## Graph Model

When many-to-many relationships are very common in data, where anything is potentially related to anything, it can be useful to model the data as a graph.

Graphs aren't limited to homogeneous data. At Facebook for example, vertices can represent people, locations, events, and comments, whereas edges can represent friendship and who commented on which post.

Graphs are amenable to evolvability because they don't have a schema specifying which record types can be nested within which other record type.

### Property Graph Model

In the property graph model, each vertex consists of:

* unique identifier
* set of outgoing edges
* set of incoming edges
* collection of key-value pair properties

Each edge consists of:

* unique identifier
* vertex at which the edge starts (tail vertex)
* vertex at which the edge ends (head vertex)
* label describing the relationship
* collection of key-value pair properties

### Triple-Store Model

The triple-store graph model is equivalent to the property graph model. All information is stored in the form of very simple three-part statements consisting of subject, predicate, and object.

For example, in (Jim, likes, bananas), the subject is "Jim", the predicate (verb) is "likes", and the object is "bananas".

The subject is a equivalent to a vertex.

The object can be a value of a primitive data type such as a string or number, in which case the predicate and object are equivalent to a key and value property. For example, (lucy, age, 33) could mean that the subject "lucy" has an "age" property of "33".

The object can also be another subject (vertex) in the graph, in which case the predicate is an edge in the graph and the subject and object are tail and head vertices respectively.

Datomic is an example of a triple-store.

## Schema-on-Read vs Schema-on-Write

Document databases are sometimes considered _schemaless_ but the truth is that although there isn't an explicit schema enforced by the database upfront, there _is_ an implicit schema assumed by readers of the data. A more accurate term may therefore be _schema-on-read_, whereas _schema-on-write_ might refer to an explicit, upfront schema that is used to ensure that writes conform to it. Schema-on-read is similar to dynamic type checking performed at run-time whereas schema-on-write is similar to static type checking performed at compile-time.

### Schema Flexibility

As a result, changes to the structure of the data in a document model can be made instantly by ensuring that the application can handle the data before and after the change. For example, adding a field in documents can be done lazily by adding the field for each processed document if it didn't already exist.

Comparatively, changes in a relational database require a schema migration which updates the schema and updates all of the existing data to reflect that new schema. Schema changes don't need to be slow nor do they need to require downtime.

Most relational databases can perform `ALTER TABLE` statements in a few milliseconds, but running `UPDATE` statements can be slow on a very large table since it causes every row to be rewritten. One optimization is perform the data migration lazily by allowing a default value of `NULL` on new columns and filling them in at read time, as with a document database field addition.

A schema-on-read approach is useful when dealing with heterogeneous data, such as when the structure of the data is determined by external systems which are subject to change at any time.

### Data Locality

Since a document is usually stored as a self-contained document, it has storage locality, so an application that routinely requires access to the entire document will benefit compared to looking-up multiple indexes and joining multiple tables.

Reading an entire document can be wasteful if only a portion of it needs to be accessed, so the data locality may be a moot point.

Updating a document also often requires rewriting the entire document, unless the modification doesn't change the encoded size of the document.

A common guideline to mitigate these issues is to strive to keep documents small, and avoid writes that increase document size.

### Data Model Convergence

There appears to be a convergence of document and relational models towards each other.

Most relational databases support schema-on-read data types such as XML or JSON.

Some document databases now support relational-like joins, such as RethinkDB, and certain MongoDB drivers which automatically resolve database references by performing a client-side join.

## Query Languages

### Structured Query Language

SQL is a declarative query language, which specifies what data to get rather than how to get it, which gives the query optimizer the flexibility to determine the best way to obtain that data while taking performance into account.

### MapReduce Queries

The MapReduce programming model can be used to process large amounts of data in bulk across many machines, based on the `map` and `reduce` functions. These functions must be pure functions, which allows them to be called on any machine in any order as many times as necessary (e.g. due to failure).

MapReduce is fairly low-level and can be used to implement higher-level query languages like SQL.

An example MongoDB MapReduce query will:

1. filter the desired data declaratively with the `query` key
2. call the `map` function for each document matching the `query`, setting `this` to the matched document
3. have the `map` function emit a key and a value
4. group the emitted key-value pairs by key, then call the `reduce` function one for all key-value pairs with the same key
5. the result is written to a collection named by the `out` key

``` javascript
db.observations.mapReduce(
  function map() {
    var year  = this.observationTimestamp.getFullYear();
    var month = this.observationTimestamp.getMonth() + 1;

    emit(year + "-" + month, this.numAnimals);
  },
  function reduce(key, values) {
    return Array.sum(values);
  },
  {
    query: { family: "Sharks" },
    out: "monthlySharkReport"
  }
);
```

Since MapReduce isn't declarative and loses out on a query optimizer's ability to improve the performance of a query, MongoDB also supports a JSON-based declarative query language known as the aggregation pipeline.

The previous MapReduce query would look like this:

``` javascript
db.observations.aggregate([
  { $match: { family: "Sharks" } },
  { $group: {
    _id: {
      year:  { $year:  "$observationTimestamp" },
      month: { $month: "$observationTimestamp" }
    },
    totalAnimals: { $sum: "$numAnimals" }
  } }
]);
```

### Graph Queries

#### Property Graph Queries

The Cypher Query Language is a declarative query language for property graphs created for the Neo4j graph database.

This example creates a graph, first defining the vertices then the edges.

```
CREATE
  (NAmerica:Location {name:'North America', type:'continent'}),
  (USA:Location      {name:'United States', type:'country'  }),
  (Idaho:Location    {name:'Idaho',         type:'state'    }),
  (Lucy:Person       {name:'Lucy' }),
  (Idaho) -[:WITHIN]->  (USA)  -[:WITHIN]-> (NAmerica),
  (Lucy)  -[:BORN_IN]-> (Idaho)
```

This example is a query that finds people who emigrated from the USA to Europe. First, it finds a person that has an outgoing `BORN_IN` edge to a vertex and from that vertex there is a chain of zero or more `WITHIN` edges eventually leading to a `Location` vertex whose `name` property is `"United States"`. Second, the person found in the first step must also have an outgoing `LIVES_IN` edge to a vertex which has a chain of zero or more outgoing `WITHIN`edges leading to a `Location` vertex whose name property is `"Europe"`. Finally, for such a person, return the `name` property.

```
MATCH
  (person) -[:BORN_IN]->  () -[:WITHIN*0..]-> (us:Location {name:'United States'}),
  (person) -[:LIVES_IN]-> () -[:WITHIN*0..]-> (eu:Location {name:'Europe'})
RETURN person.name
```

### Triple-Store Queries

The Turtle triple-store format, a subset of Notation3, can be used to describe a triple-store graph.

Vertices are named with the syntax `_:someName` and are only for identification purposes within the format, but don't exist otherwise.

``` turtle
@prefix : <urn:example:>.
_:lucy     a       :Person.
_:lucy     :name   "Lucy".
_:lucy     :bornIn _:idaho.
_:idaho    a       :Location.
_:idaho    :name   "Idaho".
_:idaho    :type   "state".
_:idaho    :within _:usa.
_:usa      a       :Location.
_:usa      :name   "United States".
_:usa      :type   "country".
_:usa      :within _:namerica.
_:namerica a       :Location.
_:namerica :name   "North America".
_:namerica :type   "continent".
```

The same data can be written more concisely by writing multiple relationships for a given subject on the same line.

``` turtle
@prefix : <urn:example:>.
_:lucy     a :Person;   :name "Lucy";          :bornIn _:idaho.
_:idaho    a :Location; :name "Idaho";         :type "state";   :within _:usa.
_:usa      a :Location; :name "United States"; :type "country"; :within _:namerica.
_:namerica a :Location; :name "North America"; :type "continent".
```

The SPARQL query language for triple-stores using the RDF data model.

``` sparql
PREFIX : <urn:example:>

SELECT ?personName WHERE {
  ?person :name ?personName.
  ?person :bornIn  / :within* / :name "United States".
  ?person :livesIn / :within* / :name "Europe".
}
```

#### Datalog

Datalog provides the foundation for later query languages. It is used as the query language of Datomic, and Cascalog is a Datalog implementation for querying large datasets in Hadoop. Datalog is a subset of Prolog, but Datomic and Cascalog use a Clojure s-expression syntax for Datalog.

Datalog's data model is similar to the triple-store model albeit generalized so that `(subject, predicate, object)` is written as `predicate(subject, object)`.

The previous data can be represented as Datalog facts.

``` prolog
name(namerica, 'North America').
type(namerica, continent).

name(usa, 'United States').
type(usa, country).
within(usa, namerica).

name(idaho, 'Idaho').
type(idaho, state).
within(idaho, usa).

name(lucy, 'Lucy').
born_in(lucy, idaho).
```

A Datalog query defines rules that tell the database about new predicates. In the following case, the two new predicates are `within_recursive` and `migrated`. These rules are derived from data or from other rules, since rules can refer to other rules just like functions can call other functions or themselves recursively. In rules, words that start with uppercase letters are variables that can match any value.

For example, `name(Location, Name)` matches the triple `name(namerica, 'North America')`, binding the variable `Location` to `namerica` and `Name` to `'North America'`.

A rule applies if the system can find a match for all of its predicates on the right-hand side of the `:-` operator, then it's as if the left-hand side of the `:-` operator was added to the database, with the variables replaced by the values they matched.

The following rules could be matched as:

1. rule 1 applies because `name(namerica, 'North America')` exists, so this generates and adds `within_recursive(namerica, 'North America')` to the database
2. rule 2 applies because `within(usa, namerica)` exists and step 1 generated `within_recursive(namerica, 'North America')`, so this generates and adds `within_recursive(usa, 'North America')` to the database
3. rule 2 applies because `within(idaho, usa)` exists and step 2 generated `within_recursive(usa, 'North America')`, so this generates and adds `within_recursive(idaho, 'North America')`

Then rule 3 can find people born in some location `BornIn` and living in some location `LivingIn`. Then querying with `BornIn` set to `'United States'` and `LivingIn` set to `'Europe'` leaving the name of the person as a variable `Who` asks the Datalog system to find out which values can appear for that variable `Who`.

``` prolog
/* Rule 1 */
within_recursive(Location, Name) :- name(Location, Name).

/* Rule 2 */
within_recursive(Location, Name) :- within(Location, Via),
                                    within_recursive(Via, Name).

/* Rule 3 */
migrated(Name, BornIn, LivingIn) :- name(Person, Name),
                                    born_in(Person, BornLoc),
                                    within_recursive(BornLoc, BornIn),
                                    lives_in(Person, LivingLoc),
                                    within_recursive(LivingLoc, LivingIn).

?- migrated(Who, 'United States', 'Europe').
/* Who = 'Lucy'. */
```

Datalog is less convenient for simple queries, but scales better with the data's complexity.

#### SQL Graph Queries

SQL can also be used to graphs represented in relational databases, but the number of joins that will be necessary to traverse a path in a graph is not known upfront. Therefore in order to achieve these arbitrarily repeated queries, recursive common table expressions are required using the `WITH RECURSIVE` directive.

The length and complexity of the graph traversal emphasizes how certain data models are better suited for different use cases. Graph models naturally match graph traversal operations, whereas relational models don't, so the same operation is much more awkward in a relational model.

``` postgresql
WITH RECURSIVE

  -- in_usa is the set of vertex IDs of all locations within the United States
  -- 1. find vertex whose `name` property is `"United States"` and make it the first element in `in_usa`
  -- 2. follow all incoming `within` edges from vertices in `in_usa` and add them to the same `in_usa` set, until all incoming `within` edges have been visited
  in_usa(vertex_id) AS (
      SELECT vertex_id FROM vertices WHERE properties->>'name' = 'United States'
    UNION
      SELECT edges.tail_vertex FROM edges
        JOIN in_usa ON edges.head_vertex = in_usa.vertex_id
        WHERE edges.label = 'within'
  ),

  -- in_europe is the set of vertex IDs of all locations within Europe
  -- 3. do the same steps 1-2 with the vertex whose name property is `"Europe"`
  in_europe(vertex_id) AS (
      SELECT vertex_id FROM vertices WHERE properties->>'name' = 'Europe'
    UNION
      SELECT edges.tail_vertex FROM edges
        JOIN in_europe ON edges.head_vertex = in_europe.vertex_id
        WHERE edges.label = 'within'
  ),

  -- born_in_usa is the set of vertex IDs of all people born in the US
  -- 4. for each vertex in `in_usa`, follow incoming `born_in` edges to find all people born anywhere within the United States
  born_in_usa(vertex_id) AS (
    SELECT edges.tail_vertex FROM edges
      JOIN in_usa ON edges.head_vertex = in_usa.vertex_id
      WHERE edges.label = 'born_in'
  ),

  -- lives_in_europe is the set of vertex IDs of all people living in Europe
  -- 5. for each vertex in `in_europe`, follow incoming `lives_in` edges to find all people who live anywhere in Europe
  lives_in_europe(vertex_id) AS (
    SELECT edges.tail_vertex FROM edges
      JOIN in_europe ON edges.head_vertex = in_europe.vertex_id
      WHERE edges.label = 'lives_in'
  )

-- 6. intersect the people born in the USA with the set of people living in Europe by joining them to find those born in the USA and living in Europe
SELECT vertices.properties->>'name'
FROM vertices
JOIN born_in_usa     ON vertices.vertex_id = born_in_usa.vertex_id
JOIN lives_in_europe ON vertices.vertex_id = lives_in_europe.vertex_id;
```

# Storage and Retrieval

An index is a separate structure derived from the primary data, so maintaining it has an overhead, such as on writes, since the index has to be updated as well. Appropriate indexes speed up read queries but slow down writes, which is why databases don't index everything by default.

## Log-Structured Storage

An append-only file storing key-value pairs can be indexed with a hash index so that every key is mapped to the byte offset of the data in the file. This hash index would have to be updated whenever a new key-value pair is appended. When a value needs to be read, the hash index can be keyed to obtain the offset into the data file at which the record begins, then the file seeked to that location and the record read.

This file-append key-value store is resilient, providing high-performance reads and writes _as long as_ all keys fit in memory; values don't need to fit in memory since they can be read from disk, if they're not already in the filesystem cache. This storage system is suited to situations where the values may be updated frequently, but there aren't many distinct keys.

This system is used in Bitcask, the default storage engine of Riak.

One way to avoid running out of disk space is to break the log file into segments of a certain size, closing it when it reaches that size and writing subsequent writes to a new segment file.

The segments can then undergo _compaction_, which discards duplicate keys in the log, keeping only the most recent update for each key. This is also known as garbage collection.

Since compaction usually decreases the size of segments, certain small segments can then be merged into a regular sized segment file, usually at the same time as compaction is performed.

Merging and compaction can be done from a separate thread while read requests continue to be served from the old segment files. When the separate thread finishes merging and compaction, it can atomically swap out the old segment files for the new segments so that new requests use them instead.

Since there are multiple files (segments), each segment needs its own in-memory hash table mapping the keys to the file offsets. A value is then obtained by checking in the most recent segment's hash table, and if absent, proceeding to the next segment's hash table. The merging and compaction process ensures that the number of segments remains small, which by extension keeps small the number of hash tables that need to be checked.

<!-- Advantages -->

Deleting records can be accomplished by appending a special deletion record, sometimes known as a _tombstone_, which indicates to the merging and compaction process to discard any previous values for the deleted key.

For crash recovery, the in-memory hash tables can be reconstructed by reading each segment file from beginning to end and adding the keys and their offsets to an in-memory hash table. Alternatively, snapshots of the in-memory hash tables can be stored on disk and simply loaded into memory.

Partially written records can be avoided by including checksums in the file. Partially in-place updated records are not a problem since there are no in-place updates.

Since records are written in strictly sequential order, there is usually one writer thread, but since segments are append-only and immutable, there can be concurrent multiple readers.

Append-only writing and segment merging are sequential write operations which are much faster than random writes on hard drives and even solid state drives.

The merge process avoids the problem of data files becoming fragmented over time.

<!-- Disadvantages -->

One problem is that the hash table must fit in memory, which is problematic when there are a very large number of keys.

Another problem is that range queries are not efficient, since each key needs to be looked up individually in separate hash maps.

## Log-Structured Merge-Trees

A sorted string table (SSTable) keeps the sequence of key-value pairs in the segment file sorted by key, and requires that each key appears only once per segment file, something that is already guaranteed by the merge process.

Merging segments is more efficient even if the files don't fit in memory, since merging is as straightforward as with [merge sort]'s merge process. Since adjacent segments are merged, duplicate keys can be ignored besides the most recent occurrence.

[merge sort]: /notes/algorithms#merge-sort

A full index of all keys no longer needs to be kept in memory. Instead the index can be sparse, only storing keys occurring every few kilobytes in the segment file. When a key is looked up and absent from the index, the file range where it should occur in the segment file is sequentially scanned.

In fact, since several key-value pairs will need to be scanned within the requested range, that range can be grouped into a block and compressed before writing it to disk. Each entry in the sparse index would then point to the start of such a compressed block.

The key-value pairs in a segment file are kept sorted by first writing them into an in-memory balanced tree structure often referred to as a _memtable_, such as a [red-black tree].

[red-black tree]: /notes/algorithms#red-black-trees

When the memtable reaches a certain size threshold, it's written out to disk in sorted order as the newly most recent SSTable file segment. New writes can continue to be written to a new memtable as the previous one is being written out to disk.

To read a value, the key is first checked in the memtable, then the most recent on-disk segment, and the next, and so on.

The merging and compaction process running in the background combines segment files and discards overwritten or deleted values.

When the system crashes, the most recent writes in the memtable that haven't been written to disk are lost. This can be mitigated by keeping a separate write-ahead log on disk where each write is immediately appended. When writing out a memtable the log is discarded and started anew. When restoring from a crash, the log is played back to recreate the state of the memtable.

Range queries can be performed efficiently since the data is stored in sorted order.

This indexing structure is known as a log-structured merge-tree (LSM-Trees). Similar systems are used in LevelDB, RocksDB, Cassandra, HBase, and Google BigTable. The full-text search indexing engine Lucene, used by Elasticsearch and Solr, uses something similar for its term dictionary, where the key is a term (word) and the value is a list of IDs of documents where it occurs (postings list).

LSM-tree lookup can be slow for non-existent keys, since the memtable and all segments need to be checked before determining that the key does not exist. Storage engines often use a [bloom filter] to determine if a key is not in the set.

[bloom filter]: /notes/algorithms#bloom-filter

The two most common strategies for determining the order and timing of how SSTables are compacted and merged are _size-tiered_ and _leveled_ compaction.

Size-tiered compaction works by successively merging newer and smaller SSTables into older and larger SSTables.

Leveled compaction works by splitting up the key range into smaller SSTables, with older moved into separate "levels," allowing compaction to proceed more incrementally and use less disk space.

RocksDB and LevelDB use leveled compaction, which is the source of the latter's name. HBase uses size-tiered compaction, and Cassandra supports both.

## B-Trees

B-Trees are balanced trees which allow efficient key-value lookups and range queries.

B-Trees break data into fixed-size blocks, also called pages, traditionally 4 KB in size, and read/write a page at a time. Pages are identified with an address or location, allowing pages to refer to each other. Leaf pages are at the bottom of the tree and they contain the values themselves or references to pages where the values can be found.

Unlike LSM-trees which only ever append writes and never modify files in-place, B-Trees modify pages in-place by overwriting them with new data, preserving all references to the page.

Since some B-Tree operations require overwriting multiple pages, such as due to splitting a page, crashes can leave the B-Tree in a corrupted state, such as with orphan pages which have no parent. This is often mitigated by an on-disk _write-ahead log_ (WAL), aka redo log, which is an append-only file to which every modification is written to _before_ it is applied to the tree pages. The database can then recover from a crash by replaying the modifications back to a consistent state.

Instead of overwriting pages and maintaining a WAL, some databases such as LMDB use a copy-on-write strategy, so that pages are copied, modified, and written to a different location, then the parents are also copied and modified with the pointers updated to point to the new page [^persistent_data_structures]. This strategy also works well with concurrency.

[^persistent_data_structures]: This reminds me of persistent data structures.

Careful concurrency control is required to coordinate the in-place modification of pages from multiple threads, otherwise a certain thread may see the tree in an inconsistent state. This is often accomplished with _latches_ (light-weight locks).

Pages can save space by abbreviating keys, storing just enough information to act as boundaries between key ranges, allowing a tree to have a higher branch factor and thus fewer levels. This optimization was originally part of a B+ Tree, but the optimization is now very commonplace.

Many B-Tree implementations try to lay out a tree so that leaf pages appear in sequential order in order to optimize scans over a large part of a key range in sorted order. This layout is difficult to maintain as the tree grows.

Many B-Tree implementations add pointers to leaf pages referencing sibling pages to optimize scanning keys in order without having to traverse back through parent pages.

Fractal Trees are B-Tree variants which borrow log-structured ideas to reduce disk seeks.

## LSM-Trees vs B-Trees

<!-- Advantages -->

LSM-Trees are usually faster for writes and B-Trees are thought to be faster for reads. LSM-Tree reads are often slower due to having to check various different structures and SSTables at different stages of compaction. However, specific workloads should be tested.

A B-Tree index has to write every piece of data to both the write-ahead log and the tree page itself, and perhaps further if the pages are split, as well as the overhead of having to write entire pages at a time even if few bytes have changed. In fact, certain storage engines write the page twice to avoid partially updated pages in case of power loss. However, LSM-Trees also rewrite the same data multiple times due to compaction and merging of SSTables.

_Write amplification_ refers to the effect in which a single write to a database results in multiple writes to disk over the database's lifetime. This is especially concerning on SSDs which can only overwrite blocks a limited number of times. The more a storage engine writes to disk the lower write throughput it can handle with the given disk bandwidth. LSM-Trees can usually sustain higher write throughput because they sometimes have lower write amplification and because they sequentially write compact SSTable files rather than overwriting several tree pages.

LSM-Trees compress better and so take up less space on disk than B-Trees, which inherently leave space unused due to fragmentation.

Most SSD firmware internally use a log-structured algorithm to turn random writes into sequential writes in the underlying hardware, minimizing the impact of a storage engine's write pattern. Nevertheless, lower write amplification and fragmentation are still beneficial since representing data compactly allows higher read and write throughput given the available bandwidth.

<!-- Disadvantages -->

LSM-Tree compaction can sometimes interfere with the performance of current reads and writes, leading to the response time of queries to log-structured storage engines to be high at higher percentiles, whereas B-Trees are often more predictable.

Finite disk write bandwidth needs to be shared between an initial write (logging and flushing a memtable to disk) and compaction threads. If write throughput is high, it is possible that the compaction process can't keep up with the rate of incoming writes, so that the number of unmerged segments keeps growing until disk space is exhausted, causing slower disk reads since they more and more segment files need to be checked. SSTable-based storage usually doesn't throttle the rate of incoming writes for compaction to keep up, so this situation needs to be monitored.

B-Trees can be better suited to strong transactional semantics by simply placing locks on ranges of keys by attaching them to the tree, whereas LSM-Trees can contain multiple copies of the same key in different segments.

## Alternative Indexes

### Secondary Indexes

In a secondary index, the indexed values are not necessarily unique. Key entries are made unique either by making each value entry be a list of rows containing that key entry as a column value or by making each key entry unique by appending a row identifier to it.

Secondary indexes are often crucial for performing joins efficiently.

### Heap Files and Non-Clustered Indexes

Indexes can store as value a reference to the row stored elsewhere in a _heap file_, which stores the rows in no particular order, possibly append-only. This approach avoids duplicating data when multiple secondary indexes exist, since each index simply references a location in the heap file.

Updating a value can be done in place if the new value is not larger, but if it is, the value probably needs to be moved to a different location within the heap where it may fit, requiring all indexes to be updated to point to the new heap location. Alternatively, a forwarding pointer can be left at the old location.

The extra indirection incurred by a heap file can be a performance penalty for reads.

### Clustered and Covering Indexes

A _clustered index_ is an index which stores the row directly within the index. For example, MySQL's InnoDB storage engine sets up the primary key index as a clustered index and secondary indexes refer to the primary key rather than a heap file location.

A _covering index_ (aka "index with included columns") is a compromise between a clustered index and a non-clustered index, storing only some of the table's columns within the index, allowing certain queries to be answered with the index alone, in which case the index "covers" the query.

Clustered and covering indexes can speed up reads but require additional storage and they also increase write overhead. Additional effort is also necessary to prevent observed inconsistencies due to the data duplication, in order to enforce transactional guarantees.

### Multi-Column Indexes

Querying multiple columns of a table can benefit from multi-column indexes.

A _concatenated index_ simply concatenates several fields into one key, with the index specifying the concatenation order.

A _multi-dimensional index_ provides a more general way of querying several columns simultaneously such as for geospatial data. A restaurant search feature may require finding all restaurants by their latitude and longitude within a rectangular area representing the map area the user is viewing. Traditional indexes can either give all restaurants in a range of latitudes or in a range of longitudes, but not both.

One way to resolve this is by mapping a two-dimensional location into a single number using a space-filling curve.

More appropriately, a specialized spatial index based on structures such as R-Trees is used. PostGIS implements geospatial indexes with R-trees using PostgreSQL's Generalized Search Tree indexing feature.

Multi-dimensional indexes aren't only for geospatial data, they can also be used for color range searching, or efficiently finding all weather observations during a year within some temperature range, for example.

### Full-text Search Fuzzy Indexes

In Lucene, the in-memory index is a finite state automaton over the characters in the keys, similar to a trie. The automaton can be transformed into a Levenshtein automaton which can search for words within a given edit distance.

## In-Memory Databases

In-memory databases are kept entirely in memory. Some in-memory databases can be durable by writing a log of changes to disk, or periodic snapshots to disk, or by replicating the in-memory state to other machines. Restarting an in-memory databases requires reloading its state either from disk or from a replica. Examples include VoltDB, MemSQL, Oracle TimesTen, and RAMCloud. Redis and Couchbase provide weak durability by writing to disk asynchronously.

Even though some in-memory databases may write a log to disk, they are still considered in-memory because it's only for durability purposes, and all reads are still served entirely from memory.

The main performance advantage of an in-memory database compared to a traditional one is not that disk doesn't need to be read during normal operation, since with enough memory, that data may already be cached in memory by the operating system. Rather, the performance advantage comes from not needing to serialize in-memory data structures.

In-memory databases can actually store more data than fits in memory by using an _anti-caching_ approach which evicts least-recently used (LRU) data from memory to disk when memory is needed, then loading it back when it's accessed again, similar to virtual memory and swap files of operating systems, except at the granularity of individual records rather than entire memory pages. However, the indexes must fit in memory.

## Analytic Processing

A transaction has come to generally mean a group of reads and writes that form a logical unit, although not necessarily with ACID (atomicity, consistency, isolation, durability) properties. Transaction processing simply means that low-latency reads and writes are possible, as opposed to batch processing jobs which only run periodically.

Online Transaction Processing (OLTP) is an access pattern marked by interactive use of a database based on a user's input.

By comparison, analytic queries need to scan over very large numbers of records, only reading a few columns per record, usually calculating aggregate statistics such as sum or average, instead of returning raw data to a user. Analytic queries can answer questions such as:

* What was the total revenue of each of store in a given month?
* How many more bananas were sold during the last promotion?
* Which brand of baby food is most often purchased together with a given brand of diapers?

Analytic queries are usually written by business analysts and fed into reports that help the company make better decisions, a process known as business intelligence.

The access pattern of analytic queries is called Online Analytic Processing (OLAP).

The main read pattern of OLTP is a small number of records per query, fetched by key, whereas OLAP performs an aggregate calculation over a large number of records.

The main write pattern of OLTP is random-access, low-latency writes based on user input, whereas OLAP is based on bulk import (ETL) or event stream.

OLTP is primarily used by the end user via a web application, whereas OLAP is primarily used by an internal analyst.

OLTP usually represents the latest state of data at a current point in time, whereas OLAP maintains a history of events that happened over time.

OLTP dataset sizes usually range from gigabytes to terabytes, whereas for OLAP they range from terabytes to petabytes.

### Data Warehouses

A _data warehouse_ is a different kind of database that better matches OLAP access patterns. It is a separate database that analysts can query without affecting OLTP operations. It contains a read-only copy of the data from the various OLTP databases, extracted by a periodic data dump or continuous stream of updates. The data is transformed into an analysis-friendly schema, cleaned up, then loaded into the data warehouse in a process known as _Extract-Transform-Load_ (ETL).

Data warehouses exist in almost all large enterprises, but are almost unheard of in smaller companies, since they have a comparatively small amount of data that can easily be queried through a conventional database.

Although many data warehouses are relational, the internals of the systems can be very different because they are optimized for different query patterns, such that most database vendors now focus on either transaction processing or analytics.

Existing vendors include Teradata, Vertica, SAP HANA, and ParAccel. Recently there have been open source SQL-on-Hadoop projects that aim to compete with commercial data warehouses, such as Apache Hive, Spark SQL, Cloudera Impala, Facebook Presto, Apache Tajo, and Apache Drill.

Many data warehouses are used in a formulaic style known as a _star schema_ (aka _dimensional modeling_). When a star schema is visualized, the fact table sits in the middle surrounded by its dimension tables like rays of a star.

At the center of the schema is a _fact table_ where each row is an event that occurred at a particular time, such as a sale in a `fact_sales` table.

Some columns are attributes, but many others are foreign key references to _dimension tables_, such that a row in a fact table is an event, and a dimension table represents extra details, such as a product in a `dim_product` dimension table which would include columns for the SKU, description, brand name, etc. Even seemingly simple information such as date and time are often stored in separate dimension tables to allow encoding additional information about the dates, such as whether it was a holiday.

A variation of a star schema is a _snowflake schema_ where dimensions are further broken down into subdimension tables, such as `dim_product` referencing a separate `dim_brand` table. Although more normalized than star schemas, star schemas are often preferred for being easier to work with.

### Column-Oriented Storage

Tables in a data warehouse are often very wide, with fact tables often having over 100 columns, and sometimes several hundred. However, typical queries only access 4 or 5 of the columns at one time.

OLTP databases usually store data in a row-oriented fashion, with all values from one row of a table stored next to each other. Even if indexes exist on the columns being queried, a row-oriented storage engine needs to load all of the rows, with all of their attributes, from disk into memory, then parse them, and only then filter out those that don't meet the required conditions.

In _column-oriented storage_, all of the values from each column are stored together. Each column is stored in a separate file, allowing a query to only read and parse those columns that are used in the query. For this to work, each column file needs to contain the rows in the same order, so that reassembling a row is as simple as taking the same entry number in each column file.

Column-oriented storage lends itself to compression. _Bitmap encoding_ can be used to compress the column values when the number of distinct values is small compared to the number of rows. A column with `$n$` distinct values can be turned into `$n$` separate bitmaps, one for each distinct value, with one bit for each row, so that the bit for that row is 1 if the row has that value and 0 if not. The bitmaps in total will be much smaller than storing the entire list of column values.

If `$n$` is very small then the bitmaps can be stored with one bit per row, otherwise they will be sparse, with a lot of zeros in most of the bitmaps. In this case, they can be further compressed with run-length encoding.

Bitmap indexes are well suited for common data warehouse queries. Some operations can be reduced to bitwise operations, because the bitmaps contains the rows in the same order.

``` postgresql
WHERE product_sk IN (30, 68, 68);
```

This query would load the three bitmaps for each `product_sk` referenced and calculate the bitwise OR of all of them.

``` postgresql
WHERE product_sk = 31 AND store_sk = 3;
```

This query would load the two bitmaps and calculate the bitwise AND.

A bottleneck for analytic queries is the bandwidth for getting data from disk into memory, as well as the bandwidth from main memory to the CPU cache.

Column-oriented storage layouts make efficient use of CPU cycles. The query engine can take a chunk of compressed column data that fits in the L1 cache and iterate in a tight loop without function calls. Column compression allows more rows from a column to fit in the same amount of L1 cache. In fact, certain operations can operate on chunks of compressed column data directly.

A column store can benefit from storing the rows sorted by a commonly accessed column, such as a date column, by speeding up those reads. Storing columns in some sorted order also facilitates compression of the column, since the primary sort column will have long sequences of the same value which lends itself well to run-length encoding, so that a sorted column in a table with billions of rows can be compressed down to a few kilobytes. Even if second and third sort keys may not has as many repeated values, highly compressing the first sort column, which is ideally the most accessed, is a huge performance gain.

If different queries benefit from different sort orders, it's also possible to store the same data in several different orders. If the data needs to be replicated anyway, the redundant data can be stored in different ways so that the best version for a particular query can be picked.

Writing to a sorted column store can appear to be difficult since inserting a row into the middle of a sorted table would require rewriting all column files in order to maintain consistent row ordering within the column files. However, an LSM-Tree can be used to back the column files to maintain a consistent row order despite inserts. Queries will then need to examine both the column data on disk and the recent writes in memory and combine the two, but that is hidden by the query optimizer.

### Materialized Data Cubes

Since analytic queries often involve aggregate functions such as `COUNT` and `AVG`, it can be wasteful to calculate them every time. The results can instead be cached in a _materialized view_, which is like a standard virtual view whose contents are the results of a query, except a materialized view actually contains the copy of the query results on disk, whereas a virtual view is just a shortcut for writing queries. Reading from a virtual view entails expanding the view's underlying query and processing it, whereas reading from a materialized view entails reading the copy of the saved results.

As with an index, a materialized view needs to be updated when the underlying data changes. This can be done automatically, but since the updates can be expensive, materialized views aren't usually used in OLTP databases, but they make more sense in more read-heavy data warehouses.

A _data cube_ or _OLAP cube_ is a materialized view that is a grid of aggregates grouped by multiple different dimensions. Certain queries become very fast through the use of a materialized data cube because they have been precomputed. Data cubes aren't as flexible as querying raw data because queries are limited to the information within the cube, so an effort is made to keep as much raw data as possible while creating data cubes to optimize certain queries as needed.

For example, a two-dimensional data cube for "date" (e.g. `date_key`) and "product" (e.g. `product_sk`) would contain in each cell the aggregate result (e.g. `SUM`) of an attribute of all facts (e.g. `fact_sales.net_price`) with that date-product combination.

A date-product cube can have `product_sk` columns and `date_key` rows. Each cell would be constructed by a query such as:

``` postgresql
SELECT SUM(fact_sales.net_price)
FROM fact_sales
WHERE date_key = row_num AND product_sk = column_num;
```

The aggregate can then be applied further on rows or columns of the cube to reduce by one dimension, such as a product's sales regardless of date or date's sales regardless of product.

Aggregating (e.g. `SUM`) an entire `date_key` row in the data cube would correspond to the total `SUM` of `net_price` for a given `date_key` regardless of `product_sk`.

``` postgresql
SELECT SUM(fact_sales.net_price)
FROM fact_sales
WHERE date_key = row_num;
```

Aggregating (e.g. `SUM`) an entire `product_sk` column in the data cube would correspond to the total `SUM` of `net_price` for a given `product_sk` regardless of `date_key`.

``` postgresql
SELECT SUM(fact_sales.net_price)
FROM face_sales
WHERE product_sk = column_num;
```

# Data Encoding and Evolution

Schema evolution refers to the changes made to a data format or schema over time, usually precipitated by changes to an application's code. Changes usually consist of new fields or record types, or existing data needs to be presented in a new way.

Schema evolution requires great care in situations where different versions of a system coexist simultaneously, with an older system expecting the old format and a newer system wanting to use the new format but needing to handle the old format as well. This situation is increasingly common.

A _rolling upgrade_ (aka _staged rollout_) is a process by which a new version of an application is deployed to a few nodes at a time, ensuring that the process is running smoothly, and gradually continuing to deploy to all nodes, allowing deployment without downtime, thereby encouraging more frequent releases and better evolvability. Flawed releases can be detected early and rolled back before they affect large numbers of users.

In client-side applications the user may not install an update right away, creating an old system that may need to continue to work with the updated data.

In both situations, old and new systems must coexist simultaneously, and they will expect old or new data schemas, requiring both backward and forward compatibility.

_Backward compatibility_ means that new code can read old data. This is straightforward to achieve because new code can be aware of old data formats and can be made to explicitly handle it.

_Forward compatibility_ means that old code can read new data. This is more difficult to achieve because it requires old code to ignore the additions made by new code.

## Textual Encoding Formats

Language-specific serialization formats such as `java.io.Serializable` or Python `pickle` are tied to a language and are at best left to very transient purposes. They may lead to security vulnerabilities if arbitrary classes can be instantiated. Versioning is often weak or non-existent. Efficiency in CPU and encoded structure size is often an afterthought.

Textual formats such as JSON, XML, and CSV have subtle issues.

There is ambiguity with number encoding. XML and CSV can't distinguish between a number and a string consisting of digits without referring to a schema. JSON doesn't distinguish between integers and floating-point numbers. Integers greater than 2^53 cannot be exactly represented in IEEE 754 double-precision numbers used by JavaScript, leading to inaccuracies when parsed by JavaScript. This lead Twitter to return 64-bit integer Tweet IDs twice in API responses: once as a JSON number and again as a decimal string.

Byte sequences aren't supported. This is usually mitigated by encoding binary data as Base64 text, with a schema indicating that it should be interpreted as Base64 encoded data, but it increases data size by 33%.

Although optional schemas are supported by XML and JSON. XML schema use is common, but many JSON tools ignore schemas, leaving the correct interpretation of the data to be hard-coded into the application.

Regardless, textual formats are popular as data interchange formats.

## Binary JSON and XML Formats

There are binary encodings for JSON (MessagePack, BSON, BJSON, UBJSON, BISON, Smile) and XML (WBXML, Fast Infoset). Some extend the datatypes but otherwise keep the data model. Since they don't describe a schema, each field name is included in the encoded data. It's unclear if the marginal space and parse time reductions that most binary encodings of JSON yield are worth the loss in human-readability.

In MessagePack, for example, the first byte indicates the type and count/size of the following data. If this information doesn't fit in a byte, a separate type indicator is used and the field count is expressed in 16 or 32 bits.

Generally, data is laid out as:

1. object/array indicator
2. data type/length indicator
3. field name
4. data

## Apache Thrift and Google Protocol Buffers

Apache Thrift (originally created at Facebook) and Google Protocol Buffers (protobuf) are fully binary encodings. A schema is required in order to encode data.

Thirft's Interface Definition Language (IDL) for describing a schema looks like:

``` thrift
struct Person {
  1: required string userName;
  2: optional i64 favoriteNumber;
  3: optional list<string> interests;
}
```

Protocol Buffers schema definition language looks like:

``` protocol-buffer
message Person {
  required string user_name = 1;
  optional int64 favorite_number = 2;
  repeated string interests = 3;
}
```

Thrift and Protocol Buffers both have a code generation tool that reads the schema and generates code which can encode or decode records of the schema. This can be a nuisance with dynamically typed languages since they can introduce an unnecessarily compilation step.

Thrift has two binary encoding formats called BinaryProtocol and CompactProtocol (DenseProtocol is only supported by C++).

Thrift's BinaryProtocol is similar to MessagePack's format except that instead of including the field names it includes _field tags_, which are the numbers specified in the schema definition used to identify fields, leading to a more compact encoding. Field tags are necessary to differentiate fields because field order can't be used since unset field values are omitted from the encoded data.

Generally, struct records are laid out as:

1. data type
2. field tag
3. length
4. data

A struct record is finally ended with a null 0x00 byte.

Thrift's CompactProtocol is similar to the BinaryProtocol except that it packs the field type and tag number into a single byte, and uses variable-length integers instead of full 64-bit, 8-byte integers, where the top bit of each byte is used to indicate whether there are still more bytes in the number.

Google Protocol Buffers encode in a manner similar to Thrift's CompactProtocol.

Optional and required field markers have no effect on the encoded data, except for a run-time check for required fields to ensure that they are present.

A field's tag can't be changed since existing encoded data would become invalid.

Regarding schema evolution, new fields can be added as long as each new field has a tag number. Old code can ignore the new fields since it doesn't know about those tag numbers.

Thrift and Protocol Buffers can be forward compatible by giving new fields tag numbers, so old code can ignore fields they don't know about, since they don't know the corresponding field numbers. The field data type information can be used to determine how many bytes in the record to skip. This allows old code to read records written by new code.

Thrift and Protocol Buffers can be backward compatible as long as field tag numbers aren't modified and new fields aren't marked required, since new code would fail to read old data that didn't provide the new field. This means that all new fields must be optional or must have a default value. This allows new code can continue to read old data fields.

Only optional fields can be removed. Removed fields' tags can never be used again, since there may remain data with those tag numbers. New code must ignore removed fields.

Protocol Buffers have a `repeated` marker instead of a generic list or array data types. Since repeated fields are just the same field repeated, it's fine to change an single-valued optional field into a multi-valued repeated field. It's backward compatible since new code reading old data will read it as a list of zero or one elements. It's forward compatible since old code reading new data only sees the _last_ element.

Thrift generic list data types don't support this schema evolution feature, but they do support nested lists.

## Apache Avro

Apache Avro is a binary encoding format that started from Hadoop since Thrift was not a good fit. Avro supports two schema languages: Avro IDL and JSON. Avro doesn't identify fields or their data types. Avro encoding simply consists of concatenated values.

``` avro
record Person {
  string userName;
  union { null, long } favoriteNumber = null;
  array<string> interests;
}
```

``` json
{
  "type": "record",
  "name": "Person",
  "fields": [
    {
      "name": "userName",
      "type": "string"
    },
    {
      "name": "favoriteNumber",
      "type": [
        "null",
        "long"
      ],
      "default": null
    },
    {
      "name": "interests",
      "type": {
        "type": "array",
        "items": "string"
      }
    }
  ]
}
```

Since Avro encoding is just a series of concatenated values without identification. Avro has a notion of a _writer schema_ used to encode the data and a _reader schema_ used to decode the data. The reader and writer schemas _only_ have to be compatible---they don't have to be identical.

The Avro library resolves differences between the writer and reader schema by translating data from the writer's schema into the reader's schema, as specified by the Avro specification. This allows writer and reader schema fields to be in different orders, since Avro's schema resolution matches them by name. Readers ignore fields not present in their schema and fill in missing expected fields with default values specified in their schema.

Avro is forward compatible because old readers can read new writers, and backwards compatible because new readers can read old writers. However, this is only possible if the only fields that are added or removed are those with default values.

Changing the data type of a field is only possible if Avro can convert the types. Changing names of fields is possible if the reader schema has aliases for field names, allowing it to match an old writer's schema field names against those aliases, but this is only backward compatible and not forward compatible, since old readers won't know the new names.

Avro union fields can take on a value of any type specified in the union. A type that can be set to `null` as well as another value must include `null` in the union type. A union field's default value must be of the type of the first branch in the union.

Adding a branch to a union type is backward compatible but not forward compatible, since old readers won't be able to interpret the additional branch types.

In large files with many records the writer schema can be written to the beginning of the file. Then readers can read the schema from the file and use it to decode the contained records. Avro object container files work this way.

In a database with different records written over time, writer schemas can be versioned within the database and each written record can be tied to a schema version which can be used to read it back. Espresso from LinkedIn works this way.

With network communications, schema versions can be negotiated before record communication. Avro RPC works this way.

In general, versioning schemas within a database can be useful as documentation and as a way to check schema backward and forward compatibility before deployments. The schema version can be an incrementing number or a hash of the schema.

One advantage of not writing field tag numbers explicitly is that schemas can more easily be dynamically generated, something which is further facilitated through Avro's JSON schema language.

This can be useful, for example, to dump a database' contents into a format generated for each database table, with each column corresponding to a field in a table record. If the database schema changes, a new schema can be generated and the database can continue to be dumped. This would not be as straightforward with Thrift or Protocol Buffers, since they would have to keep field tags consistent across database schema changes.

## Database Dataflow

Even if there were only a single process that writes to and reads from a database, it'd essentially be equivalent to sending a message to one's future self. Backward compatibility would be important so that the future self (newer reader) could decode the data written by the present self (older writer).

Realistically there will be many different applications or instances of the same application using the same database simultaneously, so both backward _and_ forward compatibility are important.

It's possible for old code to read new data that contains a new field and corresponding value, ignoring that field, only to write the data back without that new field value. Instead the old code should keep the new fields intact, even if it couldn't interpret them. The aforementioned encoding formats handle this, but it may need to be handled at the application level as well. An ORM might lose fields in the object-relational mapping process, so that then writing the objects back won't include the lost data.

_Data outlives code_ because deploying a new application version is generally much easier than deploying a new version of a database schema. Migrating (rewriting) the existing data into a new schema can be expensive on a large dataset, so it's generally avoided. Simple changes such as adding a new column with a default null value are generally possible without rewriting existing data, so that nulls are filled into the returned data when the record is read and is missing data for the field.

When taking database snapshots, it may be beneficial to encode them consistently instead of with the schema used at the time of the dump, by encoding them in a format like Avro object container files.

## Service Dataflow

In a service data flow, one process sends a request and expects a response as quickly as possible.

A design goal of service-oriented architectures (SOA), microservices, is to make an application easier to change and maintain by making component services independently deployable and evolvable, without having to coordinate with other services. This presupposes having old and new versions of services and clients running at the same time, so the data encoding between them must be compatible across API versions.

Remote Procedure Calls (RPC) tries to make network requests appear as regular function calls through the _location transparency_ abstraction. RPC tends to be used for communication between services within the same organization. Hiding the location can be problematic because:

* A network request may be lost or the connection may be slow or unavailable, so requests may need to be retried.

* Simply resending a request may result in multiple requests being received if they actually are getting through but the responses are getting lost, which can result in the same action being performed multiple times if there isn't an idempotance mechanism to deduplicate requests.

* Latency is variable. The same identical calls may take different times to complete.

* Client and Server may be implemented in different languages, allowing for inconsistencies, such as with JavaScript numbers.

Apache Thrift and Apache Avro have RPC support, and gRPC is an RPC built on top of Google Protocol Buffers. These RPC libraries are a new generation of RPC which explicitly acknowledge the network request rather than attempt to hide it.

It is reasonable to expect that all servers are updated before all clients, so only backward compatibility on requests (new servers handling old client requests) and forward compatibility on responses (old clients handling new server responses) are necessary. These compatibility qualities are inherited from the encoding format.

Any necessary compatibility-breaking changes are achieved only by maintaining multiple versions of the service API simultaneously, usually accomplished in the case of REST APIs by including the version number in the URL or an `Accept` HTTP header.

## Message-Passing Dataflow

In an asynchronous message-passing data flow, a client _message_ is sent to an intermediary _message broker_ (aka _message queue_) which stores it temporarily.

The communication process involves a process sending a message to a _queue_ (aka _topic_) which the broker ensures is delivered to one or more _subscribers_ (aka _consumers_) to that queue or topic. Each topic may have many producers and consumers.

Message-passing communication is asynchronous and usually unilateral, so a sender doesn't usually expect to receive a response, although it's possible by using a separate response channel. Instead it's "fire and forget."

Messages are usually just sequences of bytes with some metadata, so any encoding format may be used. Making the encoding backward and forward compatible allows independently changing and deploying publishers and consumers in any order. However, republishing messages to another topic needs to take care not to lose unknown fields as in [the case with databases].

[the case with databases]: #database-dataflow

A message broker:

* can improve system reliability by acting as a waiting queue when the recipient is unavailable or overloaded

* can automatically redeliver messages to processes that crashed, preventing message loss

* decouples the sender from the recipient, so that they don't have to know each others IP address and port

* allows a single message to be sent to several recipients

### Distributed Actor Model

In the actor model, logic is encapsulated in actors which may have private local state and which communicate with each other through asynchronous messages whose delivery is not guaranteed.

In the distributed actor model, the actor model is scaled across multiple nodes, so that if two actors are on separate nodes, the messages are transparently encoded and transmitted. Local transparency works better here than in RPC because the actor model already assumes that messages may be lost even within a single process, and because there is less of a fundamental mismatch between local and remote actors: both must send messages to communicate with each other.

Care must be taken to preserve backward and forward compatibility, and this is dependent on the encoding format used by the actor framework. Akka uses Java's built-in serialization by default, but can be replaced with Protocol Buffers. Erlang OTP makes it difficult to make changes to record schemas.

# Distributed Data

A _shared-memory architecture_ is one where many components (CPU, RAM, disks) are shared within a single machine under a single operating system. However, modern CPU architectures feature nonuniform memory access (NUMA), so some memory banks are closer to one CPU than others. Making efficient use of this architecture requires breaking the data down for each CPU to access nearby memory, so that partitioning is still required even if within a single machine [^cache_coherence].

[^cache_coherence]: [Cache coherence] reminds me a lot about data replication and consistency in its effort to maintain memory consistency.

[cache coherence]: /notes/computer-architecture#cache-coherence

_Vertical scaling_ (aka _scaling up_) is scaling by upgrading a machine to a more powerful one. The cost of vertical scaling grows faster than linearly, so a machine with twice the CPUs, RAM, and disk capacity is significantly more than twice as expensive. Even so, bottlenecks within a single machine may prevent it from handling twice the load. Cost limits this architecture's scalability.

A _shared-disk architecture_ is one where several machines with independent CPUs and RAM share an array of disks connected via a fast network---Network Attached Storage (NAS) or Storage Area Network (SAN). This architecture is sometimes used for data warehousing workloads. Locking contention and overhead limit this architecture's scalability.

A _shared-nothing architecture_ (aka _horizontal scaling_, _scaling out_) is one where each machine has its own independent CPUs, RAM, and disks, and coordination is done at the software level with a conventional network. This allows geographical distribution of data and processing to reduce latency for users in different locations.

Two common ways to distribute data across nodes is through replication and partitioning. Both may be combined; are _not_ mutually exclusive. For example, two partitions can be replicated so that there are two replicas per partition.

_Replication_ refers to keeping a copy of the _same_ data on different nodes, providing redundancy in the event of nodes becoming unavailable.

_Partitioning_ refers to splitting a database into smaller _partitions_ and assigning each to different nodes (aka _sharding_).

# Replication

Replicated data has the benefit that it can be geographically collocated, increase availability, and increase read throughput.

The difficulty involved with replication relates to handling _changes_ to the replicated data. After all, immutable data can simply be copied to every node once.

Three popular algorithms for replicating changes between nodes are single-leader, multi-leader, and leaderless replication. Since the fundamental constraints of networks have remained the same since the 1970s, the principles of replication haven't changed much.

A _replica_ is a node that stores a copy of the database. Every write needs to be processed by every replica so that they contain the same data, otherwise they would no longer be replicas.

## Leader-Based Replication

_Leader-based replication_ (aka _active/passive replication_, _master-slave replication_) works by designating one of the replicas as the _leader_ (aka _master_ or _primary_) and all writes are sent to and performed through it,  while reads can be processed by the leader or any follower. The other replicas are known as _followers_ (aka _read replicas_, _slaves_, _secondaries_, _hot standbys_). New writes that the leader completes are also propagated to all of the followers through a _replication log_ or _change stream_. Each follower then applies all writes in the same order as were processed on the leader, as specified in the log, to update its local copy of the data.

Many applications feature leader-based replication, including relational databases such as PostgreSQL, MySQL, Oracle Data Guard, SQL Server's AlwaysOn Availability Groups, non-relational databases like RethinkDB, Espresso, and message brokers like Kafka and RabbitMQ.

_Synchronous replication_ waits until the followers confirm that they have received the write before making it visible to other clients and signaling a successful write.

_Asynchronous replication_ sends writes to followers but _doesn't_ wait for a response.

Replication is usually completed within a second, but there is no guarantee. For example, a follower may fall behind the leader by several minutes if it's recovering from a failure, operating at near maximum capacity, or if there are network problems, so it has to process other writes before it can process the latest write.

Synchronous replication guarantees that followers' copies of the data are up-to-date and consistent with the leader's. However, if a follower doesn't respond, writes block and can't be processed until it becomes available, so it is impractical to make all followers synchronous since any one node could halt the entire system.

A _semi-synchronous_ configuration is a more common configuration where one follower is synchronous and the rest are asynchronous. If the synchronous follower becomes slow or unavailable, one of the asynchronous followers becomes synchronous, ensuring that consistent data exists on at least two nodes.

Leader-based replication is often _fully asynchronous_ so that any non-replicated writes are lost if a single leader fails and is not recoverable, with the advantage that the leader can continue processing even if all followers have fallen behind. This configuration is widely used when there are many followers or they are geographically distributed.

Creating a new follower can usually be done without downtime by:

1. Taking a consistent snapshot of the leader's data at some point in time, preferably without taking a lock on the entire database.
2. Transferring the snapshot to the follower node.
3. Connecting the follower to the leader and requesting all data changes made since the snapshot was taken, which requires knowing the snapshot's position in the replication log, known as a _log sequence number_ in PostgreSQL.
4. Applying all of the data changes that have been made since the snapshot was made.
5. The follower is now _caught up_ and it proceeds with regular follower behavior by continuing to process streamed data changes as they happen.

When a follower fails, since it keeps a log of data changes received from the leader, it can simply continue processing its log once it has recovered and request all data changes that occurred while it wasn't available. Once caught up, it can proceed with regular follower behavior.

_Failover_ is when a leader fails and a follower is promoted to be the new leader. Clients must be reconfigured to send writes to it and other followers must begin consuming data changes from it.

Automatic failover can occur by:

1. Determining when a leader has failed, such as through a timeout.
2. Appointing a new leader either through an election process or through a previously elected _controller node_. the best candidate is usually the one that is most up-to-date so as to minimize data loss.
3. Reconfiguring the system to use the new leader.

Various issues can occur with automatic failover. These issues lead some operations teams to prefer to perform failovers manually.

In an asynchronous replication configuration, the new leader may not have received all changes before the old leader failed. If the old leader becomes available again, those changes present on the old leader are usually simply discarded, since the new leader may have already received writes which would conflict with that old data.

However, discarding writes can be dangerous if coordination is required with external storage systems, such as Redis. At GitHub, an outdated follower was promoted to leader, so auto-incrementing primary keys were "reissued" so far as Redis was concerned, which lead to Redis serving cached data associated with the old leader's primary keys for requests involving the new leader's "reissued" primary keys, leaking private data.

A _split brain_ situation is where two nodes believe they are the leader, each accepting writes. This is dangerous if there is no conflict resolution process. This can happen for example if the old leader goes down and then becomes available again thinking that it's still the leader. The system needs to ensure that the old leader acknowledges the new leader and by becoming a follower or shutting down, known as _Shoot The Other Node In The Head_ (STONITH). Care must be taken to not end up with both nodes shut down.

A timeout duration needs to be carefully considered because a longer timeout means a longer recovery time (more time before the recovery process begins), but a shorter timeout could lead to unnecessary failovers being initiated. A shorter timeout could possibly be exceeded if the node is under heavy load or network problems, in which case an unnecessary failover can exacerbate problems.

## Statement-Based Replication

In _statement-based replication_, the leader logs every write request statement (e.g. in SQL: `INSERT`, `UPDATE`, `DELETE`) that it executes and forwards them to followers, each of which parse and execute the statement.

There are potential problems which can cause the data on each replica to diverge and become inconsistent.

One problem with statement-based replication is that any non-deterministic functions calls (e.g. `NOW()` or `RAND()`) may generate different values on each replica. More generally, statements with side-effects (e.g. triggers, stored procedures, user-defined functions) can result in different side-effects on each replica.

Another problem is that statements that rely on existing data (e.g. auto-incrementing columns, `UPDATE … WHERE …`) must be executed in the same order on each replica or they may have different effects. This can be a bottleneck with multiple concurrently executing transactions.

Although there are workarounds for some problems, there are too many edge cases making other replication methods more favorable.

## Write-Ahead Log Shipping

The same append-only write-ahead log kept by an LSM-Tree or B-Tree can be used to build a replica on another node, so that besides writing the log to disk it's also sent to followers over the network, so that followers processing the log end up building a copy of the exact same data structures found on the leader. This is done on PostgreSQL and Oracle.

The caveat is that the log describes data at a very low level, detailing which bytes were changed in which disk blocks. This ends up tightly coupling to the storage engine, so that it's typically not possible to run any two different versions of the database on the leader and followers.

Zero-downtime database upgrades would usually be accomplished by performing a rolling upgrade of followers, then performing a failover from the now-outdated leader to an updated follower.

Since WAL shipping is tightly coupled to the storage engine, if the replication protocol doesn't allow a version mismatch, zero-downtime upgrades aren't possible with WAL shipping.

If the replication protocol doesn't allow a version mismatch---as is common with WAL shipping---and since WAL shipping is tightly coupled to the storage engine and is therefore likely to be incompatible between database versions, zero-downtime upgrades may be impossible with WAL shipping replication.

## Logical Log Replication

_Logical log replication_ (aka _row-based replication_) usually consists of a sequence of records describing writes at table row granularity. It is considered _logical_ to distinguish from replication of the _physical_ data representation.

A logical log is decoupled from the storage engine internals, so it is more easily backward compatible, allowing for leader and followers to run different versions, thereby facilitating zero-downtime upgrades. Decoupling from the storage engine may even enable different nodes to run different storage engines.

Logical log formats are usually easier for external applications to parse, facilitating _change data capture_: the process of sending database contents to an external system such as data warehouses or custom indexes and caches.

The logical log records may consist of:

* Inserts: The new values of all columns
* Deletions: Information to uniquely identify the row, such as primary key, or the values of all columns if there is no primary key
* Updates: Information to uniquely identify the row and the new values of all columns (or those that changed)
* Transactions: Transactions that modify several rows generate several records followed by a record indicating that the transaction was committed

## Trigger-Based Replication

_Triggers_ are a way to register custom application code that is executed when a data change (write transaction) occurs.

Trigger-based replication is an _application-layer replication_, and can be used to replicate a subset of the data, to replicate from one type of database to another, or to use custom conflict resolution logic.

It works by having triggers that then log the change into a separate table which is read by an external process, which can apply any necessary application logic to replicate the data change to another system. This is done by Databus for Oracle and Bucardo for PostgreSQL.

Application-layer replication can also be achieved through other tools like Oracle GoldenGate which can read database logs to make them available to an application.

Trigger-based replication often has more overhead and is more error-prone than built-in replication, but can nonetheless be very flexible.

## Replication Lag

A _read-scaling architecture_ is one where there are many followers that can process read-only queries. It realistically only works with asynchronous replication, since with synchronous replication a single node failure would make the entire system unavailable for writing. The more nodes there are, the more likely this may happen.

On the other hand, reads from an asynchronous follower may yield outdated information if the follower has fallen behind, leading to apparent inconsistencies where running the same query at the same time on leader and follower yields different results. However, this inconsistency is temporary, since all followers are expected to catch up eventually, leading to _eventual consistency_.

_Replication lag_ refers to the delay between a write happening on the leader and being reflected on a follower. It may be a fraction of a second in practice, but if the system is under heavy load or there is a problem on the network, it can increase to seconds or even minutes.

## Read-After-Write Consistency

In asynchronous replication a situation could arise where a user makes a write that modifies some data (processed by the leader) which they then read back afterward (processed by a replica), but before the new data has finished replicating to the replica processing the read, thereby appearing to the user as if the write was lost.

_Read-after-write consistency_ (aka _read-your-writes consistency_) is a guarantee that users can read the effects of _their own_ writes.

Possible ways to implement read-after-write consistency include:

* Assign leader to process reads of data that the user may have modified. However, this requires knowing what data may have been modified, or can simply entail assuming it for certain data, such as a user's own profile information.

* If most things are potentially editable by the user, the above approach would negate read scalability, since most reads would be processed by the leader. In this case another approach can be to read all recently-modified data from the leader.

* Clients can remember the timestamp of their most recent write, which the system can use to ensure that reads reflect updates at least up until that timestamp. If the replica is not caught up to that timestamp, the read can be processed by another replica or it can wait until the replica has caught up. The timestamp can be a logical timestamp or system clock timestamp, the latter of which requires clock synchronization.

* Distributing replicas across multiple datacenters has additional complexity, since any request that needs to be served by a leader needs to be routed to the datacenter containing it.

A similar situation can occur with one user accessing data from multiple devices, in which case _cross-device read-after-write consistency_ may be necessary.

In this case, timestamp-based read-after-write becomes more difficult since one device doesn't know what updates have occurred on the other, requiring the metadata to be centralized.

Furthermore, read-from-leader read-after-write becomes more difficult since datacenter-distributed replicas don't guarantee that connections from different devices will be routed to the same datacenter, which can especially occur when connecting through a cell phone's data network, requiring requests to be routed from all of a user's devices to the same datacenter so that they end up assigned to the same leader.

## Monotonic Reads

In asynchronous replication a situation could arise where a user appears to observe changes moving backward in time if they read from different replicas, such as one to a follower with little lag and then again to a follower with greater lag. This can happen for example on a web site if refreshing a page is routed to a random server.

For example, one user may insert new data then another user may make two read queries of that data. The first query would hit a follower with little lag and so the user would see the inserted data, but the second query would hit the second follower with greater lag which hasn't received and processed the write, so the user would not see the inserted data.

_Monotonic reads_ guarantee that after a user has seen data from one point in time, they shouldn't later see it from an earlier point in time.

It is less of a guarantee than strong consistency but a stronger guarantee than eventual consistency. Reading data may still yield an old value, monotonic reads simply guarantees that if one user makes several reads in sequence, they will not read older data after having previously read newer data.

One implementation method for monotonic reads is to ensure that a user always reads from the same replica. This can be accomplished by choosing the replica based on a hash of the user ID rather than randomly, but this would require rerouting if the assigned replica fails.

## Consistent Prefix Reads

In asynchronous replication a situation could arise that appears to violate causality, particularly in partitioned (sharded) databases. This can occur if certain partitions are replicated slower than others, so that an observer may see a later write before an earlier one.

_Consistent prefix reads_ is a guarantee that ensures that if a sequence of writes happens in a certain order, then anyone reading the writes will see them appear in the same order. In particular, the state should make causal sense.

This wouldn't happen if the database always applied writes in the same order, so that reads always saw a consistent prefix. But with partitioned databases, partitions oftentimes operate independently so that there is _no global ordering of writes_, and therefore no consistent prefix, so a user may see some parts of the database in an older state and some other parts in a newer state.

One solution is to ensure that causally related writes are written to the same partition, but in some applications this is inefficient.

There are also certain algorithms can track causal dependencies.

## Replication Lag Solutions

When working with eventually consistent systems, it's worth considering what will happen when replication lag increases to several minutes or hours, instead of treating replication as synchronous when in fact it is asynchronous. If this poses a problem, then a stronger guarantee such as read-after-write may be necessary.

## Multi-Leader Replication

In _multi-leader replication_, (aka _master-master_ or _active/active replication_) each node that processes a write must forward that data change to all of the other nodes, with each leader acting as a follower of all other leaders.

The benefits rarely outweigh the added complexity of multi-leader replication when used within a single datacenter.

It makes more sense to use multi-leader replication in a multi-datacenter environment, to tolerate datacenter failure or for proximity co-location, for example.

With a single-leader configuration in a multi-datacenter deployment, every write must be processed synchronously over the internet to the datacenter with the leader, making it very sensitive to Internet problems. In the event of datacenter failure, a new leader in another datacenter can be promoted.

With a multi-leader configuration in a multi-datacenter deployment, every write is processed synchronously by the local datacenter's leader and is then replicated asynchronously to the other datacenter leaders. In the event of datacenter failure, each datacenter can continue operating independently, and replication can catch up once the failed datacenter recovers.

The major downside to multi-leader replication is the fact that write conflicts may occur when the same data is modified concurrently in two different datacenters, requiring conflict resolution.

Multi-leader replication is often retrofitted onto a database, causing surprising interactions with other database features such as auto-incrementing keys, triggers, and constraints, making it dangerous. Multi-leader replication should be avoided if possible.

Applications with offline support can be modeled as multi-leader replicating systems, where each device has a local database acting as a leader with asynchronous multi-leader replication between other replicas.

### Write Conflicts

Writes may succeed on their local leaders only to be detected as write conflicts at a later time, after asynchronous replication has completed.

The simplest way to deal with conflicts is to avoid them, such as by ensuring that all writes for a particular record go through the same leader. For example, a user's writes can always be routed to the same leader, appearing to be single-leader. However, this adds difficult if it needs to change due to datacenter failure or because the user has moved.

Databases must resolve conflicts in a _convergent_ way meaning that they must arrive at the same final value when all changes have been replicated. This can be achieved by:

1. Giving each _write_ a unique ID, then pick the write with the highest ID. Using a timestamp is known as _last write wins_ (LWW). Prone to data loss.
2. Give each _replicate_ a unique ID, then pick the write originating from the replica with the lowest ID. Prone to data loss.
3. Merge the values somehow, e.g. concatenation.
4. Record the conflict and all of its information, and use application code to resolve it at a later time, perhaps by asking the user.

Most multi-leader replication tools allow writing conflict resolution logic in application code, which can be executed on write or on read:

* On write as soon as the database detects a conflict in the log of replicated changes.
* On read. When a conflict is detected, all conflicting writes are stored. Next time it is read, the multiple versions of the data are returned, then the resolved value can be written back.

An important fact is that conflict resolution usually applies at row or document-level, regardless of whether they are part of a transaction.

There are active areas of research into conflict resolution, such as:

* **Conflict-free replicated datatypes** (CRDTs): data structures that can be used concurrently while automatically resolving conflicts via two-way merges
* **Mergeable persistnent data structures**: tracking history and using a three-way merge function, similar to Git
* **Operational transformation**: Designed for concurrent editing of ordered lists of items, such as a list of characters (i.e. a document). Used by collaborative editing applications like Google Docs

### Multi-Leader Replication Topologies

A _replication topology_ describes the communication paths for write propagation between nodes.

* **All-to-all topology**: every leader sends its writes to every other leader.

* **Circular topology**: each node receives writes from one node and forwards the combination of those writes and its own writes to another node.

* **Star topology**: a designated root node forwards writes to all other nodes. It can be generalized to a tree.

In circular and star topologies, the nodes need to forward writes, so the flow of replication messages can be interrupted if just one node fails, requiring topology reconfiguration to reroute around the failed node.

To prevent infinite replication loops, each node has a unique identifier which is used to tag each write when it has passed through a node, so that a node can ignore a write that is tagged with its own identifier.

A single node's failure can interrupt the flow of replication messages between other nodes in a circular and star topology.

More densely connected topologies have better fault tolerance since they avoid a single point of failure but they are not without their issues. For example, if a network link is faster than others then its messages can overtake others, possibly arriving out of order and leading to causality issues. Analogously, circular and star topologies enforce a degree of parallelism which introduces parallelism pitfalls such as race conditions which can introduce causality issues.

## Leaderless Replication

Leaderless replication systems allow any replica to directly accept writes from clients. Leaderless replication is used in Amazon Dynamo, Riak, Cassandra, and Voldemort. In some systems the client sends writes to several replicas while in others a coordinator node does this for the client but it does not enforce a write order.

For any one write the client performs writes to a certain set of nodes and considers the write successful if a number of nodes confirm success. This is resilient to nodes being unavailable as long as there is a sufficient number of nodes still available that confirm write success. To mitigate stale reads from a newly-available node, users perform reads from multiple nodes in parallel and use version numbers are used to determine the newest value.

Newly-available nodes achieve eventual consistency by two means in Dynamoc-style datastores.

* **Read repair**: When a client detects a stale value during a multi-node read, it writes the new value to the stale node. This mainly works for values that are frequently read.
* **Anti-entropy process**: Background processes look for differences between replicas and copy missing data. This is unlike a replication log in leader-based replication because writes are not replicated in any order, only the current latest values, and this process may be delayed and periodic rather than streaming and real-time.

    Durability is decreased without this process because values that are rarely read may be missing entirely from replicas that were unavailable while the value was written.

Leaderless replication is an option when requiring high availability and low latency, while tolerating occasional stale reads.

Leaderless replication is an option for multi-datacenter configurations, one taken by systems like Cassandra and Voldemort. The number of replicas `$n$` consists of nodes across all datacenters. Writes are sent to all replicas in this set, but only those in the local datacenter are waited on. Riak instead keeps client-database communication local to one datacenter and then performs asynchronous cross-datacenter replication in the background.

### Quorums

_Quorom reads and writes_ (strict quorums) are ones that satisfy the **Quorum Condition** and can therefore guarantee reading an up-to-date value if there are `$n$` replicas and every write is confirmed by `$w$` nodes and reads query at least `$r$` nodes, and the sum of reads and writes is greater than the number of nodes:

<div>$$ w + r > n $$</div>

This is because the set of nodes written to and read from must overlap, so at least one node will have the latest value.

Quorums are assembled and reached for a particular value, that is, a given value has `$n$` "home" nodes. In a (typical) cluster with many more than `$n$` nodes, an outage can prevent a client from connecting to enough of the necessary nodes to assemble a quorum.

Note that quorums don't have to be majorities.

A common configuration is to pick an odd number for `$n$` and set:

<div>$$ w = r = \frac {(n + 1)} {2} $$</div>

`$n$` does not represent the total number of nodes in the cluster, but rather the fact that the value is only stored on `$n$` nodes, enabling support for datasets larger than can fit in one node.

The quorum condition `$w + r > n$` enables varying levels of fault tolerance:

* `$w < n$` can still process writes if a node is unavailable (e.g. `$n = 4$`, `$w = 3$` and one node goes down, we can still satisfy `$w$`).
* `$r < n$` can still process reads if a node is unavailable
* `$r > \frac n 2$` and `$w > \frac n 2$` (majorities) can tolerate up to `$\frac n 2$` node failures

If fewer than the required `$w$` or `$r$` nodes are available then reads or writes fail.

The Quorum Condition can be avoided, `$w + r \le n$`, for lower latency and higher availability but greater likelihood of stale values. There is lower latency because fewer nodes are waited on and higher availability because fewer nodes are required to write to and read from.

Stale values can still be read despite the Quorum Condition in certain edge cases:

* In Sloppy Quorums writes may end up on different nodes than the `$r$` reads so there is no longer guaranteed overlap between `$r$` and `$w$` nodes
* It is not clear which write happened first between concurrent writes. The only safe route is to merge the concurrent writes
* Writes concurrent with reads may be reflected on only some of the replicas, so it is undetermined whether the read gets the new value
* Writes that are partially successful on fewer than the `$w$` replicas are not rolled back, so subsequent reads may return that write's value
* The number of replicas storing a new value may fall below `$w$` if it fails and its data is restored from a replica with an old value, breaking the Quorum Condition
* Edge cases with timing

When a quorum cannot be reached for a given value due to an outage, either errors can be returned or writes can be accepted to be performed partially on some "overflow" nodes that aren't among the `$n$` "home" nodes, known as a _sloppy quorum_.

In a _sloppy quorum_, `$r$` and `$w$` nodes may be comprise of nodes outside of the `$n$` "home" nodes for a given value in the event of an outage.

When the outage is resolved, _hinted handoff_ ensures that any writes committed to "overflow" nodes (that aren't part of the "home" nodes) are sent to the appropriate "home" nodes.

Sloppy quorums increase _write_ availability but decrease the likelihood of reading the latest value since it may have been written to a node outside of the `$n$` "home" nodes.

Because of this, a sloppy quorum isn't really a quorum but rather an assurance that the data is stored on `$w$` nodes _somewhere_ without guaranteeing that a read of `$r$` nodes will reflect the write until hinted handoff has run.

### Detecting Concurrent Writes

In Dynamo-style databases concurrent writes can also happen through read repair or hinted handoff.

Indiscriminately updating the value on each write would lead to permanent inconsistency. For eventual consistency, the replicas must eventually converge toward the same value.

_Last write wins_ (LWW) resolves conflicts through an attached timestamp and is the only conflict resolution method support by Cassandra. Durability suffers because several concurrent writes may report as successful to the client even though only one of them actually survives LWW conflict resolution. This may be acceptable in certain situations like caching. LWW is only safe with a write-once, immutable-thereafter access pattern. Cassandra recommends using a UUID as the key to accomplish this "write-once" aspect.

A write B is _casually dependent_ on write A if its operation builds on A's operation.

Two writes are no causally dependent if separate clients start the operation without knowing that another client is also performing an operation on the same key.

An operation A _happens before_ B if B:

* knows about A
* depends on A
* builds on A in some way

Two operations are _concurrent_ if neither happens before the other (neither knows about the other).

Given two operations A and B, either:

* A happened before B
* B happened before A
* A and B are concurrent

Only concurrent operations require conflict resolution.

The server can keep a version number for every key and increment it whenever the key is written to, storing both the new version number and the new value.

When a client reads a key the server returns all non-overwritten values as well as the latest version number.

A key must be read before writing, and the write must then include the version number from that prior read in addition to merging the values it returned. The server can then confidently overwrite all values with that version number or below since they will have been merged by the client, but it must retain all values with a higher version number since they will have been from concurrent writes.

The inclusion of the version number from the prior read determines the previous state that the write is based on. A write without a version number will not overwrite anything but will be given a version number and included in future reads.

Concurrent values to be merged are referred to as _siblings_ by Riak. The merge operation can be for example a union in the case of a list.

Simple merging (such as a union operation) poses a problem with deletion, where a union of siblings may reintroduce a deleted value if one sibling deleted it but the other still has it. This can be mitigated with a special _tombstone_ deletion marker with a corresponding version number. CRDTs are data structures that have built-in merging behavior.

### Version Vectors

Per-replica version numbers can be used when there are multiple replicas that accept writes. Each replica keeps track of the version numbers it has seen from other replicas. The collection of version numbers from all replicas is the _version vector_ and it indicates which values to overwrite and which to keep as siblings. When processing a write, it increments its own per-replica version number.

Version vectors are sent to client during reads and to database during writes (Riak calls the write direction a _causal context_), which enables the database to distinguish from overwrites and concurrent writes. Version vectors make it possible to read from one replica and write to another. Even if siblings are created, no data is lost as long as they're merged correctly.

Note that version vectors are different from version clocks.

# Partitioning

Partitioning (aka _sharding_) entails breaking datasets into smaller pieces (partitions or shards). They're known as:

* _shards_ in MongoDB, Elasticsearch, and SolrCloud
* _regions_ in HBase
* _tablets_ in BigTable
* _vnode_ in Cassandra and Riak
* _vBucket_ in Couchbase

Usually partitions are chosen so that each record belongs to exactly one.

Partitioning aids scalability because different partitions can be put on different nodes, distributing its disk use and query load across many processors.

Multiple partitions may be assigned to the same node, and in a leader-follower replication model the node may be the leader for some partitions and follower for other partitions.

Partitions are _skewed_ when some partitions have more data or queries than others. A partition with a particularly high load is a _hot spot_.

## Key Range Partitioning

Key range partitioning sorts the keys and partitions ranges of keys. It has the benefit that range queries are efficient since records in a range are next to each other.

The partitions may not be balanced due to the distribution of the data, so the partition boundaries need to adapt to the data in order to remain balanced.

If a partition grows too large it can be split in two.

## Hash Partitioning

One common way to mitigate hot spots is to use a hash function to partition keys. Partitions can be assigned a range of hashes (or of prefixes of them, like the first two bytes), where the boundaries can be evenly spaced or chosen pseudorandomly, known as _consistent hashing_. Consistent hashing is "consistent" because it operates independently of the number of servers or objects by assigning them a position in a _hash ring_. It's rarely used by databases because it doesn't work very well in practice.

Hash partitioning loses the ability to perform efficient range queries. One way to mitigate this is to use a _compound primary key_, so that only the first column of the key is hashed to determine the partition, but the other columns are used as a concatenated index for sorting data in the underlying sorted string tables (SSTables). While a range query cannot be done over the first column, it can be done over the rest of the columns. This works well if the first column is fixed, consisting of something like an identifier. This maps well to one-to-many relationships, for example given the key (user_id, created_at), all of a user's posts within a certain time range can be retrieved efficiently from a single partition.

Some outliers can result in hot spots despite hash partitioning, such as celebrity accounts on social media websites.

## Secondary Indexes

Secondary indexes make it much more complicated to partition data.

### Document-Partitioned Secondary Indexes

In document-based partitioning, using a _local index_, each partition is completely separate and maintains its own secondary indexes covering only the documents it contains. However, it's likely that not all documents satisfying the secondary index criteria are on a single partition (e.g. all _red_ cars), so the query needs to be sent to _all_ partitions and the results combined. This is known as scatter/gather and it is susceptible to tail latency amplification.

### Term-Partitioned Secondary Indexes

In term-based partitioning, a _global index_ is constructed and it is also partitioned, e.g. partition 0 contains the global index partition that points to cars with colors that start with the letters A through R. This is term-based partitioning because the term we're looking for determines the partition of the index, e.g. we are looking for "red" cars so we look in partition 0. Reads are more efficient than with document-based partitioning because index reads only have to be sent to the partition with that index partition instead of to all partitions. Writes are less efficient than with document-based partitioning because a write to a single document may now affect multiple partitions of the index for every changed term. Furthermore, in practice updates to global secondary indexes are asynchronous.

## Partition Rebalancing

Partition rebalancing is the process of moving load from one node in a cluster to another. Generally the database should continue to accept reads and writes during rebalancing.

Fully-automated partition rebalancing can be dangerous with automatic failure detection, such as if an overloaded and slow node is deemed dead or unhealthy, so the automated partition rebalancing decides to rebalance the partition to distribute load, but doing so puts even more load on the already-overloaded node as well as on other nodes on which the partition is rebalanced, and overall may have a detrimental impact on the network. This can lead to cascading failure.

### Modular Hashing

The eventual need for partition rebalancing is why modular arithmetic, such as in `hash(key) % node_count`, is a bad idea for hash partitioning, because if the number of nodes changes then most keys will need to be moved from one node to another because the result will be different for the same key.

### Fixed Partitioning

One rebalancing-friendly scheme is to create many more partitions than there are nodes and assign several partitions to each node. When a node is added to the cluster, it can steal a few partitions from every existing node such that each node has about the same number of partitions. The existing partitioning scheme continues to be used for reads and writes while the partitions are transferred to the new node, and only then switched, allowing for seamless online rebalancing.

This partitioning scheme also enables scaling partitions with the underlying node capacity, giving more partitions to more powerful nodes.

In fixed partitioning, the number of partitions is usually fixed at database setup time, so a future-proof high number should be chosen, but partition management also has overhead. Coming up with an ideal number of partitions can be difficult if the dataset size varies.

### Dynamic Partitioning

Fixed partitioning may not be the best approach for key range-partitioned databases (that are key range-partitioned perhaps to optimize for range scans) because of the likelihood for hot spots and empty partitions. For this reason some key range-partitioned databases like HBase create partitions dynamically: when a partition exceeds some configured maximum size it is split in half, and if it shrinks below some threshold it can be merged with an adjacent partition. Like with fixed partitioning, a node can have multiple partitions. If a partition is split, one of the halves can go to a node with capacity.

To prevent the rest of the cluster from sitting idle while the dataset size is such that it fits in a single partition, databases like HBase allow configuring an initial set of partitions, known as _pre-splitting_, which requires knowing what the key distribution will look like.

Dynamic partitioning can also be useful with hash-partitioned data.

### Node-Proportional Partitioning

Another strategy is to have a fixed number of partitions per node. When a new node is added, it randomly chooses that same fixed number of partitions to split in half and takes ownership of one half. This can lead to unfair splits, but on average over a large number of partitions the new node takes a fair share.

## Request Routing

_Service discovery_ concerns the problem of knowing where different services are available.

Apache ZooKeeper is one way that distributed data systems can keep track of this cluster metadata. A node registers itself with ZooKeeper and interested actors can subscribe to the information to get notifications for example when a partition changes ownership or a node is added or removed. This way a request router can know which node to route a partition to.

Other systems use a gossip protocol on the nodes so that they spread any changes in cluster state. This way, any node can receive a request and it forwards it to the correct node.

DNS is sufficient to determine the actual IP address of a node since IP addresses aren't likely to change as often as partitions.

# Transactions

Transactions can group operations into a single unit that either completely succeeds or completely fails, removing the need to handle partial failure.

The acronym ACID stands for Atomicity, Consistency, Isolation, and Durability. In practice, its meaning has become diluted into a vague marketing term.

## ACID

### Atomicity

Atomicity refers to the "all or nothing" nature of a transaction: either it succeeds (commits) entirely or fails (aborts) completely. There is no partial failure.

Atomicity does not pertain to concurrency, unlike an "atomic operation" in multi-threaded programs, where another thread can only see the state before or after the atomic operation.

### Consistency

Consistency refers to the database being in a valid state for some definition of "valid" that is application-specific.

Consistency is up to the application to ensure. The database may be facilitate ensuring some invariants such as uniqueness constraints, but maintaining consistency is generally the application's responsibility. For this reason, some believe "Consistency" doesn't really belong in ACID. Apparently, the "C" was thrown in to make the acronym work.

### Isolation

Isolation refers to the property that concurrently executing transactions are isolated from each other: they cannot step on each other's toes or view partial modifications.

Serializability means that each transaction can pretend that it is the only running transaction, the database ensuring that the result is the same as if they had run serially. In practice, this carries a heavy performance penalty so it is rarely used or not even implemented.

### Durability

Durability is the promise that data will not be lost once it has been considered to have been written, even in the event of a hardware fault or a database crash.

This is conceptually accomplished by only acknowledging a write as being successful after the database can guarantee that it has indeed been successful.

In a single-node database this may take the form of writing to an append-only, write-ahead log.

In a replicated database, this may take the form of ensuring that the write has been replicated to some number of nodes before confirming the write as successful.

## Isolation Levels

Concurrency issues are relevant when a read by one transaction happens concurrently with a modification by another.

_Serializable isolation_ is the strongest isolation level. It guarantees that transactions have the same effect as if they ran serially. This has a performance cost, so weaker levels of isolation are more used, ones that protect against only some concurrency issues.

Note that the SQL standard is ambiguous and imprecise about isolation levels.

### Read Committed

The _Read committed_ isolation level guarantees:

1. Will only read data that has already been committed (i.e. no _dirty reads_)
2. Will only overwrite data that has already been committed (i.e. no _dirty writes_)

It is a very popular isolation level set as the default in Oracle, PostgreSQL, SQL Server, MemSQL, and many others.

A _dirty read_ is a read where a transaction reads data that has been written by a transaction that has not yet committed or aborted. This is wrong because the value may yet be rolled back if the transaction is aborted, causing the read to have been invalid.

<div class="mermaid">
sequenceDiagram
    autonumber
    participant TX 1
    participant Table
    participant TX 2
    TX 1->>Table: Write = "1"
    Table->>TX 2: Read = "1"
    Table-->>TX 1: Abort
    Note right of Table: "1" is now invalid
</div>

Dirty reads are not usually prevented with row-level locks because one long-running transaction would end up blocking many other reads until it completed. Instead, most databases prevent dirty reads by remembering the original value as well as the newly-written value. Concurrent transactions that read the value are simply served the old value. Once the transaction that wrote the new value commits, transactions begin reading that new value.

A _dirty write_ is one where a transaction overwrites a value overwritten by an earlier transaction that had not yet committed. This is a problem when updating multiple objects because it can result in 'partial updates' of those multiple objects.

For example two transactions that each must write to tables A and B together for a particular update, but one of the transactions performs the writes in between the two writes of the other transaction:

<div class="mermaid">
sequenceDiagram
    autonumber
    participant TX 1
    participant A
    participant B
    participant TX 2
    TX 1->>A: Write "1A"
    TX 2->>A: Write "2A"
    TX 2->>B: Write "2B"
    TX 1->>B: Write "1B"
    Note over A, B: A = "2A", B = "1B"<br>Should be either 1A, 1B or 2A, 2B
</div>

Now tables A and B have writes from each transaction even though the intent was to write to both for any one update: table A has TX 2's write and table B has TX 1's write.

In the example above, the values ultimately written should instead be the result of any given _single_ transaction: either 1A and 1B, or 2A and 2B.

In the Read Committed isolation level, dirty writes are usually avoided by delaying later writes until earlier transactions have committed.

Dirty writes are usually prevented with row-level locks which a transaction acquires in order to write to it, and it holds the lock until the transaction is committed or aborted. Since only one transaction can hold a lock for a given transaction, all others attempting to acquire it must wait.

### Snapshot Isolation

A _nonrepeatable read_ (or _read skew_) is a temporary inconsistency (i.e. nonrepeatable) that occurs when a single transaction reads different values for a given object because it sees the changes applied by a separate transaction that committed during the original transaction's lifetime.

For example, transaction 1 reads a row, then it is updated and committed by transaction 2, and transaction 1 sees that updated value if it reads it again, thus reading different values for the same object during the same transaction.

<div class="mermaid">
sequenceDiagram
    autonumber
    participant TX 1
    participant Table
    participant TX 2
    Table->>TX 1: Read = "1"
    TX 2->>Table: Write = "2"
    Table->>TX 1: Read = "2"
    Note right of TX 1: "2" ≠ "1"
</div>

Read skew is problematic for example if performing a long-running database backup while the database is running, so the backup could end up with some parts being old and others being new, then restoring from this backup will make these inconsistencies permanent.

The _Snapshot Isolation_ level (aka _repeatable read_ in PostgreSQL) solves this problem by allowing each transaction to read from a _consistent snapshot_ of the database: all of the data that was committed into the database by the start of the transaction, so that the transaction continues to see that snapshot even if changes are subsequently committed by other transactions.

Snapshot isolation is usually implemented with multi-version concurrency control (MVCC), where the database keeps several different committed versions of an object to enable many in-progress transactions.

In fact, storage engines usually implement Read Committed Isolation with MVCC as well, using a separate snapshot for each query, whereas Snapshot Isolation would use the same snapshot for the entire transaction.

In PostgreSQL, a transaction is given a monotonically increasing unique transaction ID (`txid`), and writes are tagged with the transaction ID of the writer. Rows have `created_by` and `deleted_by` fields containing the transaction ID responsible for the respective action. Rows aren't outright deleted, but instead garbage collected when no ongoing transaction could possibly read the data. Updates are internally translated into a pair of delete and create actions.

When a transaction reads from a database, the transaction ID is used to determine which objects are visible to it (i.e. the snapshot).

1. Writes by ongoing transactions since the start of the transaction are ignored even if they commit.
2. Writes by aborted transactions are ignored.
3. Writes by later transactions are ignored.
4. All other writes are visible.

In other words an object is visible if:

1. The transaction that wrote the object had already committed by the time the reading transaction began
2. The object is not marked for deletion, or the deleting transaction did not commit before the reading transaction began

Indexes can be made to work in a multi-version database with a variety of different approaches:

* The index points to all versions of the indexed object, and an index query filters out object versions that are not visible to the transaction. Garbage collection removes old index entries when the values are deleted.
* Using immutable, persistent data structures like append-only/copy-on-write B-Trees, where every write transaction creates a new B-Tree root, with each such B-Tree root representing a consistent snapshot. Since the tree is immutable, subsequent writes cannot alter an existing tree. This requires compaction and garbage collection.

The imprecision and ambiguity of the SQL standard with respect to isolation levels means that nobody really knows what repeatable reads means.

## Lost Updates

A lost update can occur from two concurrent read-modify-write cycles, if the second write doesn't include the first modification. In this case, the second write _clobbered_ the first write.

Imagine a counter value that is meant to be incremented by transactions.

<div class="mermaid">
sequenceDiagram
    autonumber
    participant TX 1
    participant Table
    participant TX 2
    Table->>TX 1: Read = "1"
    Table->>TX 2: Read = "1"
    TX 1->>Table: Write 1 + 1 = "2"
    TX 2->>Table: Write 1 + 1 = "2"
    Note over Table: Should be 3
</div>

Lost updates can happen in scenarios such as updating a counter, updating a complex structure such as a JSON document, or two users editing a wiki page at the same time with each overwriting the entire page contents.

### Atomic Writes

Atomic updates remove the need to implement read-modify-write cycles in applications. In SQL they take the form of `UPDATE … SET x = y`.

Atomic operations are usually implemented with an exclusive lock on the object during the operation. This method is known as _cursor stability_. Another implementation option is to preform all atomic operations on a single thread.

ORMs sometimes produce read-modify-write cycle code rather than atomic updates.

### Explicit Locking

An alternative to built-in atomic operations is to explicitly acquire a lock on the affected data, such as by using `SELECT … FOR UPDATE`.

### Detecting Lost Updates

Instead of preemptively trying to avoid lost updates, the database can detect them when they occur and abort the offending (clobbering) transaction to force a retry. This can be done efficiently with snapshot isolation.

The advantage of detecting lost updates is that it eliminates the cognitive load and responsibility of the developer, who may forget to acquire certain locks or to perform certain atomic operations.

Some argue that a database must prevent lost updates in order to qualify as providing snapshot isolation. PostgreSQL's repeatable read and other databases do this, but not MySQL/InnoDB's repeatable read.

### Compare-and-Set

Compare-and-set atomic operations are ones that perform the set only if the value has not changed since it was read, otherwise it has no effect and must be retried.

In SQL, this _may_ be done by comparing that the value remains the same, but whether or not it's correct depends on the database. For example, if the database allows the `WHERE` clause to read from the old snapshot, then this may always be true and so may not be preventing lost updates.

``` sql
UPDATE pages
SET content = 'edited'
WHERE id = 123 AND content = 'old';
```

### Conflict Resolution and Replication

The problem of lost updates is also relevant in replicated databases, since updates may occur concurrently on different nodes, so any conflicts need to be resolved.

A common workaround is to allow concurrent writes to create conflicting versions of a value (aka _siblings_) which are then resolved either in the application or by special database data structures.

_Last write wins_ (LWW) is the default conflict resolution strategy in many replicated databases.

## Write Skew and Phantoms

_Write skew_ occurs when two transactions read the same objects but then update some of those objects, with each one possibly modifying a separate object.

This can be seen as a generalization of "lost updates", where the specific case where the transactions modify the same objects may result in dirty writes or lost updates.

Imagine a table that keeps track of which doctors are "on call" with each record corresponding to a specific doctor with a column for whether or not they're on call.

A constraint can be that there must be at least one doctor on call at any moment. Each doctor can start a transaction to first confirm the constraint—that there is more than one doctor currently on call—before proceeding to mark themselves as "not on call".

<div class="mermaid">
sequenceDiagram
    autonumber
    participant TX 1
    participant Table
    participant TX 2
    Table->>TX 1: Num. Dr's on call = "2"
    Table->>TX 2: Num. Dr's on call = "2"
    TX 1->>Table: Set self not on call
    TX 2->>Table: Set self not on call
    Note over Table: Nobody on call!
</div>

Another case where write skew can occur is when trying to prevent double-spending: separate transactions, unaware of each other, check to make sure that the user can afford some purchase and determine the purchase valid in isolation, but the combination of purchases made by the separate transactions exceeds the available funds.

This is somewhat similar to [lost updates](#lost-updates) except that the updates are affecting separate rows, therefore it is not so clearly a conflict as it is a race condition: if the transactions had occurred serially, this would not have happened. Write skew can be seen as a generalization of lost updates.

The general pattern is:

1. Read to ensure a requirement is satisfied (e.g. at least 2 doctors on call, no existing bookings)
2. A write (insert, update, or delete) is possibly made and committed _based on_ the result. This changes the result of the precondition in #1

Automatically preventing write skew requires true serializable isolation. Otherwise, explicitly locking the rows that a transaction depends on can help. However, rows need to exist in order for them to be locked.

If instead read has to check for the absence of rows, and the subsequent write adds a row that matches that criteria, then there is the potential for _phantoms_.

A _phantom_ is when a write in one transaction changes the result of a search query in another, which then alters that second transaction's behavior in such a way that would not have happened had they occurred serially.

Snapshot isolation prevents phantoms in read-only queries, but read-write transactions may lead to write skew.

A problem with mitigating phantoms is that there is nothing to acquire a lock on because we are expecting the absence of some record in the first place. One workaround to this problem is to flip the situation by _materializing conflicts_, in other words, having physical records that represent the "absence", for example records for "empty slots" in a booking system.

