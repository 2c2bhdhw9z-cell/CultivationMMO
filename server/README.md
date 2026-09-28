# Local server skeletons

Task 2 uses Go 1.25.1 and only its standard library for the account API, owner-control gateway, maintenance tool, and database checker. The valley zone is Godot 4.7.2.

Nothing here is hosted. Nothing connects to Railway or PostgreSQL.

From this `server` directory:

```sh
go run ./api --self-check
go run ./owner-control --self-check
go run ./maintenance --self-check
go run ./database --self-check
```

Normal API and owner-control starts bind only to loopback. The owner-control service serves no protected console assets and has zero commands. Maintenance and database commands refuse normal work because no plan, credential, database version, or migration exists.
