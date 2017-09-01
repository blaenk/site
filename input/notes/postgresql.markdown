---
title = "PostgreSQL"
published = "August 7, 2017"
excerpt = "Arguably the premiere open source RDBMS"
comments = false
---

<toc />

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

Comments are denoted by two dashes `--`. Multi-line comments can be written similar to C-style multi-line comments. Unlike C, multi-line comments can be nested.

SQL is case insensitive except when identifiers are double-quoted to preserve case, which are known as _quoted identifiers_ or _delimited identifiers_. Quoted identifiers can be used to explicitly force something to be an identifier regardless of whether or not it is also a keyword. Furthermore, they can contain any character (except the null character). A double quote can be included by writing two successive double quotes `""`.

A Unicode variant of quoted identifiers is prefixed by `U&`. Unicode characters can be included in escaped form with a backslash and four hex digits for the code point, or backslash and plus sign and six hex digits.

``` postgresql
-- Equivalent to "data"
U&"d\0061t\+000061"
```

A `UESCAPE` clause can be written after the string in order to specify an alternate escape character.

As noted earlier, unquoted names are always folded to lower case. This is an incompatibility with the SQL standard, which does the opposite (fold to upper case), so best practice is to either always or never quote a name.

Identifiers and keywords must begin with a letter or underscore, and then can consist of letters, underscores, digits, and dollar signs [^deviation].

[^deviation]: This is a PostgreSQL-specific feature, not mentioned in the SQL standard.

A string constant is delimited by single quotes `'`. A single quote can be included by escaping it by using two single quotes: `''`.

``` postgresql
'This string''s single quote'
```

Strings separated only by whitespace with at least one newline are treated as a single running string.

``` postgresql
SELECT 'foo'
'bar';

-- Equivalent to:
SELECT 'foobar';

-- Invalid:
SELECT 'foo'    'bar';
```

Escape string constants are a PostgreSQL extension to the SQL standard which are prefixed by the letter `E` and allow for C-style escape sequences:

``` postgresql
E'One line.\nTwo lines.'
```

PostgreSQL also has dollar-quoted string constants, which are delimited by: a dollar sign `$`, a tag of zero or more characters, another dollar sign `$`, then the string content, then the end delimiter. No characters inside these strings are ever escaped [^raw_string]. Furthermore, further dollar-quoted strings can be nested by using different tags at each level, which makes them useful for writing function definitions. Note that tags are case sensitive.

[^raw_string]: These are comparable to raw strings in other languages.

``` postgresql
$$This is a dollar-quoted-string$$
$MyTag$This is another string.$MyTag$


$function$
BEGIN
  RETURN ($1 - $q$[\t\r\n\v\\]$q$);
END;
$function$
```

Bit-string constants are string constants prefixed by the letter `B` and are comprised of the binary digits `0` and `1`. They can also be specified in hexadecimal notation by using the `X` prefix instead.

Numeric constants without a decimal or exponent are initially assumed to be of type 32-bit `integer` if the value would fit, or 64-bit `bigint`, or `numeric`.

Numeric constants _with_ a decimal or exponent are _always_ initially presumed to be of type `numeric`.

Constants can be forced to be interpreted as a specific data type via casting:

``` postgresql
REAL '1.23' -- string style

1.23::REAL  -- PostgreSQL style
```

Constants of arbitrary type can be specified by writing a string literal and explicitly specifying the type. The string literal is then passed to that type's conversion routine.

``` postgresql
typename 'string literal'

'string literal'::typename

CAST ( 'string literal' AS typename )
```

Note that the `type 'string literal'` syntax can't be used for array types.

Some types can also be constructed using function-like type coercion:

``` postgresql
typename ( 'string' );
```

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

A window function applies an aggregate-like function over a portion of rows selected by a query. Unlike aggregate functions, the input rows are not reduced to a single row—each row remains separate in the output.

A window function is syntactically different from a regular or aggregate function by the presence of the `OVER` clause directly after the call. The clause specifies how the rows are split up for processing.

Window functions are only permitted in the `SELECT` list and the `ORDER BY` clause of a query.

The `PARTITION BY` sub-clause partitions (groups) the rows sharing the same values for the provided expression. These partitions are processed separately by the window function. The `PARTITION BY` clause is similar to `GROUP BY` except its values are always expressions, not output-column names or numbers The `ORDER BY` sub-clause can be used to order each resulting partition. Then, for each row, the window function is computed across the rows that are in the same partition.

