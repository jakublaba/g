# git-multiaccount (gma)
gma is a cli tool for managing multiple git profiles, goal is to have it manage ssh keys + config under the hood, as well as manipulating repo urls without any hassle on user's end.

# Installation
TODO after first release

# Usage
I don't see much point in explaining each command and subcommand with flags as the help is pretty comprehensive (at least I hope so).
Here's the overview of top level commands:
```
Usage: gma [COMMAND]

Commands:
  su       Switch profiles
  profile  Manage profiles
  help     Print this message or the help of the given subcommand(s)
```

# How does it work?
gma stores your profiles as `.json` files in `~/.config/<name>/`, containing the following data:
```json
{
  "name": [name of the profile],
  "user_name": [name of git user],
  "user_email": [email of git user]
}
```
Upon creation of profile via gma, such config file is created, there is also `ssh-ed25519` key pair generated (so far `ed25519` is the only supported algorithm).
There is also an entry added to your `~/.ssh/config`, which configures `github.com-{profile}` host to use dedicated private key for the newly created profile.
When cloning a repo, gma replaces `github.com` in url with `github.com-{profile}`.
