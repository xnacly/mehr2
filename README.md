# mehr2

Linux-only distro overarching declarative package managment abstraction,
enabling setting new systems up reliably.  

> mehr2 does not support windows or macos, specifically only linux is
> supported.

## Why

I have three machines I regulary use for software dev, one of these is using
ubuntu and two are using arch, thus i have to sync all three to the same state
of installed packages. Doing so manually is a pain, so I used `mehr` to
serialize the list of packages, I use, into a configuration file. `mehr2`
attempts to introduce the missing pieces of `mehr`: a cleaner and more
scriptable configuration and installing packages from source via bash scripts
inside the mehr configuration.

## Supported package managers

| Manager | Supported |
| ------- | --------- |
| pacman  | ✅        |
| scratch | ❌        |
| cargo   | ✅        |
| go      | ❌        |
| apt     | ❌        |
| npm     | ❌        |
