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