If the `PARTITION BY` clause is omitted then there will only be one resulting partition containing all of the rows.

For each row, there is a set of rows within its partition called its _window frame_. Many window functions act only on the rows of the window frame rather than the whole partition. By default, if the `ORDER BY` clause is provided, then the window frame consists of all rows from the start of the partition (as defined by the order) up through the current row and including any rows considered equal to the current row as defined by the order.

The window frame can be specified using `RANGE` mode or `ROWS` mode.

``` postgresql
{ RANGE | ROWS } frame_start
{ RANGE | ROWS } BETWEEN frame_start AND frame_end


-- frame_start and frame_end can be
UNBOUNDED PRECEDING
CURRENT ROW
UNBOUNDED FOLLOWING

-- ROWS mode
some_value PRECEDING
some_value FOLLOWING
```

`frame_end` defaults to `CURRENT ROW` if omitted.

`UNBOUNDED PRECEDING` means the frame starts with the first row in the partition, and vice versa for `UNBOUNDED FOLLOWING` and the frame end.

A _peer row_ is a row that `ORDER BY` considers to be equivalent to the current row.

If the `ORDER BY` clause is omitted then the window frame consists of all rows in the partition, since all rows are considered peers of the row.

In `RANGE` mode, a `frame_start` of `CURRENT ROW` means that the frame starts with the first peer row in the partition, and vice versa for `frame_end`.

In `ROWS` mode, `CURRENT ROW` literally means the current row.

In `ROWS` mode, `x PRECEDING` means that the frame starts at `x` rows before, and vice versa with `x FOLLOWING`. The value `x` must be an integer, where `0` refers to the current row.

Since window functions execute _after_ aggregate functions, it's possible to include an aggregate function call as a parameter to a window function, but not vice versa.

Rows input to the window function can be filtered with a `FILTER` clause, only if the window function is an aggregate.

A sub-select can be used if it's necessary to filter or group rows after window calculations.

The asterisk `*` "argument" is used to call parameter-less aggregate functions as window functions.

``` postgresql
count(*) OVER (PARTITION BY x ORDER BY y)
```

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

# Value Expressions

A value expression is one of:

* constant or literal value
* column reference
* positional parameter reference
* subscript expression
* field selection expression
* operator invocation
* function call
* aggregate expression
* window function call
* type cast
* collation expression
* scalar subquery
* array constructor
* row constructor
* parenthesized value expression

The result of a value expression is known as a _scalar expression_ or just _expression_, to differentiate from table expressions.

A column can be optionally referenced by explicitly specifying the table in which it is contained. The table itself can be optionally qualified with a schema.

``` postgresql
correlation.columnname
```

A positional parameter reference refers to a value passed to a function or prepared query.

``` postgresql
CREATE FUNCTION dept(text) RETURNS dept
AS $$ SELECT * FROM dept WHERE name = $1 $$
LANGUAGE SQL;
```

Arrays can be indexed or sliced with subscript expressions. The subscript expression should generally be parenthesized unless it simply consists of a column reference or positional parameter.

``` postgresql
-- Indexing
array_expression[subscript]

-- Slicing
array_expression[lower_bound:upper_bound]
```

It's possible to select a particular field from values of composite type (row type) the same way it is done for columns on tables. The row expression should generally be parenthesized unless it's a table reference or positional reference.

``` postgresql
some_table.fieldname
$1.somecolumn

(row_function(a, b)).some_column

-- Parenthesize to interpret `composite_column` as column not table.
(composite_column).some_field

-- Parenthesize to interpret `some_table` as table not schema.
(some_table.composite_column).some_field
```

All fields can be selected by using the asterisk `*` in place of a field name.

``` postgresql
(composite_column).*
```

Functions that take a single argument of composite type can also be called with field selection syntax, and vice versa: field selection can be written in functional style [^ufcs]. This allows the emulation of _computed fields_. Note that this is a PostgreSQL feature.

[^ufcs]: This reminds me of Unified Function Calling Syntax (UFCS) which other languages support, such as the [D language].

[D language]: https://tour.dlang.org/tour/en/gems/uniform-function-call-syntax-ufcs

``` postgresql
some_column(some_table)

-- Equivalent to
some_table.some_column
```

Aggregate expressions represent the application of an aggregate function across rows selected by a query.

