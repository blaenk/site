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

The `PARTITION BY` sub-clause partitions (groups) the rows sharing the same values for the provided expression. These partitions are processed separately by the window function. The `PARTITION BY` clause is similar to `GROUP BY` except its values are always expressions, not output-column names or numbers The `ORDER BY` sub-clause can be used to order each resulting partition. Then, for each row, the window function is computed across the rows that are in the same partition.

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

# Inheritance

Inheritance in PostgreSQL is similar to the concept with the same name from object oriented programming languages. In PostgreSQL, a table can inherit from zero or more tables.

# Queries

Expressions can be written in the `SELECT` output list.

The `AS` option can be provided to rename an output column.

## Scalar Subqueries

A _scalar subquery_ is an ordinary parenthesized `SELECT` query that returns exactly _one_ row with _one_ column. It would be an error if it returned more than one row or column, but returning nothing at all is interpreted as being `NULL`.

``` postgresql
SELECT
  name,
  (SELECT max(pop) FROM cities WHERE cities.state = states.name)
FROM states;
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

## Row Constructors

A row constructor is an expression that builds a "row", or _composite value_. This consists of parenthesizing the fields and using a `ROW` prefix. The prefix is optional if there's more than one field. The type of this row is anonymous unless cast to a named composite type: either the row type of a table or one created with `CREATE TYPE AS`.

``` postgresql
SELECT ROW(1, 2.5, 'this is a test');
```

Row constructors are typically used for storing values in composite-type table columns or passing composite arguments to functions.

Rows can be compared and tested for being `NULL`.

The `.*` syntax may be used to expand an element row expression into fields of the row being constructed. In other words, if table `t` has fields `f1` and `f2`, this is possible:

``` postgresql
SELECT ROW(t.*, 42) FROM t;

-- Equivalent to:
SELECT ROW(t.f1, t.f2, 42) FROM t;

-- To get a row whose first field is itself a row:
SELECT ROW(t, 42) FROM t;
```

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

# Operators

Schema-qualified operators can be written by using the `OPERATOR` keyword. Note that the effective operator's precedence is the same regardless of the precedence of the operator passed as the argument.

``` postgresql
SELECT 3 OPERATOR(pg_catalog.+) 4;
```

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

There is an inverse to the `NULL` constraint, `NOT NULL`, which explicitly specifies the default constraint that the value _may be_ `NULL`.

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
