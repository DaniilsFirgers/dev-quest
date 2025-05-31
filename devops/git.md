# GIT

## Message conventions

```
<type>(optional scope): <short description up to 50 words>

[optional body]

[optional footer]
```

| Type       | Purpose                                       |
| ---------- | --------------------------------------------- |
| `feat`     | A new feature                                 |
| `fix`      | A bug fix                                     |
| `docs`     | Documentation only changes                    |
| `style`    | Code style changes (formatting, etc.)         |
| `refactor` | Code refactoring (no new features or fixes)   |
| `perf`     | Performance improvements                      |
| `test`     | Adding or updating tests                      |
| `chore`    | Other changes (build system, CI config, etc.) |

Example:

```
feat(auth): add Google login option

Implements OAuth2 flow for Google login
```

Can be used togeter with semantic releases! It will bump a version based on commit messages. then generate a `CHANGELOG.md` file and publish the release.
`Commitlint` + `Husky` as a good option!

## Branching strategies

1. **Git flow** is a structured workflow suited for projects with scheduled releases.

Typically the following branches are used:

- `main`: Production-ready code.
- `develop`:Integration branch for features.
- `feature/*`:Feature-specific branched off `develop`.
- `release/*`:Pre-release branches branched off `develop`.

Typical Flow:

1. Create a `feature/xyz` branch from `develop`.
2. Merge feature to `develop` when complete.
3. When ready to release, create `release/x.y`.
4. Merge `release/x.y` into `main` and `develop` after testing.
5. Tag the release on `main`.

## git add

1. `git add` stages changes by creating a **blob object** for the file, then stores the content in `.git/objects/` and updates the index (staging area) to include the file and its blob hash.

```
git add <path-to_file>

git commit -m "commit message"

git push (if upstream is set) or git push -u origin <branch_name> (if upstream is not set, -u sets it)
```

You can use `git commit -am "Your message"` to **automatically stage (add)** all **tracked** files that have been modified, so no need to run `git add`. But it works only for **exisiting** files that git knwos about!

2. Can check the staged content with `git diff --cached`. Will see something like this:

```
diff --git a/README.md b/README.md
index 2fd6b91..c54cd81 100644
--- a/README.md
+++ b/README.md
@@ -1 +1 @@
-Init commit
+Init commit 2
```

3. Local branch should track a remote branch for push and pull to be working without explicit `origin <branch_name>`

The **upstream** is basically a **remote branch** your local branch tracks.
Can check it by running:

```
git status
```

## git pull

## git log

`git log` shows what changes were made to the repository, by whom and when.

1. `git log --name-only` will show the commit plus **a list of files changed** like this:

```
Author: Daniils Firgers <“dfirger@gmail.com”>
Date:   Sun May 18 13:20:49 2025 +0300

    another change

README.md

commit 4a0945bd6bd5760d651bddb1d98a4b29c1bbf246
Author: Daniils Firgers <“dfirger@gmail.com”>
Date:   Sun May 18 13:07:30 2025 +0300

    add readme file

README.md
```

2. `git log -p` will show **line by line changes** per commit (need to scroll to see them).
3. `git log --stat` will combine both above.

## .git/

`.git/objects/` is the heart of the Git repository's database and it stores all the actual content and history of the project as **Git objects** like this:

```
.git/
└── objects/
    ├── 3b/
    │   └── 18e13d8db22858f7e46e8f229c8fcd9a6e6f29
    ├── e5/
    │   └── 94af239ffdd37c30529f63ae...
    ├── info/
    └── pack/
```

`.git/HEAD` tells git which branch you are on now, which will say something like:

```
ref: refs/heads/master
```

`git cat-file -p <object-hash>` shows the content of the object.
`git cat-file -t <object-hash>` shows type of the object - commit/tree/blob.

## git tag

Git tags are used to mark a a realesed version (like a big feature or a milestone), which makes it easier to find that exact point in history later. A tag is a **fixed pointer to a specific commit** and does not move unlike a **branch**.

