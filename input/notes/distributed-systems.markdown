---
title = "Distributed Systems"
published = "March 7, 2018"
comments = false
---

Notes about distributed systems and system scalability from resources such as the amazingly wonderful book _Designing Data-Intensive Applications_ by Martin Kleppmann.

<toc />

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

Higher percentiles are even more important in backend services that are called multiple times as part of serving a single end-user request, since, even if they're called in parallel, the end-user has to wait for the slowest of the calls to complete. Even if only a small percentage of backend calls are slow, the chance of getting a slow call increases with an increase in backend calls, so a higher proportion of end-user requests end up being slow.

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
* definite operations processes
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

Another limitation of the document model is when the application data uses many-to-many relationships. Joins may be reduced by denormalized but that increases the complexity of keeping the denormalized data consistent. Joins can be emulated in application code through multiple requests but that increases application complexity and is often slower than a join performed by the database. Overall these issues can lead to more complexity and decreased performance.

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

Document databases are sometimes considered _schemaless_ but the truth is that although it's not an explicit schema enforced by the database upfront, there _is_ an implicit schema assumed by readers of the data. A more accurate term may therefore be _schema-on-read_, whereas _schema-on-write_ might refer to an explicit, upfront schema that is used to ensure that writes conform to it. Schema-on-read is similar to dynamic type checking performed at run-time whereas schema-on-write is similar to static type checking performed at compile-time.

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

An index is a separate structure derived from the primary data, to maintaining it has an overhead, such as on writes, since the index has to be updated as well. Appropriate indexes speed up read queries but slow down writes, which is why databases don't index everything by default.

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

Careful concurrency control is required to coordinate the in-plage modification of pages from multiple threads, otherwise a certain thread may see the tree in an inconsistent state. This is often accomplished with _latches_ (light-weight locks).

Pages can save space by abbreviating keys, storing just enough information to act as boundaries between key ranges, allowing a tree to have a higher branch factor and thus fewer levels. This optimization was originally part of a B+ Tree, but the optimization is now very commonplace.

Many B-Tree implementations try to lay out a tree so that leaf pages appear in sequential order in order to optimize scans over a large part of a key range in sorted order. This layout is difficult to maintain as the tree grows.

Many B-Tree implementations add pointers to leaf pages referencing sibling pages to optimize scanning keys in order without having to traverse back through parent pages.

Fractal Trees are B-Tree variants which borrow log-structured ideas to reduce disk seeks.

## LSM-Trees vs B-Trees

<!-- Advantages -->

LSM-Trees are usually faster for writes and B-Trees are thought to be faster for reads. LSM-Tree reads are often slower due to having to check various different structures and SSTables at different stages of compaction. However, specific workloads should be tested.

A B-Tree index has to write every piece of data to both the write-ahead log and the tree page itself, and perhaps further if the pages are split, as well as the overhead of having to write entire pages at a time even if few bytes have changed. In fact, certain storage engines write the page twice to avoid partially updates pages in case of power loss. However, LSM-Trees also rewrite the same data multiple times due to compaction and merging of SSTables.

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

Column-oriented storage lends itself to compression. _Bitmap encoding_ can be used to compress the column values when the number of distinct values is small compared to the number of rows. A column with $n$ distinct values can be turned into $n$ separate bitmaps, one for each distinct value, with one bit for each row, so that the bit for that row is 1 if the row has that value and 0 if not. The bitmaps in total will be much smaller than storing the entire list of column values.

If $n$ is very small then the bitmaps can be stored with one bit per row, otherwise they will be sparse, with a lot of zeros in most of the bitmaps. In this case, they can be further compressed with run-length encoding.

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
