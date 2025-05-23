# The SaaS RS CLI

## Installing

```shell
$ cargo install saas-rs-cli
```

## Using

### Login

Start by performing a web login to obtain a 24-hour session token.

```shell
$ saas-rs login
Logged in. Greetings David Rauschenbach!
```

If the CLI hangs after the web login completes, try a 2nd time, or try Chrome:

```shell
$ saas-rs login --browser chrome
Logged in. Greetings David Rauschenbach!
```

### Create a new project

Start by initializing a new project

```shell
$ mkdir my_project
$ cd my_project
$ git init
$ saas-rs init --brand my-new-awesome-brand
```

### Generate a user-facing endpoint and CLI

```shell
$ saas-rs generate service user
```

### Generate a resource (a model, plus its CRUD verbs)

```shell
$ saas-rs generate resource project name:string --service user --version 1
```
