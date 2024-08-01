# Git alias for custom commands

<https://github.com/GitAlias/GitAlias>

Git alias is a Git built-in capability that enables you to configure your own custom Git commands.

For example, you can create a git alias "s" to represent "status":

```sh
git config alias.s status
```

Now when you want to check your status, you can type:

```sh
git s
```

Git alias can create more-complex commands, such as sequences of git actions, function calls to shell scripts, and integrations with other tools.


## GitAlias project

The GitAlias project is a curated collection of our favorite Git alias configurations, for shortcuts as well as for more-complex commands. To learn more, see the GitAlias link above.

GitAlias provides shortcuts such as:

```gitalias
git a = add
git b = branch
git c = commit
git l = log
git s = status
```

GitAlias provides more-complex commands such as:

```gitalias
git summary - Show a summary of overview metrics
git chart - Show highlights chart of activity per author
git churn - Show log of files that have many changes
git hew - Delete all branches that are merged into a commit
git optimizer - Optimize a repo by using pruning and repacking
git snapshot - Stash a snapshot of your current working tree
```
