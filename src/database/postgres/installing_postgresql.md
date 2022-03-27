## Installing PostgreSQL

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]


Before running these examples that access a live Postgresql database,
you will need to:

1. Install the Postgresql database server.
2. Start the server if that was not done automatically.
3. Configure the database to accept client connections at address
`127.0.0.1` (localhost) on port number `5432`.
4. Set a username and password to access the database.

The following examples expect a database user with full permissions to exist as:

* Username: postgres
* Password: ChangeMe99

Modify the examples with the actual username and password.

Obviously, such a test database should not be accessible from the
Internet, or even from outside the computer you are working on.

# Arch Linux

https://wiki.archlinux.org/title/PostgreSQL

# macOS

https://www.postgresql.org/download/macosx/

# Windows

https://www.postgresql.org/download/windows/

# Ubuntu

Ubuntu server guide instructions (can be run on Ubuntu Desktop as well):

https://ubuntu.com/server/docs/databases-postgresql

The instructions can be followed almost exactly. The instructions to
enable PostgreSQL to listen to all network interfaces may be skipped if running
these examples on the same system.

This is the actual SQL statement to set the `postgres` *database* user password:

```
ALTER USER postgres with encrypted password 'ChangeMe99';
```

Note that does not change the `postgres` system account password.

Be sure to change the password for both the database and system
`postgres` accounts before putting a database server into production.

# Create the `cats` test database

Connect to the database server using the `psql` command-line client,
using the password that was set:

```
psql -h 127.0.0.1 -U postgres -W
```

When connected, create the `cats` database with this SQL statement:

```
CREATE DATABASE cats;
```

