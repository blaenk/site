---
title = "PostgreSQL"
published = "August 7, 2017"
excerpt = "Arguably the premiere open source RDBMS"
comments = false
---

# Architecture

A PostgreSQL session consists of a server process `postgres` which manages the database files, accepts connections, and performs database actions. PostgreSQL handles multiple concurrent connections by forking a process for each connection.

# Databases

A database can be created using the `createdb` command and removed with the `dropdb` command. These and other PostgreSQL commands assume to run as a PostgreSQL user with the same name as the system account name, which can be overridden with the `-U` switch.

# psql

The `psql` command is an interactive interface into the database. It's run by specifying the database to operate on:

``` console
$ psql mydb
```

Internal commands are denoted by a backslash `\` prefix, such as `\h` which provides syntax help for SQL commands, `\?` which lists internal commands, or `\q` which quits the session.

The `\i` command reads in commands from a given file.

# SQL

## Syntax

Comments are denoted by two dashes `--`.

SQL is case insensitive except when identifiers are double-quoted to preserve case.

Identifiers and keywords must begin with a letter or underscore, and then can consist of letters, underscores, digits, and dollar signs [^deviation].

[^deviation]: This is a PostgreSQL-specific feature, not mentioned in the SQL standard.

## Concepts

A _relation_ is a mathematical term for table. Each table is a named collection of rows, each with a set of named columns, each with a specific data type. Columns have a fixed order within rows, but rows don't have a guaranteed order.

Tables are grouped into databases, and a collection of databases managed by a single PostgreSQL server constitutes a database cluster.

# Views

A _view_ gives a name to a query so that it can be referred to as an ordinary table. It is good practice to make liberal use of views in order to encapsulate the details of table structure, details which may change over time as an application evolves. Since views can be referred to as an ordinary table, it is possible to build views upon other views.

``` postgresql
CREATE VIEW myview AS
  SELECT city, temp_lo, temp_hi, prcp, date, location
  FROM weather, cities
  WHERE city = name;

SELECT * FROM myview;
```

# Foreign Keys

Maintaining _referential integrity_ refers to ensuring that a row that references another row, potentially from another table, remains valid despite operations made on the other row, such as the row being deleted or modified in some way.

# Transactions

Transactions bundle up multiple steps into a single atomic, all-or-nothing operation. If some failure occurs, none of the steps affect the database.

A transactional database like PostgreSQL guarantees that all operations made by a transaction are recorded before reporting that transaction as having completed.

Intermediate steps are not visible to other concurrent transactions, and once the transaction is complete, _all_ of the effects of the operations become visible simultaneously.

Transactions are surrounded with the `BEGIN` and `COMMIT` commands. A group of such commands is sometimes called a _transaction block_.

``` postgresql
BEGIN;
UPDATE accounts
  SET balance = balance - 100.00
  WHERE name = 'Alice';
-- etc etc
COMMIT;
```

The transaction can be canceled by using the `ROLLBACK` command, so that all operations done until then are canceled.

PostgreSQL implicitly wraps every SQL statement within a `BEGIN` … `COMMIT` transaction, with the `COMMIT` being run only if the statement was successful. Some client libraries also do this or something similar.

It's possible to define _savepoints_ which act as checkpoints within the transaction with the `SAVEPOINT` command. Savepoints can then be rolled back to by name with the `ROLLBACK TO` command, leaving the rest of the transaction up until that point intact.

Rolling back to a savepoint does not automatically release the resources associated with the savepoint, so as to allow rolling back to it again if necessary. A savepoint can be released explicitly if it's no longer needed.

> `ROLLBACK TO` is the only way to regain control of a transaction block that was put in an aborted state by the system due to an error, short of rolling it back completely and starting again.

# Window Functions

> A window function performs a calculation across a set of table rows that are somehow related to the current row. […] unlike regular aggregate functions, use of a window function does not cause rows to be grouped into a single output row — the rows retain their separate identities.

A window function is syntactically different from a regular or aggregate function by the presence of the `OVER` clause directly after the call. The clause specifies how the rows are split up for processing. The `PARTITION BY` sub-clause declares to partition (group) the rows sharing the same values for the provided expression. The `ORDER BY` sub-clause can be used to order each resulting partition. Then, for each row, the window function is computed across the rows that are in the same partition.

If the `PARTITION BY` clause is omitted then there will only be one resulting partition containing all of the rows.

For each row, there is a set of rows within its partition called its _window frame_. Many window functions act only on the rows of the window frame rather than the whole partition. By default, if the `ORDER BY` clause is provided, then the window frame consists of all rows from the start of the partition (as defined by the order) up through the current row and including any rows considered equal to the current row as defined by the order.

If the `ORDER BY` clause is omitted then the window frame consists of all rows in the partition.

Window functions are only permitted in the `SELECT` list and the `ORDER BY` clause of a query.

Since window functions execute _after_ aggregate functions, it's possible to include an aggregate function call as a parameter to a window function, but not vice versa.

A sub-select can be used if it's necessary to filter or group rows after window calculations.

A window can be defined in order to be used by multiple window function calls by using the `WINDOW` clause.

``` postgresql
SELECT sum(salary) OVER w, avg(salary) OVER w
  FROM empsalary
  WINDOW w AS (PARTITION BY depname ORDER BY salary DESC);
```

# Inheritance

Inheritance in PostgreSQL is similar to the concept with the same name from object oriented programming languages. In PostgreSQL, a table can inherit from zero or more tables.

# Queries

Expressions can be written in the `SELECT` output list.

The `AS` option can be provided to rename an output column.

# Aggregate Functions

An aggregate function computes a single result from multiple input rows [^reducer].

[^reducer]: This reminds me of reducing from functional languages.

The difference between `WHERE` and `HAVING` is that `WHERE` selects the rows before groups and aggregates are computed, whereas `HAVING` selects rows _after_ that. This is why the `WHERE` clause can't contain aggregate functions.

# Data Types

Type names are not keywords in the syntax, except where required by the SQL standard for special cases.
