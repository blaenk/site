+++
title = "PostgreSQL"
date = 2017-08-07

[note]
kind = "technology"
+++

[PostgreSQL](https://www.postgresql.org/) is an extremely robust and popular open source relational database management system (RDBMS). The third-party open source [PostGIS](https://postgis.net/) extension further adds geospatial capabilities.

<nav id="toc"></nav>

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

An interesting trick is to us the `\COPY sometable FROM STDIN` meta-command, then all subsequent lines are interpreted as data, terminated by the `\.` meta-command. This allows SQL and data to be mixed in the same file, and is something that `pg_dump` itself does when outputting SQL. Note that this only works when feeding commands into standard in.

# SQL

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

# Functions

Functions with named parameters may be called using either positional or named notation. Named notation permits an arbitrary argument order. _Mixed notation_ combines positional and named notation so that positional parameters are written first, with named parameters appearing after.

``` postgresql
CREATE FUNCTION concat_lower_or_upper(a text, b text, uppercase boolean DEFAULT false)
RETURNS text
AS
$$
 SELECT CASE
        WHEN $3 THEN UPPER($1 || ' ' || $2)
        ELSE LOWER($1 || ' ' || $2)
        END;
$$
LANGUAGE SQL IMMUTABLE STRICT;

-- Positional notation:
SELECT concat_lower_or_upper('Hello', 'World', true);
```

Named notation separates the parameter name from the argument with `=>`. For backward compatibility, the `:=` separator is also supported.

``` postgresql
-- Named notation:
SELECT concat_lower_or_upper(a => 'Hello', b => 'World');
```

Mixed notations requires _all_ positional arguments to come before named parameters.

``` postgresql
SELECT concat_lower_or_upper('Hello', 'World', uppercase => true);
```

Named and mixed notations cannot be used with aggregate functions, unless they're used as window functions.

# Window Functions

A window function applies an aggregate-like function over a portion of rows selected by a query. Unlike aggregate functions, the input rows are not reduced to a single row—each row remains separate in the output.

A window function is syntactically different from a regular or aggregate function by the presence of the `OVER` clause directly after the call. The clause specifies how the rows are split up for processing.

Window functions are only permitted in the `SELECT` list and the `ORDER BY` clause of a query.

The `PARTITION BY` sub-clause partitions (groups) the rows sharing the same values for the provided expression. These partitions are processed separately by the window function. The `PARTITION BY` clause is similar to `GROUP BY` except its values are always expressions, not output-column names or numbers. The `ORDER BY` sub-clause can be used to order each resulting partition. Then, for each row, the window function is computed across the rows that are in the same partition.

If the `PARTITION BY` clause is omitted then there will only be one resulting partition containing all of the rows.

For each row, there is a set of rows within its partition called its _window frame_. Many window functions act only on the rows of the window frame rather than the whole partition. By default, if the `ORDER BY` clause is provided, then the window frame consists of all rows from the start of the partition (as defined by the order) up through the current row and including any rows considered equal to the current row as defined by the order.

The window frame can be specified using `RANGE` mode or `ROWS` mode.

``` postgresql
{ RANGE | ROWS } frame_start
{ RANGE | ROWS } BETWEEN frame_start AND frame_end

-- where frame_start and frame_end is one of:
UNBOUNDED PRECEDING
CURRENT ROW
UNBOUNDED FOLLOWING

-- as well as these in ROWS mode:
x PRECEDING
x FOLLOWING
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

# Queries

Expressions can be written in the `SELECT` output list.

The general syntax of the `SELECT` command is:

``` postgresql
[WITH with_queries]
SELECT select_list
FROM table_expression [sort_specification];
```

## SELECT Lists

The table expression is passed on as an intermediate table for processing by the `SELECT` list, which determines which columns of the intermediate table are output.

Entries in a `SELECT` list can be given names for subsequent processing, such as in a `GROUP BY` clause. If no name is given, the default column name is given, which is the column name for column references, the function name for function calls, or a generated generic name for complex expressions.

``` postgresql
SELECT a AS value, b + c AS sum FROM …
```

After processing the `SELECT` list, it's possible to eliminate duplicate rows in the result table with the `DISTINCT` keyword. The opposite is `ALL` which explicitly requests the default behavior of retaining all rows.

Two rows are considered distinct if they differ in at least one column value. `NULL` values are considered equal for this particular comparison. It's also possible to specify arbitrary value expression(s) with `DISTINCT ON`, so that a set of rows for which all expressions are equal are considered duplicates, and only the first row of such a set is retained. Note however that `DISTINCT ON` is considered bad practice due to the potentially indeterminate nature of its results, and `FROM` and `GROUP BY` can be used instead.

``` postgresql
SELECT DISTINCT select_list …
SELECT DISTINCT ON (expression [, expression …]) select_list …
```

The table expression can be omitted entirely to simply compute values, and more generally the `SELECT` list can make calculations from columns.

``` postgresql
SELECT 3 * 4;

SELECT a, b + c FROM table1;
```

The command `TABLE name` is equivalent to `SELECT * FROM name`.

## Table Expressions

A _table expression_ computes a table. Table expressions can be as simple as `some_table` which reads just one table, or more complex constructs of base tables, joins, and subqueries.

The optional `WHERE`, `GROUP BY`, and `HAVING` clauses in the table expression specify a pipeline of transformations performed on the table derived in the `FROM` clause, each producing a virtual table that provides the rows that are passed to the `SELECT` list to compute the output rows of the query.

The `FROM` clause derives a table from one or more other tables specified in a comma-separate table reference list. A table reference can be a table name or a derived table such as a subquery, `JOIN` construct, or complex combinations of each.

When more than one table reference is listed in the `FROM` clause, the tables are cross-joined (Cartesian product of their rows).

Note that with respect to table inheritance, if a table reference names a table that is the parent of an inheritance hierarchy, all rows of that table and its descendants are produced, unless the `ONLY` keyword precedes the table name. Remember that an asterisk `*` following the table name explicitly requests the default behavior of including all descendant tables.

``` postgresql
FROM table_references…
```

## Table and Column Aliases

Temporary _table aliases_ can be given to tables and complex table references. The alias _becomes_ the new name throughout the rest of the query; it's not longer possible to refer to the table by the original name.

``` postgresql
FROM table_reference AS alias;
FROM table_reference alias;

SELECT *
FROM some_very_long_table_name s JOIN another_fairly_long_name a ON s.id = a.num;
```

Table aliases are necessary when joining a table to itself or a subquery.

``` postgresql
SELECT *
FROM people AS mother JOIN people AS child ON mother.id = child.mother_id;
```

Table columns can also be given aliases. Only the specified columns are renamed.

``` postgresql
FROM table_reference [AS] alias (column1 [, column2 [, …]])
```

Note that applying an alias to the output of a `JOIN` clause hides the original names _within_ the `JOIN`.

``` postgresql
-- Valid
SELECT a.* FROM my_table AS a JOIN your_table AS b ON …

-- Invalid; names comprising join C are hidden
SELECT a.* FROM (my_table AS a JOIN your_table AS b ON …) AS c
```

## Joins

A _joined table_ is one derived from two other (real or derived) tables via a _join_. All join types can be chained together or nested. Parentheses can be used to control join order, otherwise they nest left-to-right.

The general syntax of a joined table is:

``` postgresql
T1 join_type T2 [ join_condition ]
```

Assume that `T1` has `$N$` rows and `T2` has `$M$` rows.

### Cross Join

``` postgresql
T1 CROSS JOIN T2
```

For every possible combination of rows from `T1` and `T2` (Cartesian product), the joined table will contain a row consisting of all columns in `T1` followed by all columns in `T2`. The joined table will have `$N \cdot M$` rows.

* For each row `R1` of `T1`:
    * For each row `R2` of `T2`:
        * Add row concatenation from `R1` and `R2` to joined table

Note that the following are equivalent:

``` postgresql
-- These are all equivalent:
FROM T1 CROSS JOIN T2

FROM T1 INNER JOIN T2 ON TRUE

FROM T1, T2
```

Note that the latter equivalence doesn't necessarily hold when more than two tables appear since `JOIN` binds more tightly than comma.

``` postgresql
-- This condition can reference T1
FROM T1 CROSS JOIN T2 INNER JOIN T3 ON condition;

-- This condition cannot reference T1
FROM T1, T2 INNER JOIN T3 ON condition;
```

### Qualified Joins

``` postgresql
T1 { [INNER] | { LEFT | RIGHT | FULL } [OUTER] } JOIN T2
  ON boolean_expression

T1 { [INNER] | { LEFT | RIGHT | FULL } [OUTER] } JOIN T2
  USING ( join column list )

T1 NATURAL { [INNER] | { LEFT | RIGHT | FULL } [OUTER] } JOIN T2
```

The `LEFT`, `RIGHT`, and `FULL` keywords imply an outer join.

The join condition determines which rows from the two source tables are considered to match, and is specified in the `ON` or `USING` clause, or implicitly via `NATURAL`.

The `ON` clause takes an arbitrary boolean expression, even those which do not directly relate columns on either table, such as testing a left table's column against a constant. Such a boolean expression is tested _before_ the join, whereas a condition on a `WHERE` clause would be tested _after_ the join. This distinction matters for outer joins.

The `USING` clause is a shorthand for the common situations where both sides of the join use the same name for the joining column(s). The following clauses are equivalent.

``` postgresql
ON T1.a = T2.a AND T1.b = T2.b;
USING (a, b);
```

The `ON` clause produces all columns from `T1` followed by those in `T2`, while `USING` produces one output column for each of the listed column pairs in listed order followed by the remaining columns in `T1` and the remaining columns in `T2`.

The `NATURAL` clause is a shorthand equivalent to `USING` on all column names that appear in both input tables. If there are no common column names, then `NATURAL` behaves like a `CROSS JOIN`. Note that the use of `NATURAL` is risky as future changes to either table can manifest a new matching column name.

### Inner Join

* For each row `R1` of `T1`:
    * For each row `R2` of `T2`:
        * If `condition(R1, R2)`: add row `R1` + `R2`

The join condition of an inner join can be written either in the `WHERE` clause or in the `JOIN` clause.

``` postgresql
FROM a, b WHERE a.id = b.id AND b.val > 5;

-- Equivalent
FROM a INNER JOIN b ON (a.id = b.id) WHERE b.val > 5;
```

### Left Outer Join

* For each row `R1` of `T1`:
    * For each row `R2` of `T2`:
        * If `condition(R1, R2)`: add row: `R1` + `R2`
    * If no row was added: add row: `R1` + `T2` row with `NULL` values

Like an inner join, except that if the condition is not satisfied between `R1` and _any_ `R2`, a row is still added for `R1` and `T2` with `NULL` columns. This means that the joined table always has at least one row for each row in `T1`, i.e. at least `$N$` rows.

Be careful with `WHERE` filters. Since they apply to all rows produced by joins, if you refer only to `T2` columns, then when there is no corresponding `T2` row for any `T1` row, the column won't exist and so the condition may yield `false`, filtering the row out and essentially negating the effects of the left outer join, where a row in `T1` is represented regardless of a corresponding row in `T2`.

### Right Outer Join

* For each row `R2` of `T2`:
    * For each row `R1` of `T1`:
        * If `condition(R1, R2)`: add row: `R1` + `R2`
    * If no row was added: add row: `T1` row with `NULL` values + `R2`

Like an inner join, except that if the condition is not satisfied between `R2` and _any_ `R1`, a row is still added for `R2` and `T1` with `NULL` columns. This means that the joined table always has at least one row for each row in `T2`, i.e. at least `$M$` rows.

Be careful with `WHERE` filters. Since they apply to all rows produced by joins, if you refer only to `T1` columns, then when there is no corresponding `T1` row for any `T2` row, the column won't exist and so the condition may yield `false`, filtering the row out and essentially negating the effects of the right outer join, where a row in `T2` is represented regardless of a corresponding row in `T1`.

### Full Outer Join

* For each row `R1` of `T1`:
    * For each row `R2` of `T2`:
        * If `condition(R1, R2)`: add row: `R1` + `R2`
    * If no row was added: add row: `R1` + `T2` row with `NULL` values
* For each row `R2` of `T2`:
    * For each row `R1` of `T1`:
        * If `condition(R1, R2)`: add row: `R1` + `R2`
    * If no row was added: add row: `T1` row with `NULL` values + `R2`
* Ensure there is only one row for any `R1` + `R2`

This ensures that every row in each table is represented regardless of whether the condition is satisfied, resulting in at least `$N \cdot M$` rows.

## Derived Table Subqueries

Subqueries specifying a derived table must be within parentheses and must be assigned a table alias name.

``` postgresql
FROM (SELECT * FROM table1) AS alias_name

-- Equivalent:
FROM table1 AS alias_name
```

A subquery can be a raw `VALUES` list. Assigning names to the columns of a `VALUES` list is optional but good practice.

``` postgresql
FROM (VALUES ('anne', 'smith'), ('bob', 'jones'), ('joe', 'blow')) AS names(first, last)
```

## Table Functions

Table functions produce a set of rows made up of either base or composite data types. They are used like a table, view, or subquery in the `FROM` clause.

``` postgresql
CREATE TABLE foo (fooid int, foosubid int, fooname text);

CREATE FUNCTION getfoo(int) RETURNS SETOF foo AS $$
  SELECT * FROM foo WHERE fooid = $1;
$$ LANGUAGE SQL;

SELECT * FROM getfoo(1) AS t1;

SELECT * FROM foo
  WHERE foosubid IN (SELECT foosubid
                     FROM getfoo(foo.fooid) z
                     WHERE z.fooid = foo.fooid);

CREATE VIEW vw_getfoo AS SELECT * FROM getfoo(1);

SELECT * FROM vw_getfoo;
```

If the table function is declared as returning the pseudotype `record`, the expected row structure can be specified when the function is called.

``` postgresql
function_call [AS] alias (column_definition [, … ])
function_call AS [alias] (column_definition [, … ])
ROWS FROM(… function_call AS (column_definition [, …]) [, …])
```

The `ROWS FROM` syntax can be used to combine table functions. The `WITH ORDINALITY` clause can be used to add a column of type `bigint` to the function result columns, starting with 1.

``` postgresql
ROWS FROM(function_call [, …]) [WITH ORDINALITY] [[AS] table_alias [(column_alias [, …])]]
```

## Lateral Subqueries

Subqueries in a `FROM` clause preceded by the keyword `LATERAL` can reference columns provided by preceding `FROM` items. Otherwise, each subquery is evaluated independently and is therefore unable to cross-reference any other `FROM` item.

`LATERAL` is primarily useful when the cross-referenced column is necessary for computing the row(s) to be joined.

It's often useful to `LEFT JOIN` to a `LATERAL` subquery so that source rows appear in the result even if the `LATERAL` subquery produces no rows for them.

``` postgresql
-- Find manufacturers with no products
SELECT m.name
FROM manufacturers m LEFT JOIN LATERAL get_product_names(m.id) pname ON true
WHERE pname IS NULL;
```

Table functions can also be `LATERAL`, but for arguments it's optional since they can already contain references to columns provided by preceding `FROM` items.

A `LATERAL` item can appear at top level in the `FROM` list or within a `JOIN` tree, in which case it can also refer to any items that are on the left-hand side of the `JOIN` that it's on the right-hand side of.

Evaluation of `FROM` items containing `LATERAL` cross-references proceeds like so:

* for each row of the `FROM` item providing the cross-referenced column(s), or set of rows of multiple `FROM` items:
    * evaluate the `LATERAL` item using that row or row set's values of the columns
    * resulting row(s) are joined as usual with the rows they were computed from

``` postgresql
SELECT * FROM foo, LATERAL (SELECT * FROM bar WHERE bar.id = foo.bar_id) ss;

-- Equivalent
SELECT * FROM foo, bar WHERE bar.id = foo.bar_id;
```

## Scalar Subqueries

A _scalar subquery_ is an ordinary parenthesized `SELECT` query that returns exactly _one_ row with _one_ column. It would be an error if it returned more than one row or column, but returning nothing at all is interpreted as being `NULL`.

``` postgresql
SELECT
  name,
  (SELECT max(pop) FROM cities WHERE cities.state = states.name)
FROM states;
```

## WHERE Clause

After processing the `FROM` clause, each row of the derived virtual table is checked against the search condition of the `WHERE` clause, which is any value expression that returns a value of type boolean, and if it fails the condition the row is discarded.

``` postgresql
WHERE search_condition
```

## GROUP BY and HAVING Clauses

After passing the `WHERE` filter, the derived input table may be subject to grouping via the `GROUP BY` clause and elimination of rows via the `HAVING` clause.

Strict SQL limits `GROUP BY` to columns of the source table but PostgreSQL extends it to columns in the `SELECT` list, as well as grouping by value expressions.

``` postgresql
SELECT select_list FROM … [WHERE …]
  GROUP BY grouping_column_reference [, grouping_column_reference]…
```

The `GROUP BY` clause groups together those rows in a table that have the same values in all of the listed columns, combining each set of rows having column values into one _group row_ that represents all rows in the group. Generally if a table is grouped, columns not listed in the `GROUP BY` clause cannot be referenced except in aggregate expressions.

``` postgresql
SELECT x, sum(y) FROM test1 GROUP BY x;

-- Calculate the total sales of each product:
SELECT product_id, p.name, (sum(s.units) * p.price) AS sales
  FROM products p LEFT JOIN sales s USING (product_id)
  GROUP BY product_id, p.name, p.price;
```

Grouping without aggregate expressions effectively calculates the set of distinct values in a column, which can also be achieved using the `DISTINCT` clause.

If a query contains aggregate function calls but no `GROUP BY` clause, the result is a single group row (or none if eliminated by `HAVING`). The same is true with the mere presence of a `HAVING` clause.

The `HAVING` clause can be used to eliminate groups from the result. Expressions in the `HAVING` clause may refer to both grouped and ungrouped expressions (which would involve aggregate functions).

``` postgresql
SELECT select_list
FROM … [WHERE …]
GROUP BY …
HAVING boolean_expression
```

The `GROUPING SETS` syntax can be used to group into separate sets and aggregates computed for each group. Each sublist of `GROUPING SETS` can specify zero or more columns or expressions and they're interpreted as in `GROUP BY`. An empty grouping set means that all rows are formed into a single group, as in the case with aggregate functions with no `GROUP BY` clause.

References to grouping columns or expressions are replaced by `NULL` values in result rows for grouping sets in which those columns do not appear.

``` postgresql
SELECT brand, size, sum(sales)
FROM items_sold
GROUP BY GROUPING SETS ((brand), (size), ());
```

The `ROLLUP` clause is a shorthand representing the given list of expressions and all prefixes of the list including the empty list.

``` postgresql
ROLLUP ( e1, e2, e3, … )

-- Equivalent
GROUPING SETS (
  ( e1, e2, e3, … ),
  …
  ( e1, e2 ),
  ( e1 ),
  ( )
)
```

The `CUBE` clause is a shorthand representing the given list and all of its possible subsets (the power set).

``` postgresql
CUBE ( a, b, c )

-- Equivalent
GROUPING SETS (
  ( a, b, c ),
  ( a, b    ),
  ( a,    c ),
  ( a       ),
  (    b, c ),
  (    b    ),
  (       c ),
  (         )
)
```

## Window Function Processing

Window functions are evaluated after any grouping, aggregation and `HAVING` filtering is performed, so that if a query has any aggregates, `GROUP BY`, or `HAVING`, then the rows seen by the window functions are the group rows and not the original table rows.

## Combining Queries

The results of two queries can be combined with set operations union, intersection, and difference. These operations can be nested and chained. Each operation removes duplicates unless `ALL` is specified.

In order to compute a union, intersection, or difference of two queries, the queries must be _union compatible_, meaning they return the same number of columns and the corresponding columns have compatible data types.

``` postgresql
query1 UNION [ALL] query2
query1 INTERSECT [ALL] query2

-- Set difference
query1 EXCEPT [ALL] query2
```

## Sorting Rows

After a query has produced an output table (i.e. the `SELECT` list has been processed), it can optionally be sorted with the `ORDER BY` clause. The sort expression(s) can be any expression that would be valid in the query's `SELECT` list. If more than one expression is specified, the later values are used to sort rows that are equal according to preceding values. Each expression can have its own ordering independent of the others'. The "smaller" value is defined in terms of the less-than operator `<`.

The `NULLS FIRST` and `NULLS LAST` options are used to determine whether `NULL`s appear before or after non-`NULL` values in the sort ordering. The default is for `NULL`s to be treated as larger than any non-`NULL` value, i.e. `NULLS FIRST` for `DESC` ordering and `NULLS LAST` otherwise.

``` postgresql
SELECT select_list
  FROM table_expression
  ORDER BY sort_expression1 [ASC | DESC] [NULLS { FIRST | LAST }]
           [, sort_expression2 [ASC | DESC] [NULLS { FIRST | LAST }] …]
```

The sort expression can be the column label or a number of an output column.

``` postgresql
SELECT a + b AS sum, c FROM table1 ORDER BY sum;

SELECT a, max(b) FROM table1 GROUP BY a ORDER BY 1;
```

## LIMIT and OFFSET

A query's results can be limited to a certain maximum number of rows with `LIMIT`. It's important to use an `ORDER BY` clause since the returned rows will be unpredictable.

``` postgresql
SELECT select_list
  FROM table_expression
  [ ORDER BY … ]
  [ LIMIT { number | ALL } ] [ OFFSET number ]
```

The `OFFSET` clause can be used to skip a specified number of rows before beginning to return rows. An `OFFSET` is processed before any `LIMIT`.

Note that the rows skipped by an `OFFSET` clause still have to be computed by the server, so a large `OFFSET` may be inefficient.

## VALUES Lists

The `VALUES` syntax can be used to generate a "constant table" that can be used in a query without actually having to create and populate an on-table disk.

``` postgresql
VALUES ( expression [, …] ) [, …]
```

Each parenthesized list of expressions generates a row in the table, so each list must have the same number of elements, and corresponding elements must have compatible data types. The data type assigned to each data type is determined using the rules for `UNION`.

``` postgresql
VALUES (1, 'one'), (2, 'two'), (3, 'three');

-- Effectively equivalent to:
  SELECT 1 AS column1, 'one' AS column2
UNION ALL
  SELECT 2, 'two'
UNION ALL
  SELECT 3, 'three';
```

PostgreSQL assigns the column names `column1`, `column2`, etc., although column names aren't specified by the SQL standard so it's a good practice to name them explicitly.

``` postgresql
SELECT *
FROM (VALUES (1, 'one'),
             (2, 'two'),
             (3, 'three')) AS t (num, letter);
```

The `VALUES` command followed by expression lists is treated syntactically equivalent to a `SELECT` statement and can appear anywhere a `SELECT` can. It can be used as part of a `UNION` and can have a sort specification attached. It's most commonly used as a data source in an `INSERT` command and as a subquery.

## Common Table Expressions

The `WITH` syntax can be used to write auxiliary statements, often referred to as _Common Table Expressions_ (CTEs), for use in a larger query. Common Table Expressions can be thought of as defining temporary tables that exist just for one query. Each auxiliary statement in a `WITH` clause can be a `SELECT`, `INSERT`, `UPDATE`, or `DELETE`. The `WITH` clause is attached to a primary statement that can also be any one of those.

### SELECT in WITH

The basic value of `SELECT` in `WITH` is to decompose complex queries into simpler parts.

`WITH` queries are evaluated only once per execution of the parent query even if they're referred to more than once by the parent query or sibling `WITH` queries. Expensive calculations and queries that are needed in multiple places can be placed within a `WITH` query to avoid redundant work.

The following query displays per-product sales totals in only the top sales regions. Writing it without `WITH` would have necessitated two levels of nested sub-`SELECT`s.

``` postgresql
WITH regional_sales AS (
    SELECT region, SUM(amount) AS total_sales
    FROM orders
    GROUP BY region
  ), top_regions AS (
    SELECT region
    FROM regional_sales
    WHERE total_sales > (SELECT SUM(total_sales)/10 FROM regional_sales)
  )
SELECT region,
       product,
       SUM(quantity) AS product_units,
       SUM(amount) AS product_sales
FROM orders
WHERE region IN (SELECT region FROM top_regions)
GROUP BY region, product;
```

The `RECURSIVE` modifier allows a `WITH` query to refer to its own output. A recursive `WITH`'s general form is a non-recursive term (base case), then a `UNION`, then a recursive term. They're usually used to deal with hierarchically-structured data.

``` postgresql
WITH RECURSIVE t(n) AS (
    VALUES (1)
  UNION ALL
    SELECT n+1 FROM t WHERE n < 100
)
SELECT sum(n) FROM t;
```

A recursive query is evaluated as follows. Note that the process more closely describes iteration, not recursion.

1. Evaluate the non-recursive term. For `UNION` (not `UNION ALL`), discard duplicate rows. Include the remaining rows in the result of the recursive query and also add them to a temporary _working table_.
2. As long as the working table is not empty, repeat these steps:
    a. Evaluate the recursive term, substituting the current contents of the working table for the recursive self-reference. Discard `UNION` duplicates. Include remaining rows in the result of the recursive query and add them to a temporary _intermediate table_.
    b. Replace the contents of the working table with those of the intermediate table, then empty the intermediate table.

Sometimes it may be necessary to maintain an array of visited values in order to ensure termination of the recursive query. A query can be tested for termination by adding a `LIMIT` to the parent query in some cases.

``` postgresql
-- Can loop if there are cycles in the graph.
-- UNION would take care of duplicates if it weren't for the
-- depth computed column.
WITH RECURSIVE search_graph(id, link, data, depth) AS (
    SELECT g.id, g.link, g.data, 1
    FROM graph g
  UNION ALL
    SELECT g.id, g.link, g.data, sg.depth + 1
    FROM graph g, search_graph sg
    WHERE g.id = sg.link
)
SELECT * FROM search_graph;

-- Keeps track of the path and whether it's a cycle.
WITH RECURSIVE search_graph(id, link, data, depth, path, cycle) AS (
    SELECT g.id, g.link, g.data, 1,
      ARRAY[g.id], -- Path only consists of starter node.
      false        -- Can't possibly be a cycle yet.
    FROM graph g
  UNION ALL
    SELECT g.id, g.link, g.data, sg.depth + 1,
      path || g.id,    -- Append node to path.
      g.id = ANY(path) -- It's a cycle if node has been visited.
    FROM graph g, search_graph sg
    WHERE g.id = sg.link AND NOT cycle
)
SELECT * FROM search_graph;
```

### Data-Modifying Statements in WITH

Data-modifying statements can be used in `WITH` to perform multiple operations in the same query. For this purpose, data-modifying statements in `WITH` usually have `RETURNING` clauses.

Note that data-modifying statements in `WITH` are executed exactly once and always to completion, regardless of whether the primary query reads any of the output, unlike `SELECT` in `WITH` which is executed only as far as the primary query demands output.

Sub-statements in `WITH` are executed concurrently with each other and with the main query, so the order in which updates incurred by data-modifying statements in `WITH` actually occur is unpredictable. All statements are executed with the same snapshot, so they _do not_ see each other's effects on the target tables, so `RETURNING` is the only way to communicate changes between different `WITH` sub-statements and the main query.

``` postgresql
-- Returns `products` unaffected by UPDATE.
WITH t AS (
  UPDATE products SET price = price * 1.05
  RETURNING *
)
SELECT * FROM products;

-- Returns `products` affected by UPDATE.
WITH t AS (
  UPDATE products SET price = price * 1.05
  RETURNING *
)
SELECT * FROM t;
```

The following query moves rows from the `products` to `products_log`.

Note that data-modifying statements are only allowed in `WITH` clauses that are attached to the top-level statement, which is why the CTE is attached to the `INSERT` statement and not the sub-`SELECT`.

``` postgresql
WITH moved_rows AS (
  DELETE FROM products
  WHERE
      "date" >= '2010-10-01' AND
      "date" < '2010-11-01'
  RETURNING *
)
INSERT INTO products_log
SELECT * FROM moved_rows;
```

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

# Expression Evaluation

The order of evaluation of subexpressions is not defined; they're not evaluated in left-to-right order. Certain expressions may be short-circuited, _not_ necessarily in a left-to-right order. For example, it's possible that `somefunc()` is never called at all:

``` postgresql
SELECT somefunc() OR true;
```

Evaluation order can be forced with a `CASE` expression. For example, to avoid dividing by zero:

``` postgresql
-- Right operand may be evaluated before left,
-- defeating the purpose of the guard check.
SELECT … WHERE x > 0 AND y / x > 1.5;

-- Explicitly force the evaluation order:
SELECT …
WHERE CASE WHEN x > 0
      THEN y / x > 1.5
      ELSE false
      END;
```

This can't be used to prevent early evaluation of constant subexpressions, such as functions and operators marked `IMMUTABLE`, which may be evaluated when the query is planned rather than when it is executed. For example, the planner may try to simplify a constant subexpression which divides by zero even if, when executed, that subexpression would never be evaluated. Consider a table where the value of all rows' column `x` is greater than `0`.

``` postgresql
SELECT
  CASE WHEN tab.x > 0
  -- In practice, it may be that x > 0 for all rows.
  THEN tab.x
  -- Regardless, the planner may attempt to simplify
  -- this constant subexpression.
  ELSE 1 / 0
  END
FROM tab;
```

The above situation may also occur when queries are executed within functions, where function arguments and local variables may be inserted into queries as constants for planning purposes.

Note that `CASE` inhibits optimization attempts, so in this case it would be better to use math to avoid division-by-zero:

``` postgresql
SELECT … WHERE y > 1.5 * x;
```

Also, since aggregate expressions are computed before other expressions in a `SELECT` list or `HAVING` clause, `CASE` cannot prevent evaluation of interior aggregate expressions. For example, in the query below, the `min()` and `avg()` aggregates are computed over all input rows before the `CASE` clause ever takes effect, which would yield a division-by-zero if any row has zero employees. This query should instead use `WHERE` or `FILTER` clauses to discard rows with zero employees.

``` postgresql
SELECT CASE WHEN min(employees) > 0
            THEN avg(expenses / employees)
       END
FROM departments;
```

# Aggregate Functions

An aggregate function reduces multiple inputs to a single output value.

The difference between `WHERE` and `HAVING` is that `WHERE` selects the rows before groups and aggregates are computed, whereas `HAVING` selects rows _after_ that. This is why an aggregate function may only appear in the result list or the `HAVING` clause and not in a `WHERE` clause.

An aggregate expression within a subquery usually applies over the rows of the subquery, _except when_ its arguments or `FILTER` clause contain any outer-level variables, in which case the aggregate then belongs to the nearest outer level and so applies over the rows of _that_ query, so that the aggregate expression is effectively an outer reference for the subquery it appears in, and acts as a constant over any one evaluation of the subquery. In this case, the placement restriction (i.e. in the result list or `HAVING` clause) applies with respect to the query level that the aggregate belongs to.

Most aggregate functions ignore `NULL` inputs, i.e. rows in which one or more of the expression(s) yield `NULL` are discarded. Assume this to be true for all built-in functions unless otherwise specified.

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

Each data type has an external representation determined by its input and output functions, and some input and output functions are not invertible, so the result of an output function may lose accuracy compared to the original input.

## Integer Types

| Name       | Size    |
| :--------- | :------ |
| `smallint` | 2 bytes |
| `integer`  | 4 bytes |
| `bigint`   | 8 bytes |

Numeric types of two, four, and eight-byte integers and four and eight-byte floating-point numbers, and selectable-precision decimals.

Attempts to store values outside of the allowed range for an integer type results in an error.

## Arbitrary Precision Types

| Name       | Size     |
| :--------- | :------- |
| `numeric`  | variable |
| `decimal`  | variable |

The `numeric` and `decimal` types are equivalent.

The `numeric` type can store very large numbers, and is recommended for monetary amounts and other exact quantities. The _scale_ of a `numeric` is the count of decimal digits in the fractional part, and its _precision_ is the total count of significant digits in the whole number (i.e. _both_ sides of the decimal point).

The `numeric`'s maximum precision and scale can be set when specifying a column type as `NUMERIC(precision, scale)`, and the scale may be omitted and defaulted to zero. Omitting both the maximum precision and scale implies a `numeric` that can store values of any precision and scale up to the implementation's limit. For portability reasons, always specify the precision and scale.

Note that the precision and scale are maximums, not fixed sizes; the values are not stored with leading or trailing zeroes, so `numeric` is more similar to `varchar(n)` than to `char(n)`.

If the value to be stored is greater than the column's declared scale, it is rounded to the specified scale (fractional digits), and if the number of digits to the left of the decimal point exceeds the precision minus the scale, an error is raised. The `numeric` type rounds ties away from zero.

The `numeric` type allows the special value `NaN` which must be single-quoted. Unlike other `NaN` implementations which don't consider it to be equal to any other numeric value including `NaN` itself, PostgreSQL treats `NaN` as equal to itself and greater than all non-`NaN` values in order to allow them to be sorted and used in tree-based indexes.

## Floating-Point Types

| Name               | Size     |
| :---------         | :------- |
| `real`             | 4 bytes  |
| `double precision` | 8 bytes  |

The `real` and `double precision` types are IEEE 754 Binary Floating-Point numbers. Even though IEEE 754 specifies that `NaN` should nto compare equal to any other floating-point value including `NaN` itself, PostgreSQL treats `NaN` as equal to itself and greater than all non-`NaN` values in order to allow them to be sorted and used in tree-based indexes.

The SQL standard notation `float` and `float(p)` can be used to specify inexact numeric types, where `p` specifies the minimum acceptable precision in _binary_ digits.

## Serial Types

| Name          | Size     | Column Type |
| :---------    | :------- | :---------- |
| `smallserial` | 2 bytes  | `smallint`  |
| `serial`      | 4 bytes  | `integer`   |
| `bigserial`   | 8 bytes  | `bigint`    |

The `serial` types aren't true types but rather a notational convenience for creating unique identifier columns similar to other databases' `AUTO_INCREMENT`.

Since these types are implemented using sequences, there may be gaps in the sequence of values which appear in the column even if no rows are ever deleted, for example, if an inserting transaction is rolled back.

In order to insert the next value of the sequence into a `serial` column, simply specify that the column should be assigned its default value, either by excluding the column or through the use of `DEFAULT`.

The use of a `SERIAL` "type" essentially creates an integer column with its default values assigned from a sequence generator, with a `NOT NULL` constraint. The sequence is marked as "owned by" the column so that it is dropped if the column or table is dropped.

It may also be preferable to add `UNIQUE` and `PRIMARY KEY` constraints to prevent duplicates from being inserted accidentally.

``` postgresql
CREATE TABLE tablename (
  colname SERIAL
);

-- Equivalent to this:
CREATE SEQUENCE tablename_colname_seq;

CREATE TABLE tablename (
  colname integer NOT NULL DEFAULT nextval('tablename_colname_seq')
);

ALTER SEQUENCE tablename_colname_seq OWNED BY tablename.colname;
```

## Monetary Types

| Name       | Size     |
| :--------- | :------- |
| `money`    | 8 bytes  |

The `money` type stores a currency amount with a fixed fractional precision. Input is accepted as integer or floating-point literals as well as typical currency formatting such as `$1,000.00`.

The output of `money` is locale-sensitive as dictated by `lc_monetary`. When restoring a dump into a new database, care should be taken to ensure that the `lc_monetary` setting is the same.

Dividing one `money` value by another cancels out the currency units, resulting in a `double precision` value.

## Character Types

| Name                                 | Size                       |
| :---------                           | :-------                   |
| `character varying(n)`, `varchar(n)` | variable-length with limit |
| `character(n)`, `char(n)`            | fixed-length, blank padded |
| `text`                               | variable unlimited length  |

An attempt to store a longer string into a column of max or fixed-length types results in an error unless the excess characters are spaces, in which case it is truncated to the maximum length [^sql_standard_requirement].

[^sql_standard_requirement]: This is required by the SQL standard.

Strings of shorter length are space-padded in `character(n)` columns, and displayed as such. However, trailing spaces are treated as semantically insignificant and disregarded when comparing two values of type `character`.

Explicitly casting a value to `character varying(n)` or `character(n)` causes an over-length value to be truncated to `n` characters without raising an error [^sql_standard_requirement].

A type of `character` without specifying the length is equivalent to `character(1)`.

Specifying a type of `character varying` without a length specifier makes the type accept strings of any size, similar to `text`.

Long strings are compressed by the system automatically. Very long strings are stored in background tables so that they don't interfere with rapid access to shorter column values.

The longest possible `character` string is about 1 GB.

Although `character(n)` may have performance advantages in other databases, there is no performance difference between all string types in PostgreSQL, although in practice `character(n)` is usually the slowest because of its additional storage costs.

## Binary Data Types

| Name       | Size                          |
| :--------- | :-------                      |
| `bytea`    | 1-4 bytes + the binary string |

A `bytea` binary string is a sequence of bytes, and is an appropriate type for storing data as "raw bytes."

The SQL standard defines a binary string type `BLOB` or `BINARY LARGE OBJECT` which has an input format different from `bytea` but the provided functions and operators are mostly the same.

Two external formats for input and output are supported: PostgreSQL's historical "escape" format and "hex" format. Both are always accepted on input.

The "hex" format encodes binary data as two hexadecimal digits per byte, with the most significant nibble first, with the entire string preceded by the sequence `\x` as a way of distinguishing it from escape format. The hex digits can be upper or lowercase, with optional whitespace between digit pairs. It tends to be faster to convert than escape format, so its use is preferred.

``` postgresql
SELECT E'\\xDEADBEEF';
```

The "escape" format represents a binary string as a sequence of ASCII characters, converting those bytes that cannot be represented as ASCII into special escape sequences. All octet values _can_ be escaped, but certain octet values _must_ be escaped. To escape an octet, convert it to its three-digit octal value and precede it by a backslash (or two if necessary).

The reason that multiple backslashes may be required is that an input string written as a string literal must pass through two parse phases in PostgreSQL. The first backslash of each pair is interpreted as an escape character by the string-literal parser and is consumed, leaving the second backslash to be recognized by the `bytea` input function as starting either a three digit octal value or escaping another backslash. For example, `E'\\001` becomes `\001` after passing through the escape string parser, which is then sent to the `bytea` input function where it's converted to a single octet with a decimal value of 1.

The use of this format is discouraged.

``` postgresql
SELECT E'\\000'::bytea;
SELECT E'\''::bytea;
SELECT E'\\\\'::bytea;
SELECT E'\\001'::bytea;
```

## Date and Time Types

| Name                                  | Size     | Description    |
| :---------                            | :------- | :----------    |
| `timestamp [(p)] [without time zone]` | 8 bytes  | date, time     |
| `timestamp [(p)] with time zone`      | 8 bytes  | date, time, tz |
| `date`                                | 4 bytes  | date           |
| `time [(p)] [without time zone]`      | 8 bytes  | time           |
| `time [(p)] with time zone`           | 12 bytes | time, tz       |
| `interval [fields] [(p)]`             | 16 bytes | time interval  |

The SQL standard requires that writing just `timestamp` be equivalent to `timestamp without time zone`. As a PostgreSQL extension, the type `timestampz` is accepted as an abbreviation for `timestamp with time zone`.

The `time`, `timestamp`, and `interval` types accept an optional precision value `p` that specifies the number of fractional digits retained in the "seconds" field.

The `interval` type can restrict the set of stored fields by writing one of the following phrases:

* `YEAR`
* `MONTH`
* `DAY`
* `HOUR`
* `MINUTE`
* `SECOND`
* `YEAR TO MONTH`
* `DAY TO HOUR`
* `DAY TO MINUTE`
* `DAY TO SECOND`
* `HOUR TO MINUTE`
* `HOUR TO SECOND`
* `MINUTE TO SECOND`

Of course, if the precision parameter is also specified then the `SECOND` field must be included.

Date and time input is accepted in almost any reasonable format including [ISO 8601], SQL-compatible, traditional POSTGRES, etc. Any date or time literal input needs to be enclosed in single quotes, like text strings.

[ISO 8601]: https://en.wikipedia.org/wiki/ISO_8601

``` postgresql
type [(p)] 'value'
```

Valid inputs for time types consist of the time of day followed by an optional time zone. A time zone is ignored if it's input to a type without a time zone. A date is ignored unless specifying a time zone name that involves a daylight-savings rule such as `America/Los_Angeles`, in which case specifying the date is required in order to determine whether standard or daylight-savings time applies.

Valid input for the time stamp types consists of the concatenation of date and time followed by an optional time zone and an optional `AD` or `BC`.

Remember that PostgreSQL never examines the content of a literal string _before_ determining its type, so supplying a `TIMESTAMP` literal string with a time zone won't actually create a `TIMESTAMP WITH TIME ZONE` unless that type is explicitly state.

For a `TIMESTAMP WITH TIME ZONE`, the actual value that is internally stored is always in UTC. When such a value is output, it's always converted from UTC to the current `timezone` and displayed as local time in that zone. To see the time in another time zone either change `timezone` or use the `AT TIME ZONE` phrase. Similarly, conversion between `TIMESTAMP` and `TIMESTAMP WITH TIME ZONE` normally assumes that the `TIMESTAMP` should be taken as `timezone` local time, but a different one can be specified for the conversion using `AT TIME ZONE`.

Certain special values like `now` are notational shorthands that are converted to ordinary date/time values as soon as they're read.

| Input String | Valid Types                 | Description                        |
| :--          | :--                         | :--                                |
| `epoch`      | `date`, `timestamp`         | 1970-01-01 00:00:00+00             |
| `infinity`   | `date`, `timestamp`         | later than all other time stamps   |
| `-infinity`  | `date`, `timestamp`         | earlier than all other time stamps |
| `now`        | `date`, `time`, `timestamp` | current transaction's start time   |
| `today`      | `date`, `timestamp`         | midnight today                     |
| `tomorrow`   | `date`, `timestamp`         | midnight tomorrow                  |
| `yesterday`  | `date`, `timestamp`         | midnight yesterday                 |
| `allballs`   | `time`                      | 00:00:00.00 UTC                    |

The current time for the corresponding date type can be obtained with:

* `CURRENT_DATE`
* `CURRENT_TIME`
* `CURRENT_TIMESTAMP`
* `LOCALTIME`
* `LOCALTIMESTAMP`

There are a variety of output styles such as `ISO` for ISO 8601. Note however that although ISO 8601 specifies separating date and time with a `T`, PostgreSQL does so with a space on output for readability and consistency with [RFC 3339].

[RFC 3339]: https://www.ietf.org/rfc/rfc3339.txt

The use of `TIME WITH TIME ZONE` is discouraged because time zones in the real world have little meaning unless associated with a date as well since the offset can vary through the year with daylight-saving time boundaries. Instead date/time types that contain both date and time should be used when using time zones. Otherwise, PostgreSQL assumes the local time zone for any type containing only either date or time.

Time zones can be specified in one of three ways. The difference between abbreviations and full names is that abbreviations represent a specific offset from UTC, whereas many full names imply a local daylight-savings time rule.

* Full IANA time zone name such as `America/Los_Angeles`. This can imply a set of daylight savings transition-date rules.
* Abbreviation such as `PST`. This only defines an offset from UTC.
* POSIX-style time zone specifications of the form `STDoffset` or `STDoffsetDST` where `STD` is a zone abbreviation, `offset` is a numeric offset in hours west from UTC, and `DST` is an optional daylight savings zone abbreviation assumed to be one hour ahead of the given offset. Such as `EST5EDT`.

    Note that in POSIX time zone names, positive offsets are used for locations _west_ of Greenwich, whereas everywhere else PostgreSQL follows ISO 8601 convention of positive timezone offsets being _east_ of Greenwich.

Interval values are written as follows, where `quantity` is a number, `unit` is a `microsecond`, `millisecond`, `second`, `minute`, `hour`, `day`, `week`, `month`, `year`, `decade`, `century`, `millenium`, or abbreviations or plurals of them, and `direction` is `ago` or empty. The `ago` direction negates all fields.

Internally interval values are stored as months, days, and seconds.

``` postgresql
[@] quantity unit [quantity unit…] [direction]
```

Quantities of days, hours, minutes, and seconds can be specified without explicit unit markings.

``` postgresql
'1 12:59:10'

-- Equivalent to:
'1 day 12 hours 59 min 10 sec'
```

Fields to the right of the least significant field allowed by the `fields` specification are silently discarded.

``` postgresql
-- Drops the seconds field, but not day field.
INTERVAL '1 day 2:03:04' HOUR TO MINUTE
```

Intervals can also be written as ISO 8601 time intervals.

``` postgresql
-- Format with designators:
P quantity unit [ quantity unit …] [ T [ quantity unit …]]

-- Alternative format:
P [ years-months-days ] [ T hours:minutes:seconds ]
```

## Boolean Type

In SQL the `boolean` type can be `true`, `false`, or `unknown` represented by `null`.

The values for `true` can be:

* `TRUE`
* `'t'`
* `'true'`
* `'y'`
* `'yes'`
* `'on'`
* `'1'`

The values for `false` can be:

* `FALSE`
* `'f'`
* `'false`
* `'n'`
* `'no`
* `'off'`
* `'0'`

## Enumerated Types

Enumerated types are created with the `CREATE TYPE` command. Enumerated types can be used in table and function definitions like any other type. Enum labels (the values) are case-sensitive, with significant white space.

The order of values in an enumerated type is the order in which they were listed when created.

``` postgresql
CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');

CREATE TABLE person (
  name text,
  current_mood mood
);

INSERT INTO person VALUES ('Moe', 'happy');

SELECT * FROM person WHERE current_mood = 'happy';
```

## Geometric Types

Points are two-dimensional points specified as a comma-delimited point with optional parentheses.

``` postgresql
( x , y )
  x , y
```

Lines are represented by the linear equation `$Ax + By + C = 0$` where `$A$` and `$B$` are not both zero. Lines are specified as comma-delimited values of `$A$`, `$B$`, and `$C$`, or as a sequence of two Points.

``` postgresql
{ A, B, C }

[ ( x1 , y1 ) , ( x2 , y2 ) ]
( ( x1 , y1 ) , ( x2 , y2 ) )
  ( x1 , y1 ) , ( x2 , y2 )
    x1 , y1   ,   x2 , y2
```

Line Segments (`lseg`) are represented by a pair of Points defining its endpoints.

``` postgresql
[ ( x1 , y1 ) , ( x2 , y2 ) ]
( ( x1 , y1 ) , ( x2 , y2 ) )
  ( x1 , y1 ) , ( x2 , y2 )
    x1 , y1   ,   x2 , y2
```

Boxes are represented by a pair of Points defining its opposite corners.

``` postgresql
( ( x1 , y1 ) , ( x2 , y2 ) )
  ( x1 , y1 ) , ( x2 , y2 )
    x1 , y1   ,   x2 , y2
```

Paths are represented by lists of connected points. Paths can be open (first and last points are considered not connected) or closed (first and last points are considered connected).

Open paths are denoted by square brackets `[]`, while open paths are denoted by parentheses `()`. Omitting the outermost parentheses implies a closed path.

``` postgresql
[ ( x1 , y1 ) , ... , ( xn , yn ) ]
( ( x1 , y1 ) , ... , ( xn , yn ) )
  ( x1 , y1 ) , ... , ( xn , yn )
  ( x1 , y1   , ... ,   xn , yn )
    x1 , y1   , ... ,   xn , yn
```

Polygons are represented by lists of points denoting its vertices.

``` postgresql
( ( x1 , y1 ) , ... , ( xn , yn ) )
  ( x1 , y1 ) , ... , ( xn , yn )
  ( x1 , y1   , ... ,   xn , yn )
    x1 , y1   , ... ,   xn , yn
```

Circles are represented by a center point and radius.

``` postgresql
< ( x , y ) , r >
( ( x , y ) , r )
  ( x , y ) , r
    x , y   , r
```

## Network Address Types

PostgreSQL supports data types to store IPv4, IPv6, and MAC addresses. These types are preferred over plain text types because of their additional input error checking and specialized operators and functions.

| Name       | Size          | Description                      |
| :--------- | :-------      | :----------                      |
| `cidr`     | 7 or 19 bytes | IPv4 and IPv6 networks           |
| `inet`     | 7 or 19 bytes | IPv4 and IPv6 hosts and networks |
| `macaddr`  | 6 bytes       | MAC addresses                    |

IPv4 addresses sort before IPv6, even those IPv4 addresses encapsulated or mapped to IPv6 addresses.

The input format for type `inet` is `address/y` where `y` is the number of bits in the netmask, and if missing, is assumed to be 32 for IPv4 and 128 for IPv6 so that it represents a single host.

The `cidr` type _only_ accepts network addresses, not hosts. In other words, `inet` accepts values with non-zero bits to the right of the netmask, while `cidr` does not.

## Bit String Types

Bit strings are binary strings that can be used to store or visualize bit masks. The two bit string types are `bit(n)` and `bit varying(n)`.

The `bit` type data must match the length `n` exactly; it cannot be shorter or longer. `bit varying` type data can be shorter than `n`, but not longer.

Writing `bit` without a length `(n)` implies a length of 1, i.e. `bit(1)`.

Writing `bit varying` without a length `(n)` implies an unlimited length.

## Text Search Types

A `tsvector` is a sorted list of distinct lexemes: words that have been normalized to merge different variants of the same word. Note that this normalization is _not_ performed by `tsvector`, but can be done by functions such as `to_tsvector()`.

Integer positions can be attached to lexemes. Positions normally indicate the source word's location in the document, which can then be used for proximity ranking.

Lexemes can further be labeled with a weight of `A` through `D`, where `D` is the default weight. Weights typically reflect the document structure, such as to distinguish between title and body words.

A `tsquery` stores lexemes that are to be searched for. The contained lexemes can be combined using Boolean operators and the phrase search (FOLLOWED BY) operators `<->` and its variant `<N>` which takes the distance `N` between the two lexemes being searched for. The `<->` operator is equivalent to `<1>`.

As with `tsvector`, the `tsquery` type expects and does _not_ perform normalization of words, but can be done by functions such as `to_tsquery()`.

Lexemes in a `tsquery` can be labeled with one or more weight letters, restricting them to only matching on lexemes in the `tsvector` with one of those weights.

Lexemes can be labeled with `*` to specify prefix matching, so that the query matches any word in the `tsvector` beginning with that lexeme.

``` postgresql
-- 'postgraduate' is stemmed to 'postgradu'
-- 'postgres' is stemmed to `postgr`
-- 'postgr' matches the beginning of 'postgradu'
SELECT to_tsvector('postgraduate') @@ to_tsquery('postgres:*');
```

## UUID Type

The `uuid` type stores Universally Unique Identifiers (UUID) as per RFC 4122.

While PostgreSQL supports storage and comparison of UUIDs, it doesn't support generating UUIDs because no single algorithm is well suited for every application. The `uuid-ossp` module implements standard algorithms, and the `pgcrypto` module provides a generation function for random UUIDs.

## XML Type

The `xml` type can store XML data, with the advantage over plain `text` being that it can check the input values for well-formedness and support functions and type-safe operations. The function `xmlparse()` can be used to parse character data into an `xml` value. The reverse can be achieved with the `xmlserialize()` function.

When using the normal text mode of communication between client and server, since PostgreSQL converts all character data passed between the client and server and vice versa to the character encoding of the receiving end, the encoding declarations contained in the XML data could become invalid, and so they are ignored. It is therefore the responsibility of the client to convert documents to the current client encoding before sending them, or to adjust the client encoding.

When using the binary mode of communication between client and server, no encoding conversion is performed, so the encoding declaration in the XML data is observed and assumed to be UTF-8 if missing.

Since there is no universally useful comparison function for arbitrary XML data, there are no comparison operators defined for the `xml` type, meaning that it's not possible to retrieve rows by comparing an `xml` column against a search value. Consequently this means that it's not possible to create an index directly on a column of type `xml`. Alternatively it's possible to index an XPath expression.

## JSON Types

The `json` type stores an exact copy of the input text, which processing functions must reparse on each call. The `jsonb` type stores a decomposed binary format which is slightly slower to input but significantly faster to process, as no reparsing is necessary on each call. The `jsonb` type also supports indexing. The `jsonb` type is generally preferred over `json`.

Since the `json` type stores the input text verbatim, key order is preserved and duplicate keys are unaffected, and only the final occurrence takes effect when it is actually parsed. Since the `jsonb` type performs the parsing upfront, key order is not preserved, and only the final occurrence of a duplicate key is preserved.

The process of converting JSON input into `jsonb` necessitates mapping values of primitive types onto native PostgreSQL types. As a result, there are additional constraints on what constitutes valid `jsonb`.

| JSON      | PostgreSQL | Notes                             |
| :--       | :--        | :--                               |
| `string`  | `text`     | `\u0000` disallowed               |
| `number`  | `numeric`  | `NaN` and `infinity` disallowed   |
| `boolean` | `boolean`  | only lowercase `true` and `false` |
| `null`    | N/A        | SQL `NULL` is a different concept |

On input, object keys must always be quoted strings.

``` postgresql
SELECT '{"bar": "baz", "balance": 7.77, "active": false}'::json;
```

Updating JSON documents requires a row-level lock on the whole row, so their size should be minimized to decrease lock contention among updating transactions.

The `jsonb` type supports a _containment test_ operation which tests whether one `jsonb` document has contains another one. Generally, the contained object must match the containing object as to structure and data contents, possibly after discarding some non-matching array elements or object key/value pairs from the containing object. The exception is that an array may contain a primitive value.

``` postgresql
-- Scalar value identity:
SELECT '"foo"'::jsonb @> '"foo"'::jsonb;

-- Arrays:
SELECT '[1, 2, 3]'::jsonb @> '[1, 3]'::jsonb;

-- Order is irrelevant:
SELECT '[1, 2, 3]'::jsonb @> '[3, 1]'::jsonb;

-- Duplicate elements are irrelevant:
SELECT '[1, 2, 3]'::jsonb @> '[1, 2, 2]'::jsonb;

-- Array may contain a primitive value:
SELECT '["foo", "bar"]'::jsonb @> '"bar"'::jsonb;

-- Objects:
SELECT '{"product": "PostgreSQL", "version": 9.4, "jsonb": true}'::jsonb
         @> '{"version": 9.4}'::jsonb;

-- It must match the structure:
SELECT '[1, 2, [1, 3]]'::jsonb @> '[1, 3]'::jsonb;  -- yields false

-- But with a layer of nesting, it is contained:
SELECT '[1, 2, [1, 3]]'::jsonb @> '[[1, 3]]'::jsonb;

-- Similarly, containment is not reported here:
SELECT '{"foo": {"bar": "baz"}}'::jsonb @> '{"bar": "baz"}'::jsonb;  -- yields false

-- A top-level key and an empty object is contained:
SELECT '{"foo": {"bar": "baz"}}'::jsonb @> '{"foo": {}}'::jsonb;
```

The _existence_ operator is a variation of containment, testing whether a string appears as an object key or array element at the top level of the `jsonb` value.

``` postgresql
SELECT '"foo"'::jsonb ? 'foo';

SELECT '["foo", "bar", "baz"]'::jsonb ? 'bar';

SELECT '{"foo": "bar"}'::jsonb ? 'foo';

-- As with containment, existence must match at the top level:
SELECT '{"foo": {"bar": "baz"}}'::jsonb ? 'bar'; -- yields false
```

## Type Casts

PostgreSQL supports two equivalent syntaxes for type casts. The `CAST` syntax conforms to the SQL standard, whereas the `::` is historical PostgreSQL syntax.

``` postgresql
CAST ( expression AS target_type )

expression::target_type
```

A cast of a value expression of a known type represents a run-time type conversion which will only succeed if the corresponding type conversion operation has been defined. This is _different_ from a "cast" of a constant, which represents the initial assignment of a type and so will succeed for any type as long as the string literal is acceptable input for the target type.

Target types can sometimes be inferred and explicit type casts omitted, such as when assigning to a table column, in which case the system automatically applies an implicit type cast. Note that this is _only_ done for certain types for which system catalogs know this to be an OK operation.

A third type cast syntax is the function-like syntax. Naturally this only works for types whose names are valid as function names. An example of a type whose name is not a valid function name is `double precision`. Function-like type cast syntax should be avoided due to this inconsistency.

Note that function-like syntax is literally a direct invocation of the registered, underlying conversion function, which by convention has the same name as the output type.

``` postgresql
target_type ( expression )
```

## Arrays

An array constructor uses brackets `[]` and an `ARRAY` prefix. The array element type is the common type of the member expressions, in a manner similar to `UNION` and `CASE`, unless the constructor is explicitly cast, which has the same effect as casting each individual element expression.

Note that array indices begin at `1`.

``` postgresql
ARRAY[1, 2, 3]

ARRAY[1, 2, 22.7]::integer[]; -- {1, 2, 23}
```

Arrays can be nested to produce multidimensional arrays. Interior arrays may omit the `ARRAY` prefix. Note that multidimensional arrays _must_ be rectangular (i.e. they can't be jagged), so all interior arrays at the same level must have the same dimension. Outer casts propagate to inner constructors.

Interior array elements may be any expression that yields an array.

Empty arrays must be explicitly cast to the desired type, since it's impossible to have an array of no type.

``` postgresql
SELECT ARRAY[ARRAY[1, 2], ARRAY[3, 4]];
-- {{1, 2}, {3, 4}}

-- Equivalent:
SELECT ARRAY[[1, 2], [3, 4]];

SELECT ARRAY[]::integer[];
```

It's possible to build an array from the results of a subquery by placing it within the array constructor _without_ the brackets `[]`. The subquery _must_ return a single column.

``` postgresql
SELECT ARRAY(SELECT oid FROM pg_proc WHERE proname LIKE 'bytea%');
-- {2011, 1954, …}
```

PostgreSQL allows columns of a table to be defined as variable-length multidimensional arrays of any built-in or user-defined base type, enum type, or composite type.

Although it's possible to specify the exact size of an array, the current implementation ignores any size limits. The current implementation also does not enforce the declared number of dimensions. Arrays of a given type are all considered to be the same type, regardless of dimension or size. Specifying the dimensions and their size in `CREATE TABLE` is pure documentation.

``` postgresql
CREATE TABLE sal_emp (
  name           text,
  pay_by_quarter integer[],
  schedule       text[][]
);
```

It's also possible to use the SQL standard syntax specifying the type followed by `ARRAY`.

``` postgresql
pay_by_quarter integer ARRAY[4],
```

Array values can be input as literal constants by enclosing the comma delimited values in curly braces `{}`.

Note, however, that the `ARRAY` constructor syntax is often easier to work with than the array-literal syntax, since element values can be written the same way they would be written when not members of an array.

``` postgresql
'{ val1 , val2 , … }'
```

By default, such arrays are one-based unless the array subscript ranges are explicitly written before the array contents.

``` postgresql
'[1:1][-2:-1][3:5]={{{1,2,3},{4,5,6}}}'
```

A particular element can be set to `NULL`.

Although an array's size and dimension are ignored, literal constant inputs must have uniform extents for each dimension.

``` postgresql
INSERT INTO sal_emp
  VALUES ('Bill',
    '{10000, 10000, 10000, 10000}',
    '{{"meeting", "lunch"}, {"meeting"}}');

-- ERROR: multidimensional arrays must have array expressions with matching dimensions
```

It's possible to access arbitrary rectangular slices of an array or subarrays using the `lower:upper` syntax for one or more dimensions. If any dimension is written as a slice then all dimensions are treated as slices. A dimension with a single number and no colon is treated as being `1:n`, so to obtain a single element, it must be repeated as `n:n`. It's a good practice to use explicit slices on every dimension if even a single slice is used.

``` postgresql
SELECT schedule[1:2][1:1] FROM sal_emp WHERE name = 'Bill';
```

If a slice bound is omitted, it's assumed to be the corresponding extent of the array.

A subscript expression returns `NULL` if either the array itself or any of the subscript expressions are `NULL`, or if the subscript is outside of the array bounds.

A slice expression yields `NULL` if either the array itself or any of the subscript expressions are `NULL`. However, when slicing completely outside of the array bounds, a slice expression yields an empty, zero-dimensional array instead of `NULL`. If a slice only partially overlaps the array bounds, it is silently reduced to just the overlapping region instead of returning `NULL`.

An array's dimensions can be obtained as text with the `array_dims()` function or as integers with the `array_lower()` and `array_upper()` functions.

The `array_length()` function returns the length of the specified array dimension.

The `cardinality()` function returns the total number of elements in an array across all dimensions.

Array values can be replaced/overwritten completely, or a single element or slice can be updated.

Arrays can be enlarged by assigning past array bounds. Any previously non-existent elements in between are filled with `NULL`.

Subscripted slice assignment allows the creation of an array that does not use one-based subscripts, e.g. assigning to `somearray[-2:7]` would create an array with subscript values from -2 to 7.

The concatenation operator `||` can be used to concatenate two arrays, resulting in a new array. The concatenation operator can also be used to push a single element at the beginning or end of an array of one dimension higher than the element. The array's lower-bound remains the same.

The functions `array_prepend()`, `array_append()`, and `array_cat()` can also be used to construct new arrays.

The `ANY` operator can be used to test if any element in an array satisfies the given condition:

``` postgresql
SELECT * FROM sal_emp WHERE 10000 = ANY (pay_by_quarter);
```

The `ALL` operator can be used to test if all elements in an array satisfy the given condition:

``` postgresql
SELECT * FROM sal_emp WHERE 10000 = ALL (pay_by_quarter);
```

The `&&` operator checks whether the left operand overlaps with the right operand.

The `array_position()` and `array_positions()` functions return the subscript of the first occurrence or all occurrences, respectively.

## Composite Types

A _composite type_ represents the structure of a row: essentially a list of field names and their data types. A column of a table can be declared to be of composite type.

The syntax `CREATE TYPE … AS` to create a composite type is similar to `CREATE TABLE` except that only field names and their types can be specified, not constraints. The `AS` component is crucial to differentiate it from a typical `CREATE TYPE` command.

``` postgresql
CREATE TYPE complex AS (
  r double precision,
  i double precision
);

CREATE TYPE inventory_item AS (
  name        text,
  supplier_id integer,
  price       numeric
);
```

Whenever a table is created, a composite type is also automatically created with the same name as the table, to represent its row type. However, since no constraints are associated with a composite type, any constraints associated with a table definition _do not_ apply to values of the composite type outside of the table.

Composite values can be constructed via literal constants, with each field value optionally (or necessarily, if it contains commas or parentheses) enclosed in double quotes. A field can be given a `NULL` value by omitting its value position in the list of values.

Interior whitespace is considered to be part of the field, and may or may not be significant depending on the field's type.

``` postgresql
'(val1,val2,…)'

'("fuzzy dice",42,1.99)'

-- Final field receives NULL
'("fuzzy dice",42,1.99,)'

-- Final field receives empty string
'("fuzzy dice",42,1.99,"")'
```

A row constructor is an expression that builds a "row", or _composite value_. This consists of parenthesizing the fields and using a `ROW` prefix. The prefix is optional if there's more than one field. The type of this row is anonymous unless cast to a named composite type: either the row type of a table or one created with `CREATE TYPE … AS`.

``` postgresql
SELECT ROW(1, 2.5, 'this is a test');
```

Row constructors are typically used for storing values in composite-type table columns or passing composite arguments to functions. They may be preferable to composite type literal constants as they don't require multiple levels of quoting. In fact, the `ROW` keyword is optional as long as there is more than one field in the expression.

``` postgresql
ROW('fuzzy dice',42,1.99,NULL)

-- Equivalent:
('fuzzy dice',42,1.99,NULL)
```

Rows can be compared and tested for being `NULL`.

The dot operator can be used to access a field of a composite column. The column name should be enclosed in parentheses to prevent the parser from assuming that it is a table name.

``` postgresql
-- Incorrect; assumes `item` is a table name.
SELECT item.name FROM on_hand WHERE item.price > 9.99;

-- Correct
SELECT (item).name FROM on_hand WHERE (item).price > 9.99;

-- Qualified table name
SELECT (on_hand.item).name FROM on_hand WHERE (on_hand.item).price > 9.99;

SELECT (my_func(…)).field FROM …;
```

It's possible to overwrite an entire composite column value or update an individual field of a composite column. It's not necessary to enclose the column name in parentheses right after the `SET`, but it is necessary on the right-hand side of the equal sign.

``` postgresql
INSERT INTO mytab (complex_col) VALUES((1.1,2.2));
UPDATE mytab SET complex_col = ROW(1.1,2.2) WHERE …;

INSERT INTO mytab (complex_col.r, complex_col.i) VALUES(1.1, 2.2);
UPDATE mytab SET complex_col.r = (complex_col).r + 1 WHERE …;
```

The `.*` syntax may be used to expand an element row expression into fields of the row being constructed. In other words, if table `t` has fields `f1` and `f2`, this is possible:

``` postgresql
SELECT ROW(t.*, 42) FROM t;

-- Equivalent to:
SELECT ROW(t.f1, t.f2, 42) FROM t;

-- To get a row whose first field is itself a row:
SELECT ROW(t, 42) FROM t;
```

In PostgreSQL, a reference to a table name in a query is effectively a reference to the composite value of the table's current row.

``` postgresql
SELECT c FROM inventory_item c;
-- => ("fuzzy dice",42,1.99)
```

Selecting all fields of a composite-valued expression with `.*` expands to field-selecting all fields. If the composite-valued expression is a function call yielding a composite value, then the function will be called once for each field.

``` postgresql
SELECT (myfunc(x)).* FROM some_table;
SELECT (myfunc(x)).a, (myfunc(x)).b, (myfunc(x)).c FROM some_table;

-- To call `myfunc()` once only.
-- `OFFSET 0` prevents optimizer from flattening sub-select
-- to the traditional naive expansion.
SELECT (m).* FROM (SELECT myfunc(x) AS m FROM some_table OFFSET 0) ss;
```

This expansion only applies at the top level of a `SELECT `output list, a `RETURNING` list on `INSERT`, `UPDATE`, or `DELETE`, a `VALUES` clause, or a row constructor. In all other contexts, attaching `.*` to a composite value doesn't change anything.

``` postgresql
-- Equivalent:
SELECT somefunc(c.*) FROM inventory_item c;
SELECT somefunc(c) FROM inventory_item c;

-- Equivalent:
SELECT * FROM inventory_item c ORDER BY c;
SELECT * FROM inventory_item c ORDER BY c.*;
SELECT * FROM inventory_item c ORDER BY ROW(c.*);
```

Field selection is also possible through _functional notation_, where `field(table)` is equivalent to `table.field`.

``` postgresql
SELECT c.name FROM inventory_item c WHERE c.price > 1000;
SELECT name(c) FROM inventory_item c WHERE price(c) > 1000;
```

The reverse of functional notation is also possible: a function that accepts a single argument of composite type can be called with either notation. This can be used to implement computed fields. Care should be taken to avoid giving a function with a single composite-type argument the same name as any of that composite type's fields. In that case, function interpretation can be forced by schema-qualifying the function name.

``` postgresql
SELECT somefunc(c) FROM inventory_item c;
SELECT somefunc(c.*) FROM inventory_item c;
SELECT c.somefunc FROM inventory_item c;
```

## Range Types

| Name        | Description          |
| :---------  | :----------          |
| `int4range` | integer              |
| `int8range` | bigint               |
| `numrange`  | numeric              |
| `tsrange`   | timestamp without tz |
| `tstzrange` | timestamp with tz    |
| `daterange` | date                 |

Range types represent a range of values of some element type, known as the range's subtype, e.g. ranges of timestamps. The subtype must have a total order. There are some built-in range types, and others can be created with `CREATE TYPE`.

``` postgresql
-- Containment
SELECT int4range(10, 20) @> 3;

-- Overlaps
SELECT numrange(11.1, 22.2) && numrange(20.0, 30.0);

-- Extract the upper bound
SELECT upper(int8range(15, 25));

-- Compute the intersection
SELECT int4range(10, 20) * int4range(15, 25);

-- Is the range empty?
SELECT isempty(numrange(1, 5));
```

Each range type has a constructor function with the same name as the range type, which makes it unnecessary to perform extra quoting of the bound values. A three-argument constructor form exists which takes the type of bounds to use for the range, e.g. `"[)"`. Using `NULL` for either range bound causes that side to be unbounded.

A _discrete range_ is one whose element type has a well-defined step, a clear idea of a next or previous value, such as with integer or date dates and unlike with _continuous ranges_ between numeric or timestamp types.

A discrete range type should have a _canonicalization_ function that is aware of the desired step size for the element type, and handles the conversion of equivalent values of the range type to have identical representations. The built-in discrete range types all use a canonical form of range type `[)`. For example, `(1, 14]` would be represented as its canonical form `[2, 15)`.

Constraints on ranges can be created, typically in the form of exclusion constraints such as "non-overlapping."

``` postgresql
CREATE TABLE reservation (
  during tsrange,
  EXCLUDE USING GIST (during WITH &&)
);
```

## Object Identifier Types

Object identifiers (OIDs) of type `oid` are used internally by PostgreSQL as primary keys for various system tables. There are several alias types for `oid`, such as `regrole`, which don't provide any operations of their own except for specialized input and output routines which are able to accept and display symbolic names for system objects rather than the raw numeric value, which allows for simplified lookup of OID values.

``` postgresql
-- Raw OID lookup:
SELECT * FROM pg_attribute
  WHERE attrelid = (SELECT oid FROM pg_class WHERE relname = 'mytable');

-- Symbolic lookup through regclass alias type:
SELECT * FROM pg_attribute WHERE attrelid = 'mytable'::regclass;
```

Most OID alias types also create dependencies, so that if a constant of one of those types appears in a stored expression (e.g. column default expression, or view), a dependency is created on the referenced object. Specifically, if a column has a default expression of `nextval('my_seq'::regclass)`, PostgreSQL recognizes that the default expression depends on the sequence `my_seq`, preventing it from being dropped without first removing the default expression.

A tuple identifier (tid; row identifier) is a pair of (block number, tuple index within block) which identifies the physical location of a row within its table.

## Pseudo-Types

A pseudo-type cannot be used as a column data type, but can otherwise be used to declare a function's argument or result type. For example, there is `any`, `anyarray`, `anynonarary`, `anyenum`, `anyrange`, `record` for an unspecified row type, etc.

# Functions and Operators

SQL uses a three-valued logic system with `true`, `false`, and `null`.

| a    | b    | a AND b | a OR b |
| :--  | :--  | :--     | :--    |
| T    | T    | T       | T      |
| T    | F    | F       | T      |
| T    | NULL | NULL    | T      |
| F    | F    | F       | F      |
| F    | NULL | F       | NULL   |
| NULL | NULL | NULL    | NULL   |

| a    | NOT a |
| :--  | :--   |
| T    | F     |
| F    | T     |
| NULL | NULL  |

Schema-qualified operators can be written by using the `OPERATOR` keyword. Note that the effective operator's precedence is the same regardless of the precedence of the operator passed as the argument.

``` postgresql
SELECT 3 OPERATOR(pg_catalog.+) 4;
```

## Comparison Operators

There are two "not equal" comparison operators, `<>` and `!=`, with the `!=` operator being converted to `<>` in the parser stage, so it's not possible to implement each one to do different things.

Ordinary comparison operators yield `NULL` when either input is `NULL`. This can be interpreted as, `NULL` represents an unknown value, and the comparison result of two unknown values, or a known and unknown value, is unknown.

However, comparison predicates effectively act as though `NULL` were a normal data value rather than "unknown". The `IS DISTINCT` predicate for example returns `false` if both inputs are `NULL` and `true` when only one is. Likewise, `IS NOT DISTINCT FROM` returns `true` when both inputs are `NULL` and false when only one is.

For `IS NULL`, if the expression is row-valued, the result is `true` when the row expression is itself `NULL` or when _all_ of the row's fields are `NULL`, while `IS NOT NULL` yields `true` when the row expression is itself non-`NULL` and _all_ of the row's fields are non-null. This means that a row-valued expression with a mix of `NULL` and non-`NULL` fields returns `false` for both tests. For this reason, it may be preferable in certain cases to use `IS DISTINCT FROM NULL`, which would only check if the row value itself is `NULL` without checking its fields.

The `IS UNKNOWN` predicates are effectively the same as `IS NULL`, treating a `NULL` input as logical "unknown," except that the input expression _must_ be of Boolean type.

``` postgresql
-- inclusive: a >= x AND a <= y
a BETWEEN x AND y

-- negation: a < x OR a > y
a NOT BETWEEN x AND y

-- as above, after sorting the comparison values
a BETWEEN SYMMETRIC x AND y
a NOT BETWEEN SYMMETRIC x AND y

-- not equal (treats null as ordinary value)
a IS DISTINCT FROM b
a IS NOT DISTINCT FROM b

expr IS NULL
exprt IS NOT NULL

bool_expr IS TRUE
bool_expr IS NOT TRUE

bool_expr IS FALSE
bool_expr IS NOT FALSE

bool_expr IS UNKNOWN
bool_expr IS NOT UNKNOWN
```

## Mathematical Operators

Some of the mathematical operators even include the square root operator `|/`, cube root operator `||/`, factorial operator `!`, exponentiation `^`.

## String Functions and Operators

`regexp_match` returns the captured substrings matching a POSIX regular expression.

`regexp_split_to_array` and `regexp_split_to_table` split a string using a POSIX regular expression.

`split_part` splits a string on a given delimiter and returns the specified piece.

`format` is similar to the C function `sprintf`.

`concat` will concatenate all arguments, ignoring those that are `NULL`.

`concat_ws` will concatenate all arguments using the separator specified as the first argument.

`concat`, `concat_ws`, and `format` are variadic functions, so arrays can use the `VARIADIC` keyword to pass all elements are function arguments.

``` postgresql
format('You are %s years old', 20) -- 'You are 20 years old'
concat_ws(' ', 'john', 'smith', 'doe') -- 'john smith doe'
concat_ws(', ', VARIADIC ARRAY['one', 'two']) -- 'one, two'
```

## Binary String Functions and Operators

`octet_length` counts the number of bytes in a `BYTEA` binary string.

`get_bit` and `get_byte` can be used to obtain the corresponding bit or byte. There are also `set_bit` and `set_byte`. Note that unlike 1-based array indexing, these functions are 0-based.

## Bit String Functions and Operators

The operands of `&`, `|`, and `#` must be of equal length. Bit shifting preserves the original length of the string.

## Pattern Matching

Pattern matching can be done with `LIKE`, `SIMILAR TO`, or POSIX regular expressions. Pattern matching that goes beyond this can be implemented with a user-defined function written in Perl (for PCRE).

Note that regular expressions can take arbitrary time to execute, so they shouldn't be accepted from users and if they are then a statement timeout should be imposed. The same goes for `SIMILAR TO`.

### LIKE

In a `LIKE` expression, `_` stands for any character (like `.` in regular expressions) and `%` stands for any sequence of zero or more characters (like `.*` in regular expressions). These can be escaped with the escape character which is `\` by default unless selected with the `ESCAPE` clause. To match the escape character itself, write it twice.

A `LIKE` expression covers the entire string, so a pattern without `%` or `_` is treated like string equality.

`ILIKE` is a case-insensitive `LIKE`, based on the active locale.

There are some aliases:

| Operator | Keyword |
| :------- | :------ |
| `~~`     | `LIKE`  |
| `!~~`    | `NOT LIKE` |
| `~~*`    | `ILIKE` |
| `!~~*`   | `NOT ILIKE` |

### SIMILAR TO

The `SIMILAR TO` operator is similar to `LIKE` except that it uses the SQL standard's definition of a regular expression. It also uses `%` and `_` from `LIKE`, as well as `|`, `*`, `+`, `?`, `{m}`, `{m,}`, `{m,n}`, `()`, and `[…]`. Note that instead of the regular expression period `.`, the `_` character is used.

`substring` can extract a string matched by a `SIMILAR TO` pattern. The part of the string to be extracted should be surrounded by the escape character followed by a double quote.

``` postgresql
substring('foobar' FROM '%#"o_b#"%' FOR '#') -- => oob
```

### POSIX Regular Expressions

Unlike `LIKE` and `SIMILAR TO` patterns, POSIX Regular Expressions can match any part of the string, unless explicitly anchored.

There are some operators for matching on POSIX Regular Expressions.

| Operator | Purpose |
| :------  | :------ |
| `~`      | case-sensitive match |
| `~*`     | case-insensitive match |
| `!~`     | case-sensitive negative match |
| `!~*`    | case-insensitive negative match |

`substring` works with POSIX Regular Expressions but it returns the portion of the text that matched, or the first parenthetical group if any exists. The whole pattern can be parenthesized to avoid this behavior, or non-capturing parentheses can be used.

`regexp_replace` can substitute matches of a string. The replacement string can contain back-references to capture groups denoted with `\n`, with `\&` denoting the entire match should be inserted. It takes an optional flags parameter denoting e.g. case-insensitivity with `i`, global matching with `g`, etc.

`regexp_matches` returns an array of all captured groups. If the `g` flag was used then the result will be a row of such an array for each match. If there are no capture groups in the pattern then each row is a single-element text array consisting of the substring matching the whole pattern.

`regexp_split_to_table` splits a string with a given pattern delimiter into rows.

`regexp_split_to_array` is similar except it returns an array of text instead.

PostgreSQL regular expressions (Advanced Regular Expressions (AREs)) can begin with an embedded options sequence of the form `(?xyz)`.

The `x` option can be used in regular expressions to enable expanded syntax where whitespace is not significant and where comments are allowed. A whitespace character or `#` comment delimiter can be treated literally by escaping it. Comments can also occur inline with the form `(?#text)`.

## Data Type Formatting

`to_char` converts to `TEXT`.

`to_date` converts `TEXT` TO `DATE`.

`to_number` converts `TEXT` to `NUMERIC`.

`to_timestamp` converts `TEXT` to `TIMESTAMP`.

## Date and Time Functions and Operators

`INTERVAL`s, `DATE`s, and `TIME`s can be added and doing so has the expected effect.

``` postgresql
DATE '2001-09-28' + INTERVAL '1 hour' -- TIMESTAMP '2001-09-28 01:00:00'
21 * INTERVAL '1 day' -- INTERVAL '21 days'
```

`age` subtracts two timestamps and produces a written-out description of the result. When only one argument is given, it uses the `current_date` as the other.

``` postgresql
age(timestamp '2001-04-10', timestamp '1957-06-13') -- 43 years 9 mons 27 days
```

`current_timestamp` gets the current date and time <span class="highlight">at the start of the transaction</span>.

`clock_timestamp` gets the current date and time, this changes during statement execution.

`to_timestamp` converts a Unix epoch to a `TIMESTAMP`.

`make_date`, `make_interval`, `make_time`, `make_timestamp`, and `make_timestampz` can be used to create the corresponding type from component types.

``` postgresql
make_interval(days => 10) -- INTERVAL '10 days'
```

`extract` gets an arbitrary field from a given `TIMESTAMP`, `TIME`, or `INTERVAL`. `DATE`s are cast to `TIMESTAMP`. The possible field names are:

* `CENTURY`
* `DAY`
* `DECADE`
* `DOW` --- day of the week, Sunday is 0
* `DOY` --- day of the year
* `EPOCH`
* `HOUR`
* `ISODOW` --- day of the week per ISO 8601 numbering, Monday is 0
* `ISOYEAR` --- week-numbering year per ISO 8601
* `MICROSECONDS`
* `MILLENNIUM`
* `MILLISECONDS`
* `MINUTE`
* `MONTH`
* `QUARTER` --- quarter of the year
* `SECOND`
* `TIMEZONE` --- UTC offset
* `TIMEZONE_HOUR`
* `TIMEZONE_MINUTE`
* `WEEK`
* `YEAR`

`date_trunc` truncates the `TIMESTAMP` or `INTERVAL` such that any field less significant than the specified field is set to zero.

The `OVERLAPS` operator checks if two time periods, defined by their endpoints, overlap. The endpoints can pairs of dates, times, or timestamps; or one of those plus an interval as the end-point.

The time period is interpreted as the half-open interval `$\left[\text{start}, \text{end}\right)$` unless both times are the same, then it is considered an instant in time. This means that two periods with only an end-point in common will _not_ be considered as overlapping.

Note that adding an interval to a `TIMESTAMPZ`, the `days` component ignores timezone changes such as daylight saving time, while the `hours` component obeys it, so that adding `'24 hours'` may end up adding 25 when done across a timezone change, while adding `'1 day'` will just.

``` postgresql
(start1, end1) OVERLAPS (start2, end2)
(start1, length1) OVERLAPS (start2, length2)
```

The `AT TIME ZONE` construct can be used to convert timestamps to different time zones, or to give a `TIMESTAMP WITHOUT TIME ZONE` a time zone. The time zone can be expressed as a text string like `PST` or an interval like `INTERVAL '-08:00'`.

Some time functions return a time from the start of the current transaction. This is deliberate so that the entire transaction can have a consistent view of time. The functions are:

* `CURRENT_TIME`
* `CURRENT_TIMESTAMP`
* `LOCALTIME`
* `LOCAL_TIMESTAMP`

PostgreSQL additionally provides functions which give the time at other points in time:

* `transaction_timestamp()` --- synonymous with `CURRENT_TIMESTAMP` but more explicit
* `statement_timestamp()` --- start time of the current statement
* `clock_timestamp()` --- actual current time which may vary within a single SQL command
* `timeofday()` --- same as `clock_timestamp()` but formatted as a string
* `now()` --- equivalent to `transaction_timestamp()`

All date/time data types also accept the special literal `now` to specify the current date and time, which is interpreted as the transaction start time. However, **do not** use this in a `DEFAULT` clause when creating a table, since the system will convert it to a timestamp as soon as it is parsed, so the default value will actually take on the value of the start of the transaction that created the table.

## Geometric Functions and Operators

The "same as" operator `~=` represents equality for point, box, polygon, and circle types. Some of those types also have a `=` operator but it compares for equal _areas_ only, just like `<=` and other scalar comparison operators.

Translation is done by adding or subtracting a `POINT`.

Scaling and rotation is done by multiplying by a `POINT`.

Note that [PostGIS](#postgis) has other geometric functions.

## JSON Functions and Operators

`->` gets an array element indexed at 0 (negative indices wrap around), or an object field by key. `->>` does the same but gets the result as `TEXT`.

`#>` gets the JSON object at the specified path specified as an array of `TEXT` with an element for each path component. `#>>` does the same but gets the result as `TEXT`. If the path doesn't exist the result is `NULL`.

``` postgresql
'{"a": {"b":{"c": "foo"}}}'::json #> '{a,b}' -- => {"c": "foo"}

'{"a":[1,2,3],"b":[4,5,6]}'::json #>> '{a,2}' -- => 3
```

There are variants of these operators for the `JSONB` type as well as for `JSON`.

Comparison operators are only available for `JSONB`.

There are also some `JSONB`-only operators:

`@>` is the general containment operator and works as expected with `JSON`B types, testing whether the left value contains the right path or value entries at the _top level_. `<@` is the same but with the operands flipped.

`?` checks if the right-operand string exists as a top-level key in the left `JSONB` value. `?|` is similar but checks if _any_ of the string keys in an array are present, while `?&` checks if _all_ of them exist.

`||` merges two `JSONB` values. Note that this is a shallow, top-level merge. It is _not_ recursive.

`-` deletes the key/value pair or key string from the left operand, with key/value pairs matched by their key. It also accepts an array of `TEXT` key/value pairs or string keys. It also accepts integer indices to remove elements from an array.

`#-` is like `#>` except it deletes the value at the path.

`to_json` and `to_jsonb` converts the value to `JSON` or `JSONB`. This is done recursively, using casts if available. There are also `array_to_json` and `row_to_json`, but they are the same except for these offering a pretty-printing option.

`json_build_object` takes a variadic argument list of alternating key-value arguments.

`json_each` expands the outer-most object into rows of key-value pairs. There is a `json_each_text` variant that converts each value to `TEXT`.

`json_populate_record` maps each top-level key-value pair to matching columns of a given row type and produces one row of that type. JSON fields that don't appear in the target row type are omitted, and fields that don't exist for a corresponding target row type column will be `NULL`. This function is typically used in the `FROM` clause with the JSON column. `json_populate_recordset` does the same but for an array of objects, producing multiple rows for each object.

`json_to_record` and `json_to_recordset` are similar except they can build an arbitrary record (or records) from a JSON object (or array of objects), so the caller must explicitly define the structure with an `AS` clause.

``` postgresql
SELECT json_field_1, json_field_2
FROM json_populate_record(NULL::myrowtype, table.json_column);

SELECT json_field_1, json_field_2
FROM json_to_record(
  '{"a":1,"b":[1,2,3],"c":[1,2,3],"e":"bar","r": {"a": 123, "b": "a b c"}}'
  ) AS x(a INT, b TEXT, c INT[], d TEXT, r myrowtype);
```

`json_array_elements` is similar but it expands a JSON array to rows of values. The `json_array_elements_text` variant encodes each value as `TEXT`.

`json_typeof` returns the type of the outermost JSON value as a `TEXT` string, which can be `object`, `array`, `string`, `number`, `boolean`, or `null`.

`json_strip_nulls` removes all _object_ fields that have null values.

`jsonb_set` returns the JSON value with the value at the specified path replaced with a new value, or the value is added unless the `create_missing` parameter is set to `FALSE`. However, every value in the path except the final component must already exist, that is, only the target would be added.

``` postgresql
jsonb_set('[{"f1":1,"f2":null},2,null,3]', -- JSON object
          '{0,f1}',                        -- target path
          '[2,3,4]',                       -- new value
          FALSE)                           -- don't create if missing
-- [{"f1":[2,3,4],"f2":null},2,null,3]

-- create_missing is TRUE by default
jsonb_set('[{"f1":1,"f2":null},2]', '{0,f3}','[2,3,4]')
-- [{"f1": 1, "f2": null, "f3": [2, 3, 4]}, 2]
```

`jsonb_insert` can be used to insert into JSON arrays. The value is inserted before the target unless the `insert_after` argument is `TRUE`. In JSON objects, the value is only inserted if it doesn't already exist.

``` postgresql
-- insert_after is FALSE by default
jsonb_insert('{"a": [0,1,2]}', '{a, 1}', '"new_value"')
-- {"a": [0, "new_value", 1, 2]}

jsonb_insert('{"a": [0,1,2]}', '{a, 1}', '"new_value"', TRUE)
-- {"a": [0, 1, "new_value", 2]}
```

`jsonb_pretty` can format the given JSON value to be human readable.

## Sequence Manipulation Functions

Sequence objects (AKA _sequence generators_ AKA _sequences_) are single-row tables typically used to generate unique identifiers for rows of a table.

`currval` returns the value most recently obtained with `nextval` for the given sequence <span class="highlight">in the current session</span>, while `lastval` does the same for any sequence. This returns a session-local value that is predictable whether or not other sessions have executed `nextval` since the current session did.

`nextval` advances the the given sequence and returns the new value in one atomic operation. `nextval` operations are <span class="highlight">never rolled back</span> to avoid blocking concurrent transactions that obtain numbers from the same sequence. This means that any one returned value will be considered used and will never be returned again, even if the surrounding transaction aborts for example. This means that sequence objects cannot produce "gapless" sequences.

`setval` sets the sequence's current value.

The sequence to operate on is specified by a `regclass` which is the `OID` of the sequence in the `pg_class` system catalog, which can be obtained by just writing it as a literal so that the `regclass` data type input converter computes it, i.e. `nextval('somesequence')`.

This is an "early binding" to the OID because it becomes a constant of type `regclass` which is just an `OID`, so when used for sequence references in column defaults and views it will continue to refer to the same sequence even after the sequence has been renamed, schema reassignment, etc. "Late binding" where the reference is resolved at run time can be forced by explicitly coercing the sequence name to `TEXT`.

## Conditional Expressions

### CASE

Note that if the following conditional expressions are too restricting, it's recommended to write a stored procedure in a more expressive language.

`CASE` is similar to `if` statements. It can be used wherever an expression is valid. If there is no `ELSE` clause and no condition is `TRUE` then the result is `NULL`. Note that the data types of all result expressions <span class="highlight">must be convertible to a single output type</span>.

``` postgres
CASE WHEN condition THEN result
  [WHEN …]
  [ELSE result]
END
```

There is another form which computes the expression and compares its value to the value of every `WHEN` clause, similar to a switch statement.

``` postgresql
CASE expression
  WHEN value THEN result
  [WHEN …]
  [ELSE result]
END
```

`CASE` expressions don't evaluate any subexpressions that it doesn't need to evaluate to determine the result.

### COALESCE

`COALESCE` returns the first of its arguments that is not `NULL`, or `NULL` if none are.

`COALESCE` only evaluates the arguments that are needed to evaluate the result.

### NULLIF

`NULLIF` returns `NULL` if both arguments are equal, otherwise the first argument is returned. This is an inverse of `COALESCE`.

``` postgresql
NULLIF(value, '(none)') -- NULL if value = '(none)'
```

### GREATEST and LEAST

`GREATEST` and `LEAST` return the largest or smallest argument provided. The expressions must all be convertible to a common data type which will also be the type of the result.

They return `NULL` if all expressions evaluate to `NULL`.

## Array Functions

Array comparison operators operate pair-wise.

The concatenation operator `||` can be used to concatenate arrays, add an array to a higher dimension array, or prepending or appending an element.

`array_length` can obtain the length of a <span class="highlight">specific dimension</span> of an array.

`array_position` finds the index of the first occurrence of the given target optionally starting from a specific index.

`array_to_string` joins an array with a given delimiter and optionally replaces `NULL` with a given string.

``` postgresql
array_to_string(ARRAY[1, 2, 3, NULL, 5], ',', '*') -- 1,2,3,*,5
```

`string_to_array` performs the reverse action, splitting a string into an array of elements with a specified delimiter and string to treat as `NULL`.

``` postgresql
string_to_array('xx~^~yy~^~zz', '~^~', 'yy') -- {xx,NULL,zz}
```

`unnest` expands an array to a set of rows for each element. Multiple arrays can be provided in which case their elements are mapped to distinct columns.

## Range Functions and Operators

Ranges can be tested for equality, comparison, containment (e.g. `@>`), overlap `&&`, strictly left of `<<` and right of `>>` checks, "does not extend to the left of" `&<` or "right of" `&>` tests, and adjacent tests `-|-`.

Comparison checks compare the lower bounds first and only compare the upper bounds if the lower bounds are equal.

The union of a range can be computed with `+`, the range intersection with `*` (i.e., the range that overlaps both ranges), and the difference with `-`. The union `+` and difference `-` operators fail if the resulting range would need to contain two disjoint sub-ranges.

## Aggregate Functions

Aggregate functions compute a single result from a set of input values.

These functions (except for `count`) return `NULL` when no rows are selected, so e.g. `sum` would return `NULL` not 0, so `COALESCE` may be used in conjunction.

Aggregate functions that support "partial mode" may participate in various optimizations such as parallel aggregation.

`array_agg` produces an array of the argument type.

`json_agg` produces a `JSON` array.

`json_object_agg(name, value, …)` produces an object with the given key-value pairs.

`string_agg(expr, delimiter)` concatenates all input values into a string separated by the given delimiter.

`avg` computes the average of all input values.

`max` computes the maximum expression, while `min` computes the minimum expression.

`sum` computes the sum of the expression across all input rows.

`count` computes the number of input rows for which the value of the expression is not `NULL`, unless `*` is the argument.

`bool_and` is `TRUE` if all input values are `TRUE`, while `bool_or` is `TRUE` if _any_ input value is `TRUE`. `every` is synonymous to `bool_and`.

## Window Functions

Window functions perform calculations across sets of rows that are realted to the query row. Window functions _must_ be invoked with an `OVER` clause. Any built-in or user-defined normal aggregate function can be used as a window function.

## Membership Tests

### EXISTS

`EXISTS` takes an arbitrary subquery that is evaluated to determine whether it returns _any_ rows.

The subquery can refer to variables from the surrounding query, in which case they act as constants during any one evaluation of the subquery. The subquery usually executes long enough to determine whether at least one row is returned. Since row existence is all that matters, a common idiom is to `SELECT 1`. 

`EXISTS` can be used to perform something like an inner join producing _at most_ one output row for each left table row even if there are multiple matching rows in the right table.

``` postgresql
SELECT col
FROM left_table
WHERE EXISTS (SELECT 1
              FROM right_table
              WHERE left_table.col = right_table.col);
```

### IN

An `IN` expression can take a subquery that returns exactly one column. If the left expression is `NULL` or if there are no equal values in the right and at least one right value is `NULL`, then the result is `NULL` _not_ `FALSE`.

``` postgresql
expression IN (subquery)
```

There is another form where the left side is a row constructor, in which case it checks to see if the right hand subquery produces at least one row with the same columns and values. Two rows are only considered equal if all of their corresponding members are non-`NULL` and equal, or otherwise considered unequal if all of their corresponding members are non-`NULL` and unequal. The result of the comparison is `NULL` (unknown) otherwise. If all of the per-row results are unequal or `NULL` with at least one `NULL` then the result is `NULL`.

``` postgresql
row_constructor IN (subquery)
```

There is also a variant that takes a parenthesized list of scalar expressions instead of a subquery. It is a shorthand notation for doing an explicit equality test for each value in the list `OR`ed together.

``` postgresql
number IN (1, 2, 3)

-- Equivalent to:
number = 1 OR number = 2 OR number = 3
```

`NOT IN …` has the inverse effect. If the left expression yields `NULL` or therea re no equal right values and at least one right value is `NULL`, then the result is `NULL`.

There is also a variant of `NOT IN …` that takes a parenthesized list of scalar expressions instead of a subquery. It is a shorthand notation for doing an explicit equality test for each value in the list `OR`ed together.

### ANY and SOME

With `ANY` and `SOME` the right side is a subquery which must return exactly one column and th eleft side is evaluated and compared to each row of the subquery using the given Boolean-yielding operator. `SOME` is a synonym of `ALL`.

Note that `IN` is equivalent to `= ANY`.

``` postgresql
expression operator ANY (subquery)
expression operator SOME (subquery)
```

If there are no successes and there is at least one `NULL` produced by the operator's result, then the result of the `ANY` will be `NULL` not `FALSE`.

There are also row constructor variants in which case the subquery must produce the same columns, and the columns are compared paire-wise with the specified operator.

There is also a variant that takes a parenthesized expression that must yield an array value.

### ALL

The `ALL` construct is similar to `ANY` except that it only returns `TRUE` if _all_ values produced by the subquery return `TRUE` for the given operator against the left expression. There is also a row constructor variant.

`NOT IN` is equivalent to `<> ALL`.

There is also a variant that takes a parenthesized expression that must yield an array value.

### Single-Row Comparison

A row constructor can be used to produce a row compared against a subquery that produces at most one row with the same columns.

``` postgresql
row_constructor operator (subquery)
```

A single row constructor can also be compared to another. Both row values must have the same number of fields, and each element must be of a type which has a default B-tree operator class. Incompatibilities may be averted if the comparison is determined by earlier compatible elements.

For equality operators `=` and `<>`:

* If _all_ corresponding members are non-`NULL` and equal, the rows are equal
* If _any_ corresponding members are non-`NULL` and unequal, the rows are unequal
* Otherwise the comparison result is `NULL` (unknown)

For the other comparison operators, row elements are compared left-to-right, stopping as soon as an unequal or `NULL` pair of elements is found. If either of this pair is `NULL`, the comparison result is `NULL`. Otherwise the comparison of the elements determines the result.

# Collation Expressions

_Collation_ refers to the set of rules that determine how data is compared and sorted. The collation of a particular expression can be overridden using a `COLLATE` clause.

``` postgresql
expr COLLATE the_collation
```

When a collation is omitted, it's derived from the columns involved in the expression, or if no column is involved in the expression then it defaults to the default collation of the database.

A common use of the `COLLATE` clause is to override the sort order in an `ORDER BY` clause.

``` postgresql
SELECT a, b, c
FROM tbl
WHERE …
ORDER BY a COLLATE "C";
```

Another use is overriding the collation of a function or operator that has locale-sensitive results.

``` postgresql
SELECT *
FROM tbl
WHERE a > 'foo' COLLATE "C";
```

Note that even though the `COLLATE` expression above is attached to the `'foo'` argument of the `>` operator when we intend to affect the collation of the `>` operator itself, this doesn't matter because the collation used by operators and functions is derived by considering all arguments, and an explicit` COLLATE` clause overrides the collations of all other arguments. By extension, attaching non-matching `COLLATE` clauses to multiple arguments is an error.

This means, in fact, that the `COLLATE` expression _must_ be attached to an argument, since parenthesizing the operation and attaching it to the parenthesized group would attempt to apply it to the _result_ of the operation, which in this case is of non-collatable data type `boolean`.

# Data Definition

SQL does not guarantee the order of rows in a table; order is imposed when the table is read.

Tables are created with the `CREATE TABLE` command which takes the table name and a list of column names and their types.

``` postgresql
CREATE TABLE products (
  product_no integer,
  name text,
  price numeric
);
```

Tables can be dropped with the `DROP TABLE` command. Attempting to drop a table that doesn't exist is an error. The `DROP TABLE IF EXISTS` variant can be used to silence that error.

``` postgresql
DROP TABLE products;
```

Columns can be assigned default values which are used when the column isn't given an explicit value or when a command requests that the default be used. If the default value is omitted, it is assumed to be `NULL`. Default values appear after the column type, with the `DEFAULT` keyword.

``` postgresql
CREATE TABLE products (
  product_no integer,
  name text,
  price numeric DEFAULT 9.99
);
```

The default value may be any expression which will be evaluated whenever the default value is to be inserted. One common expression to use is `CURRENT_TIMESTAMP` so that a timestamp of the time of row insertion is used. Another common default value expression is to increment a sequence generator, for which the `SERIAL` sugar exists.

``` postgresql
CREATE TABLE products (
  product_no integer DEFAULT nextval('products_product_no_seq'),

  -- Equivalent to:
  product_no SERIAL,
  …
);
```

Every table also has _system columns_ that are implicitly defined by the system, which means that it is not possible to user-define columns with clashing names.

| Name       | Purpose                                                        |
|------------|----------------------------------------------------------------|
| `oid`      | row's object ID                                                |
| `tableoid` | table's object ID                                              |
| `xmin`     | transaction ID of row version's inserting transaction          |
| `cmin`     | command ID within inserting transaction                        |
| `xmax`     | transaction ID of undeleted row version's deleting transaction |
| `cmax`     | command ID within deleting transaction                         |
| `ctid`     | row's physical location within table                           |

## Schemas

Each database contains one or more named schemas, each of which can contain named objects such as tables, data types, functions, and operators.

Schemas facilitate many users using a single database. They allow the organization of database objects into logical groups. For example, third-party applications may operate in separate schemas to avoid colliding with user objects.

Any given connection can only access data from a single database: the one connected to. However, a user can access objects from any schema in that database as long as they have the required privileges.

A schema can be created with the `CREATE SCHEMA` command. It's possible to create a schema that will be owned by someone else with the `AUTHORIZATION` option, and if the schema name is omitted then it will be named after the authorized user.

``` postgresql
CREATE SCHEMA myschema;

-- Give ownership to myuser:
CREATE SCHEMA myschema AUTHORIZATION myuser;
```

An empty schema can be dropped with the `DROP SCHEMA` command. The `CASCADE` option can be specified to drop any contained objects.

``` postgresql
DROP SCHEMA myschema;

-- Drop all contained objects too:
DROP SCHEMA myschema CASCADE;
```

Objects can be created within the schema by giving them a qualified name consisting of the schema name as the prefix.

``` postgresql
CREATE TABLE myschema.mytable ( … );

-- More general:
CREATE TABLE mydatabase.myschema.mytable ( … );
```

By default, tables and other objects without a qualified schema are put into a schema named "public".

A schema _search path_ is consulted when the system attempts to lookup an unqualified name. Non-existent schemas in the search path are ignored. The first effective schema in the search path is called the _current schema_, which is also the schema in which new unqualified tables are created.

The `pg_`-prefixed schemas comprise a PostgreSQL namespace. For example, the `pg_catalog` schema contains system tables and all of the built-in data types, functions, and operators. The `pg_catalog` schema is implicitly always part of the search path, although it can also be explicitly placed, such as at the end of the search path to enable user-defined names to override built-in ones.

In order to qualify operators it's necessary to use the `OPERATOR` keyword:

``` postgresql
SELECT 3 OPERATOR(pg_catalog.+) 4;
```

A schema's owner can grant access privilege to another user with the `USAGE` privilege, and the `CREATE` privilege can be granted to allow the creation of objects within the schema. By default, everyone has `CREATE` and `USAGE` privileges on the `public` schema.

## Table Alteration

It's possible to alter existing tables in a variety of ways with the `ALTER TABLE` command. This is preferred over dropping the table and recreating it when it already has a lot of data or when the table is already referenced by other database objects, such as foreign key constraints.

A table can be renamed with the `RENAME` clause:

``` postgresql
ALTER TABLE products RENAME TO items;
```

A column can be renamed with the `RENAME COLUMN` clause:

``` postgresql
ALTER TABLE products RENAME COLUMN product_no TO product_number;
```

A column can be added to a table with the `ADD COLUMN` clause, which accepts all of the same options that a column description accepts within a `CREATE TABLE` command:

``` postgresql
ALTER TABLE products ADD COLUMN description text;

-- With a constraint:
ALTER TABLE products ADD COLUMN description text
CHECK (description <> '');
```

A column can be removed with the `DROP COLUMN` clause. Table constraints involving the column are dropped, unless it is a foreign key constraint, unless the `CASCADE` option is given.

``` postgresql
ALTER TABLE products DROP COLUMN description;

-- Drop anything that depends on the column:
ALTER TABLE products DROP COLUMN description CASCADE;
```

A constraint can be added to the table using table constraint syntax.

``` postgresql
ALTER TABLE products ADD CHECK (name <> '');
ALTER TABLE products ADD CONSTRAINT some_name UNIQUE (product_no);
ALTER TABLE products ADD FOREIGN KEY (product_group_id) REFERENCES product_groups;

-- Add Not-NULL constraint
ALTER TABLE products ALTER COLUMN product_no SET NOT NULL;
```

A constraint can be removed from a table by name. The `CASCADE` option may be necessary in order to drop everything that may depend on that constraint, such as a foreign key constraint depending on a unique or primary key constraint on the referenced column(s).

``` postgresql
ALTER TABLE products DROP CONSTRAINT some_constraint;

-- Remove Not-NULL constraint
ALTER TABLE products ALTER COLUMN product_no DROP NOT NULL;
```

A column's default value can be changed with the `ALTER COLUMN` clause. Since this is setting the new _default_, it doesn't affect any existing defaulted values.

``` postgresql
ALTER TABLE products ALTER COLUMN price SET DEFAULT 7.77;
```

A column's default value can be removed with the `DROP DEFAULT` option. This is equivalent to setting the default to `NULL`, making this option idempotent.

``` postgresql
ALTER TABLE products ALTER COLUMN price DROP DEFAULT;
```

The type of a column can be changed with the `TYPE` option. This operation only succeeds if every row's corresponding column value can be converted to the new type by an _implicit cast_. Otherwise, an explicit conversion can be specified with the `USING` option.

PostgreSQL also attempts to convert the default value to the new type and any existing affected constraints. This may not always yield expected results, so it's advised to drop the constraints, convert the column, then recreate them.

``` postgresql
ALTER TABLE products ALTER COLUMN price TYPE numeric(10,2);
```

## Constraints

Constraints are a way of limiting the kind of data stored in a table. Attempting to store data in a column that would violate a constraint causes an error to be raised.

### Check Constraints

A _check constraint_ is the most generic constraint type. It simply specifies that a value in a column must satisfy some Boolean predicate. Constraints come after the data type with the keyword `CHECK`.

``` postgresql
CREATE TABLE products (
  product_no integer,
  name text,
  price numeric CHECK (price > 0)
);
```

Constraints may be given names with keyword `CONSTRAINT` in order to clarify error messages and to gain the ability to refer to them for future alteration.

``` postgresql
CREATE TABLE products (
  product_no integer,
  name text,
  price numeric CONSTRAINT positive_price CHECK (price > 0)
);
```

Checked constraints may refer to multiple columns, in which case it is not attached to any particular column but instead appears as a separate item in the comma-separated column list. The order between column and constraint definitions may be mixed.

### Column Constraints

_Column constraints_ are constraints attached to a particular column, whereas _table constraints_ are constraints that are written separately from any one column. Like column constraints, table constraints can be given names with the `CONSTRAINT` keyword.

``` postgresql
CREATE TABLE products (
  product_no integer,
  name text,
  price numeric CHECK (price > 0),
  discounted_price numeric CHECK (discounted_price > 0),
  CHECK (price > discounted_price)
);
```

Note that column constraints may be written as table constraints, but the reverse is not always possible.

``` postgresql
-- The above can also be expressed as:
CREATE TABLE products (
  product_no integer,
  name text,
  price numeric,
  CHECK (price > 0),
  discounted_price numeric,
  CHECK (discounted_price > 0),
  CHECK (price > discounted_price)
);
```

### Not-NULL Constraints

A _not-null constraint_ is one that ensures that a value is not `NULL`. Note that this can also be done via checked constraints with an `IS NOT NULL` expression, but a not-null constraint is more efficient in PostgreSQL at the expense of being unable to name them.

``` postgresql
CREATE TABLE products (
  product_no integer NOT NULL,
  name text NOT NULL,
  price numeric
);
```

Note that columns may have more than one constraint, written in any order, which doesn't necessarily determine the order in which they are checked.

There is an inverse to the `NOT NULL` constraint, `NULL`, which explicitly specifies the default constraint that the value _may be_ `NULL`.

It is generally a good idea to mark the majority of columns `NOT NULL`.

### Unique Constraints

_Unique constraints_ ensure that data in a column or group of columns is unique among all other rows in the table. This is commonly used for row identifiers, since otherwise the identifier could not be used reliably to identify a single row. A unique constraint is represented by the `UNIQUE` keyword. Unique constraints may be given names via `CONSTRAINT`.

``` postgresql
CREATE TABLE products (
  product_no integer UNIQUE,
  name text,
  price numeric
);
```

It can also be written as a table constraint.

``` postgresql
CREATE TABLE products (
  product_no integer,
  name text,
  price numeric,
  UNIQUE (product_no)
);
```

Unique constraints may be specified for a _group_ of columns, which ensures that the _combination_ of values of the specified columns is unique across the entire table, by using a table constraint with a comma-separated list of columns.

``` postgresql
CREATE TABLE products (
  a integer,
  b integer,
  c integer,
  UNIQUE (a, c)
);
```

Creating a unique constraint also automatically creates a unique B-tree index on the column(s) involved in the constraint.

Since any two `NULL` values are _never_ considered to be equal, it is possible to store duplicate rows despite a multi-column constraint if at least one of the constrained columns contains a `NULL` value, as per the SQL standard.

### Primary Key Constraints

A _primary key constraint_ is one that indicates that a column or group of columns can be used as a _unique identifier_ for rows in a table. This necessitates that the values be unique and _not_ null, i.e. similar to `UNIQUE NOT NULL`, except that the existence of a primary key constraint automatically creates a B-tree index on the constrained column(s), and forces the column(s) to be marked `NOT NULL`.

A table can have at most one primary key, but may have multiple unique not-null constraints.

A primary key defines the default target column(s) for foreign keys referencing the table.

``` postgresql
CREATE TABLE products (
  product_no integer PRIMARY KEY,
  name text,
  price numeric
);

-- More than one column:
CREATE TABLE example (
  a integer,
  b integer,
  c integer,
  PRIMARY KEY (a, c)
);
```

### Foreign Key Constraints

A _foreign key constraint_ declares that values in a column must match values of some row in another table, so as to maintain _referential integrity_ between two related tables. In practice this means that a row cannot be created on the referencing table if it doesn't have a foreign key value that exists in the referenced table.

If no explicit referenced column is specified then the primary key of the referenced table is used.

``` postgresql
CREATE TABLE products (
  product_no integer PRIMARY KEY,
  name text,
  price numeric
);

CREATE TABLE orders (
  order_id integer PRIMARY KEY,
  product_no integer REFERENCES products (product_no),

  -- Equivalent:
  product_no integer REFERENCES products,

  quantity integer
);
```

A foreign key can constrain and reference a group of columns, in which case it needs to be specified in table constraint form.

Foreign keys must reference columns that are primary keys or uniquely constrained, which implies that the referenced columns always have an index.

Foreign key constraints must specify `NOT NULL` if they want to enforce that each foreign constraint is satisfied. Otherwise any referencing columns may be `NULL` unless `MATCH FULL` is specified which requires all referencing columns to be set or `NULL`.

``` postgresql
CREATE TABLE t1 (
  a integer PRIMARY KEY,
  b integer,
  c integer,
  FOREIGN KEY (b, c) REFERENCES other_table (c1, c2);
);
```

It's possible to define more than one foreign key constraint, something which is often done to implement many-to-many relationships.

It's possible to configure what occurs when a referenced row is removed by using an `ON DELETE` clause:

The `ON DELETE RESTRICT` clause can be used to prevent the referenced row from being deleted.

The `ON DELETE NO ACTION` clause is the default behavior, which simply raises an error, essentially preventing the deletion. The difference between this and `RESTRICT` is that this check can be deferred until the end of a transaction.

The `ON DELETE CASCADE` clause causes referencing row(s) to be deleted as well.

The `ON DELETE SET NULL` clause can be used to set the foreign key column(s) in the referencing row(s) to `NULL`. There is also an `ON DELETE SET DEFAULT` variant which sets the default value for that type instead. Both of these behaviors are still subject to any constraints.

There is a corresponding `ON UPDATE` clause with the same possible options.

Since deleting or updating a referenced row requires a scan of referencing tables, it's a good idea to create an index for referencing columns.

### Exclusion Constraints

Exclusion constraints ensure that no two rows satisfy a given set of operators, that is, the constraint is satisfied if at least one operator returns false or `NULL`. An exclusion constraint automatically adds an index of the type specified.

For example, an exclusion constraint can be used to ensure that no two circles overlap.

``` postgresql
CREATE TABLE circles (
  c circle,
  EXCLUDE USING gist (c WITH &&)
);
```

# Data Manipulation

## Insertion

The `INSERT INTO … VALUES` command lists the values in the order in which the columns appear in the table, unless the columns are explicitly listed.

``` postgresql
INSERT INTO products VALUES (1, 'Cheese', 9.99);

-- Or:
INSERT INTO products (product_no, name, price)
VALUES (1, 'Cheese', 9.99);
```

Any columns that aren't given values are filled with their default values. It's also possible to explicitly request default values with the `DEFAULT` value.

``` postgresql
INSERT INTO products (product_no, name, price)
VALUES (1, 'Cheese', DEFAULT);
```

Multiple rows can be inserted by listing multiple row tuples.

``` postgresql
INSERT INTO products (product_no, name, price)
VALUES (1, 'Cheese', 9.99),
       (2, 'Bread', 1.99);
```

The result of a query (no rows, one row, or many rows) can be inserted into a table.

``` postgresql
INSERT INTO products (product_no, name, price)
  SELECT product_no, name, price,
  FROM new_products
  WHERE release_date = 'today';
```

Note that bulk loading can be more efficient when done with the `COPY` command.

## Updating

Updating requires the name of the table and column to update, the new value of that column, and which row(s) to update specified as conditions. If the row condition is omitted, then the update applies to all rows in the table. The new column value can be any scalar expression.

It is not an error to attempt an update that does not match any rows.

``` postgresql
UPDATE products
SET price = 10
WHERE price = 5;
```

More than one column can be updated by listing more than one assignment in the `SET` clause.

``` postgresql
UPDATE mytable
SET a = 5, b = 3, c = 1
WHERE a > 0;
```

## Deleting

Note that omitting a condition in a `DELETE` statement makes it apply to _all rows in the table_.

``` postgresql
DELETE FROM products WHERE price = 10;

-- DELETES ALL ROWS
DELETE FROM products;
```

## Returning Modified Rows

The `INSERT`, `UPDATE`, and `DELETE` commands have an optional `RETURNING` clause that can return data from rows that are manipulated by those commands, thereby avoiding an additional query. The allowed contents of the `RETURNING` clause are the same as `SELECT`'s output list.

If the table has triggers, the data available to `RETURNING` is the row as modified by those triggers.This makes `RETURNING` useful for inspecting columns computed by triggers.

The `RETURNING` clause can be useful when paired with the `INSERT` command to access computed default values, such as a `serial` column's unique row identifier.

``` postgresql
CREATE TABLE users (
  firstname text,
  lastname text,
  id serial primary key
);

-- Get the inserted row's default-computed id.
INSERT INTO users (firstname, lastname)
VALUES ('Joe', 'Cool')
RETURNING id;
```

The `RETURNING` clause can be useful when paired with the `UPDATE` command to retrieve the new computed content of a modified row.

``` postgresql
-- Get the modified rows' newly computed prices.
UPDATE products SET price = price * 1.10
WHERE price <= 9.99
RETURNING name, price AS new_price;
```

The `RETURNING` clause can be useful when paired with the `DELETE` command to obtain the content of the deleted row.

``` postgresql
DELETE FROM products
WHERE obsoletion_date = 'today'
RETURNING *;
```

# Privileges

Each created object is assigned an owner, which is usually the role that executed the creation statement. For most object kinds, the initial configuration is such that only the owner or a superuser can do anything with the object unless another role is granted _privilege_. The right to modify or destroy the object is always the privilege of the owner _only_.

Different privileges apply to different kinds of objects. The different kinds of privilege are:

* `SELECT`
* `INSERT`
* `UPDATE`
* `DELETE`
* `TRUNCATE`
* `REFERENCES`
* `TRIGGER`
* `CREATE`
* `CONNECT`
* `TEMPORARY`
* `EXECUTE`
* `USAGE`

An object can be assigned to a new owner with the appropriate `ALTER` command for the particular object kind. Superusers can always do this, and ordinary roles can only do this if they are the current owner of the object _and_ a member of the new owning role.

Specific privileges can be granted with the `GRANT` command. Specifying `ALL` as the privilege grants all of the privileges. The special role `PUBLIC` can be used to grant a privilege to every role on the system.

It's possible to grant a privilege which carries the additional privilege to grant that same privilege to others (known as "with grant privilege"), and if the grant option is subsequently revoked then everyone who received that privilege also loses it.

``` postgresql
GRANT UPDATE ON accounts TO joe;
```

Privileges can be revoked with the `REVOKE` command. Note that the owner's special privileges to `DROP`, `GRANT`, and `REVOKE` are implicit in being the owner, but the owner can revoke their other ordinary privileges.

``` postgresql
REVOKE ALL ON accounts FROM PUBLIC;
```

# Row Security Policies

Also known as _Row-Level Security_ (RLS).

Tables can have row security policies which restrict, on a per-user basis, which rows are returned by normal queries or inserted, updated, or deleted by data modification commands.

When row security is enabled on a table, all access must be allowed by the policy, which is default-deny when none is specified. Table-wide operations such as `TRUNCATE` or `REFERENCES` are not subject to row security. Row security policies can be specific to commands, roles, or both.

The condition for which rows are visible or modifiable according to a policy is expressed by an expression that yields a Boolean result, which is then evaluated for each row prior to any conditions or functions of the user's query. Separate expressions can be specified for separate readable and modifiable policies.

Superusers, roles with the `BYPASSRLS` attribute, and table owners bypass row security, although the owner can choose to subject themselves to RLS.

Enabling or disabling row security or adding policies is a privilege of the owner only. Removing row security does not remove any existing policies, it simply ignores them.

The following example only allows the `managers` role to access rows, and only rows of their accounts.

``` postgresql
CREATE TABLE accounts (manager text, company text, contact_email text);

ALTER TABLE accounts ENABLE ROW LEVEL SECURITY;

CREATE POLICY account_managers ON accounts TO managers
  USING (manager = current_user);
```

Row security should be turned off when doing a backup to avoid certain rows from being omitted in the backup.

# Inheritance

Inheritance in PostgreSQL is similar to the concept with the same name from object oriented programming languages.

In PostgreSQL a table can inherit from zero or more tables.

``` postgresql
CREATE TABLE cities (
  name text,
  population float,
  altitude int
);

CREATE TABLE capitals (
  state char(2)
) INHERITS (cities);
```

A query can reference either all rows of that table or all rows of that table _plus_ all of its descendant tables, the latter behavior being the default. An asterisk `*` suffix on the table name can be included to explicitly specify that all tables of the specified type should be queried.

``` postgresql
-- Query all kinds of cities, including capitals
SELECT name, altitude
  FROM cities
  WHERE altitude > 500;

-- Equivalent:
SELECT name, altitude
  FROM cities*
  WHERE altitude > 500;
```

The system column `tableoid` can be used to determine the source table of a row.

``` postgresql
SELECT c.tableoid::regclass, c.name, c.altitude
FROM cities c
WHERE c.altitude > 500;
```

A query can be restricted to a specific type of table with the `FROM ONLY` clause. The `ONLY` keyword is supported by many commands including `SELECT`, `UPDATE`, and `DELETE`.

``` postgresql
-- Query only cities, excluding capitals
SELECT name, altitude
  FROM ONLY cities
  WHERE altitude > 500;
```

Note that inheritance does not automatically propagate data from `INSERT` or `COPY` commands. That is, it would not be correct to insert a capital into `cities` expecting it to be routed to `capitals`.

Check constraints and not-null constraints are automatically inherited by children, unless specified otherwise via `NO INHERIT` clauses, but other constraints such as unique, primary, or foreign key constraints are not inherited.

A table that inherits from more than one table is comprised of the union of the columns of the parent tables plus the columns in the child table. Duplicate columns and check/not-null constraints are merged if they are of the same type, otherwise an error is raised.

Existing tables can have their inheritance relationship linked or unlinked with the `ALTER TABLE` command assuming they are compatible. This is often used for table partitioning.

Parent tables cannot be dropped while children exist, nor can columns or check constraints of child tables be dropped or altered. However, a parent and all of its children can be removed with the `CASCADE` option, and a parent's columns and checks can be altered and the changes will be propagated to all children.

Note that inherited queries only perform access permission checks on the parent table. Likewise a child table's row security policies are only applicable when the table is explicitly named in the query.

Not all SQL commands work with inheritance hierarchies, such as database maintenance and tuning commands (e.g. `REINDEX`, `VACUUM`), which only work on individual physical tables.

A serious limitation is that indexes (unique constraints implied) and foreign key constraints only apply to single tables and not children. This means:

* A `UNIQUE` or `PRIMARY` constraint on the parent table will not prevent a duplicate row on a child table.
* A foreign key constraint is not propagated to children; they must be manually added on the child.
* A table referencing the parent will not mean that child tables can be referenced.

# Partitioning

Partitioning entails logically splitting a table into smaller physical pieces. The benefits are:

* Query performance can be improved when most of the heavily accessed rows are in a single or a few partitions, which can in turn reduce index size, which improves the possibility that the most heavily used parts of the index fit in memory.
* Query or update performance can be improved for accesses that span a large percentage of a single partition through the use of a sequential scan versus an index and random access.
* Bulk loads and deletes can simply entail adding or removing partitions.
* Rarely-used data can be migrated to cheaper and slower storage.

These benefits are generally only worthwhile when the table is very large, typically when it can't fit in physical memory.

There are two main partitioning schemes.

Range partitioning involves partitioning into ranges of a key column or set of columns such that there is no overlap between the ranges.

List partitioning involves explicitly listing which keys appear in which partition.

The partitioning process typically involves:

1. A master table is created from which all partitions will inherit, which specifies the columns but does not store any data or define any check constraints.
2. Children inherit from the parent, usually without specifying additional columns.
3. Table constraints are added to each child to specify which key values belong in it. It's crucial to ensure that there is no overlap.

    ``` postgresql
    -- List partitioning:
    CHECK ( county IN ( 'Los Angeles', 'Orange' ))

    -- Range partitioning:
    CHECK ( outletID >= 100 AND outletID < 200 )
    ```

4. Create an index on the key column(s) for each child.
5. Optionally create a trigger or rule to redirect data inserted into the master table to the correct partition.
6. Ensure that `constraint_exclusion` is disabled.

One scenario for leveraging partitioning might be for tables where only recent rows (e.g. past month) rows are accessed. Data can then be rotated throughout different partitions as they age, with the oldest partition simply being dropped if there's a cut-off.

Partitioned tables have a few caveats:

* No automatic way to verify that all `CHECK` constraints are mutually exclusive.
* The partition key column(s) cannot be easily changed.
* Manual `VACUUM` and `ANALYZE` commands must be run on each partition individually.
* `INSERT` commands with `ON CONFLICT` probably won't work as expected since conflicts with child relations aren't considered.

_Constraint exclusion_ is a query optimization that causes the planner to analyze the check constraints of each partition to try to prove that a partition need not be scanned because it will not contain any candidate rows, and if it succeeds in proving this then the partition can be excluded.

Constraint exclusion has a few caveats:

* It only works when the query's `WHERE` clause contains constants. For example, comparing against `CURRENT_TIMESTAMP` cannot be optimized because the planner cannot know which partition it would fall under at run time.
* The partitioning constraints should be simple in order to facilitate the query planner's attempt to prove that the partitions won't be visited.
* All constraints on all partitions of the master table are examined, which can increase query planning time as the number of partitions and constraints increases (e.g. more than 100).

# Dependency Tracking

The creation of database objects often implies dependencies between those objects. PostgreSQL prevents dropping objects that are being depended on unless explicitly specified via `CASCADE`, in which case the dependent objects are dropped recursively as well. The default behavior is `RESTRICT`.

Dependency tracking for functions is based on the arguments and result types, but not the function body.

# PostGIS

Functions with the `ST_` prefix are from the SQL/MM specification, while older functions from the OGC/SFSQL specifications lack the prefix.

The `geometry` type is planar and uses Cartesian math. PostGIS autocasts a `geometry` to its bounding box when needed.

The `geography` type is spheroidal and geodetic, with lines and polygons drawn on a curved surface. It assumes that all data is based on a geodetic coordinate system, specifically WGS 84 lon/lat SRID 4326, unless otherwise specified.

The `raster` type is a multiband cell and they're used to model space as a grid of rectangular cells each containing a numeric array of values. They organize information using _pixels_, AKA _cells_. A pixel is just a space holder for data with no physical dimension. Pixels are organized in rows and columns to form tiles. They are only positional designations, the actual data is stored in _bands_, AKA _channels_ or _dimensions_. A _tile_ has a width and height measured in pixels and is usually stored in each row. Each raster can have multiple bands, but must have at least one. Each band can only store numeric values. Pixel types specify the type of numbers that a given band can store.

A _georeferenced raster_ corresponds to actual geographical locations, so that the physical size of a pixel takes on a real unit of measure, which can be measured with `ST_PixelWidth` and `ST_PixelHeight`. Georeferenced rasters have an assigned SRID to denote the SRS.

Referencing a particular pixel on a raster requires a pixel-numbering convention relative to spatial coordinates, which is usually positive in the X direction and negative in the Y direction of the coordinate space, starting at the top-left corner of the tile rectangle.

`ST_DWithin` checks if the minimum distance between two geometries is within the specified distance.

`ST_Buffer` takes a geometry and radially expands it by a specified number of units, resulting in a _buffer zone_ or _corridor_.

Type modifiers (aka _typmods_) are used when specifying the length of a `varchar(8)` or the precision in `numeric(8,2)`. Check constraints can be used to achieve the same effect, such as a check constraint that limits the length of a `varchar` column to 8.

The `geometry` and `geography` types have hierarchical structures, so it's possible (and eoucraged) to be more specific where possible and use the subtypes. Although the subtypes are not data types, they can be enforced through type modifiers, such as `geometry(POINT, 4326)` specifies the `POINT` subtype type modifier and the 4326 SRID type modifier.

All data types have a _spatial reference identifier_ (SRID). Only objects with the same SRID can have spatial relationships between them, such as their distance or whether one is contained in another.

A _simple linestring_ is one that doesn't cross itself, and can be checked with `ST_IsSimple`.

A _closed linestring_ is one that ends where it begins, otherwise it's considered to be open. When a closed linestring outlines the boundary of a polygon it's called the polygon's exterior ring. A polygon must have exactly one exterior ring, but may have one or more inner rings, each creating a hole in the overall polygon.

The well-known text (WKT) representation of a polygon is a set of closed linstrings, with the first designating the exterior ring and subsequent ones designating the inner rings.

For a polygon to be considered valid:

* the rings may not overlap each other
* two rings can't share a common boundary
* inner rings cannot lie partly outside its exterior ring

Many GEOS-based functions in PostGIS behave unpredictably with invalid geometries, so care should be taken to ensure that geometries are valid. `ST_IsValid` can determine validity, and `ST_IsValidReason` can explain the first validity offense.

PostGIS has many geometry collection types such as `MULTIPOLYGON` which can store multiple polygons. It also has a `GEOMETRYCOLLECTION` type is a heterogeneous collection of geometries as long as all geometries have the same saptial reference system and coordinate dimensions. This is unlike multigeometries which are homogeneous.

Multilinestrings are simple if all contained linestrings are simple and they don't intersect each other except at boundary points.

Multipolygons are valid if all contained polygons are valid and they don't overlap.

A spatial reference identifier (SRID) is a foreign key into the `spatial_ref_sys` table, which is a catalog of the spatial reference systems available to the database, including its name, projection parameters, and the originating organization. The SRID column is user-input, so SRID 4326 can't be guaranteed to correspond go the global SRS EPSG:4326 in every other database.

Functions that expect an SRID but find it missing will default to the unknown value.

Switching SRIDs doesn't alter the fact that the coordinate system underlying the geometry data type is always cartesian.

Geodetics is the science of measuring and modeling the earth, while cartography is the science of representing it on flat maps.

A spatial reference system is comprised of an ellipsoid, datum, and projection.

A geoid is produced by taking gravity readings at various sea levels to arrive at a consensus and then use that constant gravitational force to map an equigravitational surface.

An ellipsoid is composed of three radii: a and b are equatorial (along the X and Y axes) and c is the polar radius (along the Z axis). One ellipsoid may fit the curvature of one spot better than it would another spot, so there are a multitude of ellipsoids in use today.

The datum is an "anchor" point on the earth where the axix arrives at the surface, i.e. the poles. Even if two reference systems use the same ellipsoid, they may have different anchors (datum). A horizontal datum specifies where on the plane of the earth to pin down the ellipsoid, and a vertical datum specifies the height.

The most popular coordinate reference system is the geographical coordinate system (lon/lat) with longitude lines connecting the poles and latitude lines parallel to the equator.

All data serve in the form of latitude and longitude is unprojected. Designing a projection is a balance of four conflicting features:

* measurement accuracy
* shape (angle representation)
* direction (is north really north)
* range of area supported

Using a `geometry` type that is SRID 4326 data will actually project it onto a flat surface, treating longitude as X and latitude as Y. Some functions that work with `geography` actually piggyback on `geometry`, such as `ST_Buffer`, so caution should be taken when using them for large areas where the curvature of the earth becomes a concern.

Note that `ST_Transform` is not a lossless function, as it introduces floating-point errors that can quickly accumulate.

`ST_NPoints` is a PostGIS-specific function that returns the number of points in a geometry. The `ST_NumPoints` function is standard but only works on linestrings.

`ST_Boundary` returns the geometry that determines the separation between points in the geometry and the rest of the coordinate space.