Aggregate expressions can be given the `DISTINCT` qualifier to specify that the aggregate should be invoked once for each _distinct_ value of the expression, or set of values, found in the input rows [^slow_count_distinct].

[^slow_count_distinct]: Note that it seems that this may actually be a slow construct because it first sorts the input rows. [Source](https://stackoverflow.com/questions/11250253/)

``` postgresql
count(DISTINCT *)
```

# Aggregate Functions

An aggregate function reduces multiple inputs to a single output value.

The difference between `WHERE` and `HAVING` is that `WHERE` selects the rows before groups and aggregates are computed, whereas `HAVING` selects rows _after_ that. This is why an aggregate function may only appear in the result list or the `HAVING` clause and not in a `WHERE` clause.

An aggregate expression within a subquery usually applies over the rows of the subquery, _except when_ its arguments or `FILTER` clause contain any outer-level variables, in which case the aggregate then belongs to the nearest outer level and so applies over the rows of _that_ query, so that the aggregate expression is effectively an outer reference for the subquery it appears in, and acts as a constant over any one evaluation of the subquery. In this case, the placement restriction (i.e. in the result list or `HAVING` clause) applies with respect to the query level that the aggregate belongs to.

Most aggregate functions ignore null inputs, i.e. rows in which one or more of the expression(s) yield null are discarded. Assume this to be true for all built-in functions unless otherwise specified.

``` postgresql
-- Total input rows.
count(*)

-- Total input rows where `field` is non-NULL.
count(field)

-- Total distinct rows where `field` is non-NULL.
count(DISTINCT field)
```

An `ORDER BY` clause can be provided to sort the input rows before being aggregated. This is necessary for certain functions whose output depends on the order of the input, such as `array_agg`.

``` postgresql
SELECT array_agg(field_one ORDER BY field_two DESC)
FROM table;
```

Note that the `ORDER BY` clause must go after _all_ aggregate function arguments.

``` postgresql
SELECT string_agg(field_one, ',', ORDER BY field_two DESC)
FROM table;
```

Note that placing the `ORDER BY` clause after an earlier argument is actually the act of _starting_ the `ORDER BY` clause earlier, such that subsequent arguments are actually _part of_ the `ORDER BY` clause, and thus are additional fields on which to order the input rows.

Note that if _both_ the `DISTINCT` and `ORDER BY` clauses are used, it's not possible to sort on an expression absent from the `DISTINCT` list. In fact, the ability to specify both clauses is a PostgreSQL extension.

A `FILTER` clause may be provided so that only the input rows for which the clause holds true are fed to the aggregate function.

``` postgresql
SELECT
  count(*) AS unfiltered,
  count(*) FILTER (WHERE i < 5) AS filtered
FROM
  generate_series(1, 10) AS s(i);
```

Ordered-set aggregates are aggregates which _require_ an `ORDER BY` clause. Examples include rank and percentile calculations. For these aggregates, the `ORDER BY` clause is written inside the `WITHIN GROUP` clause:

``` postgresql
some_aggregate(direct arguments, …)
  WITHIN GROUP ( ORDER BY aggregated arguments, … )
```

The `ORDER BY`'s arguments, _aggregated arguments_, are evaluated once per input row, just as regular aggregate functions' arguments would be, then sorted as per the `ORDER BY` clause, and then fed to the aggregate function [^map_sort_aggregate]. The arguments passed to the ordered-set aggregate are known as _direct arguments_ and are only evaluated once per aggregate call, _not_ once per input row as with regular aggregates. Because of this, direct arguments are typically used for values that are fixed throughout the aggregate calculation, such as percentile fractions. The direct argument list may be empty.

[^map_sort_aggregate]: If I understand correctly, it seems like the arguments to `ORDER BY` specify how to map the input rows, then those mapped results are sorted as per the `ORDER BY`, and _only then_ are those sorted mapped results fed into the ordered-set aggregate function.

``` postgresql
SELECT percentile_cont(0.5) WITHIN GROUP (ORDER BY income)
FROM households;
```

# Data Types

Type names are not keywords in the syntax, except where required by the SQL standard for special cases.

# Operators

Schema-qualified operators can be written by using the `OPERATOR` keyword. Note that the effective operator's precedence is the same regardless of the precedence of the operator passed as the argument.

``` postgresql
SELECT 3 OPERATOR(pg_catalog.+) 4;
```
