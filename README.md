# mehr2

mehr enables declarative package provisioning across Linux distributions.

The main goal is to keep multiple systems in sync with a single configuration,
delegating package installs to native package management providers like:
pacman, apt, cargo and npm.

mehr only tracks packages installed by mehr in a lock file and restricts its
effects on these.

## Name

mehr is a pun on nix. nix in German means nothing, mehr translates to more,
which is inherently more than nothing. mehr2 is an iteration on the original
mehr.

## Why

I have three machines I regularly use for software dev, one of these is using
ubuntu and two are using arch, thus i have to sync all three to the same state
of installed packages. Doing so manually is a pain, so I used `mehr` to
serialize the list of packages, I use, into a configuration file. `mehr2`
attempts to introduce the missing pieces of `mehr`: a cleaner and more
scriptable configuration and installing packages from source via bash scripts
inside the mehr configuration.

## Goals and Features

- thin abstraction over native package managers (called providers)
- minimalist configuration file
- custom package installs and configs via scratch packages
- mehr manages ONLY what mehr owns:

  meta data is persisted in a lockfile and mehr will not interact
  with packages not tracked by mehr

## Supported providers

| Provider | Supported    |
| -------- | ------------ |
| pacman   | yes          |
| cargo    | yes          |
| go       | planned      |
| apt      | planned      |
| npm      | planned      |
| nix      | out of scope |

## Quickstart

Grab the latest static x86 Linux release from the
[releases page](https://github.com/xnacly/mehr2/releases):

```sh
curl -LO https://github.com/xnacly/mehr2/releases/latest/download/mehr2-x86_64-linux
chmod +x ./mehr2-x86_64-linux
```

Drop a configuration file at `$XDG_CONFIG_HOME/mehr2/mehr2.lua` (or
`~/.config/mehr2/mehr2.lua`, or anywhere pointed to by `$MEHR_PATH`), see
[mehr2.lua](./mehr2.lua) for an annotated example.

```sh
./mehr2-x86_64-linux info     # show what mehr would manage and what's already installed
./mehr2-x86_64-linux sync     # install everything in the config that isn't yet on the system
./mehr2-x86_64-linux update   # upgrade packages mehr already manages
```

## Configuration

View [mehr2.lua](./mehr2.lua) for an annotated configuration file.

## Cli

```text
Declarative package provisioning across Linux distributions

Usage: mehr2 [OPTIONS] <CMD>

Arguments:
  <CMD>
          Possible values:
          - sync:      Sync system to configuration file
          - update:    Attempt to update all mehr managed packages
          - info:      Overview over packages managed by mehr
          - providers: Show a list of providers
          - version

Options:
  -f, --force
          Ignore the lock file and treat every package in the configuration as pending. With `sync`, this reinstalls every native package and re-runs every scratch script.
          
          Use this when the system has drifted from the lock or when you want scratch packages rebuilt against fresh upstreams.
          
          Especially useful when combined with --only-provider scratch, this rebuilds all scratch packages.

      --json
          give the actions output in json

  -s, --silent
          remove all info, error, warn, trace, debug logs

  -d, --dry
          stub all actions producing side effects, logged instead

  -p, --only-provider <PROVIDER>
          Restrict the action to a single provider by name (for instance `pacman`, `cargo`, `scratch`). Other providers are skipped entirely

  -h, --help
          Print help (see a summary with '-h')
```