1. `git tag <tag_name>` or an annotated tag with `git tag -a <tag_name> -m "Release version"`
   **REMEMBER** - when you do `git checkout v1.0` to a tag or a commit you enter a detached `HEAD` mode where **`HEAD` points directly to a commit or a tag instead of a branch**. You can look around or test older versions. **BUT**, if you want to commit changes to detached `HEAD`, you need to create a new branch from it like this:

```
git checkout -b <new_branch_name> <start_point>
```

2. **REMEMBER** - tags are not pushed to the remote by default, you need to run:

```
git push origin <tag_name>
```

## git reflog a.k.a LIFE SAVER :)

1. Recover a deleted branch

   1. Run `git reflog` to see the **every movement of `HEAD` and branch references** in your local repository.

   ```
   d3adb33 HEAD@{0}: checkout: moving from feature to main
   c0ffee1 HEAD@{1}: commit: Work on feature
   beef456 HEAD@{2}: checkout: moving from develop to feature

   ```

   2. Identify the commit hash (`c0ffee1` for example) related to yout deleted branch.

   3. Create a new branch at that commit:

   ```
   git checkout -b recovered-branch <commit-hash>
   ```

   4. Push the recovered branch back to the remote:

   ```
   git push -u origin recovered-branch
   ```

## git branch

1. List all **local** branches (the currently checked out branch is marked with \*):

```
git branch
```

2. Create a new branch without checking out to it:

```
git branch <new_branch_name>
```

3. Create a new branch and checkout to it:

```
git checkout -b <new_branch_name>
```

4. Delete a branch **locally**:

```
git branch -d <my_branch>
```

5. Delete a **remote** branch:

```
git push origin --delete <remote_branch>
```

6. Rename the current branch (both locally and remotely):

```
git branch -m <new_name>

git push origin --delete <old_branch_name>

git push origin <new_name>

```

## git rebase vs git merge

1. **Git rebase** is git command that **moves or replays** a sequence of commits from one branch into another. It gives a linear commit history.

   > “Take all the commits I made on my branch and pretend I started from somewhere else.”

It is preferred when pushing a small amount of commits developed in a short period of time (hours or minutes). However, it applies only to **local** branch changes! Once the commit is pushed to a **remote** branch it **should not** be rebased!

**Never** rebase **shared/public** branches that others are working on - it rewrites history and causes conflicts!

Here is a typical flow:

```
git checkout feature/login

git fetch origin

git rebase origin/main

--- resolve conflicts ---

git push --force-with-lease ( use it instead of --force!!)
```

**Why use _--force-with-lease_** instead of _--force_?

- When _--force_ is used it says:

  > "I don’t care what’s on the remote — overwrite it with my version."

  Which can **delete commits** made by others, **lose work** if local branch is outdated!

- When _--force-with-lease_ is used:

  > "Only force-push if no one else has pushed to this branch since I last pulled/fetched."

  Which is a safer alternative to _--force_

**To abort rebasing**:

```
git rebase abort
```

2. **Git merge** is a git command that combines changes form one branch into another. It integrates the histories of two branches, creating a single unified branch.

There are two types of merges:

- **Fast forward**
  Happens when the branch you want to merge is directly ahead of the current branch. So git just moves the current branhc pointer forward to the target's branch's commit (no divergent changes).

  ```
  A --- B --- C (feature)
  ^
  main
  ```

- **Non-fast forward**
  Happens when branches have diverged, meaning that both have commits that the other does not have. Git creates a new merge commit that combines the changes from both branches.

  ```
  A --- B --- C -------- M (main)
    \                 /
      D --- E ---------(feature)

  ```

## git cherry-pick

Lets you select a specific commit from another branch that you want to apply into a target branch, without merging the full branch.

```
git checkout <target_branch>

git cherry-pick <target_commit_hash> or <hash1> <hash2>
```

After resolving **conflicts** run:

```
git cherry-pick continue
```

When to use it?

- Hotfix to production (fix made in another branch needs to be applied asap)
- Selective feature migration
