+++
title = "Spark"
date = 2019-04-18

[note]
kind = "technology"
+++

Spark is a tool for managing and coordinating the execution of tasks on large data across a cluster of computers. It is written in [Scala](/notes/scala).

Spark libraries exist for other languages, such as Python. Spark translates such code into code that can run on executor JVMs.

<nav id="toc"></nav>

# Concepts

The _cluster_ that Spark tasks execute on is managed by a _cluster manager_ such as Spark's, YARN, or Mesos. These cluster managers provision resources to applications for them to do their work.

The _driver_ process runs the `main()` function on a node in the cluster. It maintains information about the Spark Application, responds to the user's program, and analyzes, distributes, and schedules the work across executors.

An _executor_ process actually executes the code assigned to it by the driver and reports the state of its computation back to the driver node. The number of executors per node can be configured.

Spark also has a _local mode_ where the driver and executors are simply processes on the same machine.

The `SparkSession` object is the entry point to running Spark code. Standalone applications must create the `SparkSession` object themselves, while it's otherwise created implicitly in interactive mode consoles.

A _distributed collection_ is one that is partitioned across the various executors. The core Spark abstractions are DataSets, `DataFrame`s, SQL Tables, and Resilient Distributed Datasets (RDDs). All of these are distributed collections.

A `DataFrame` represents a table of data with rows and columns. A `DataFrame`'s _schema_ is a list defining the columns and their types. A parallelism of one is had when either there is one partition and many executors, or many partitions and one executor. `DataFrame`s are high-level and can't manipulate partitions manually, but there are lower-level APIs that can do this, such as RDDs.

Spark's core data structures are immutable and are manipulated through specifying _transformations_ to be applied to them. Spark doesn't act on transformations until an action is called.

There are transformations with both _narrow_ and _wide_ dependencies.

Transformations with narrow dependencies, _narrow transformations_, contribute a single output partition for every input partition. Spark automatically performs _pipelining_ with narrow transformations, so that multiple operations are performed in-memory.

Transformations with wide dependencies, _wide transformations_, contribute many output partitions for every input partition. This is often referred to as a _shuffle_, where Spark exchanges partitions across the cluster. Shuffles cause Spark to write results to disk.

# Spark SQL

There is a Spark SQL console via `spark-sql`.