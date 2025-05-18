# GIT

## git add

`git add` stages changes by creating a **blob object** for the file, then stores the content in `.git/objects/` and updates the index (staging area) to include the file and its blob hash.

Can check the staged content with `git diff --cached`. Will see something like this:

```
diff --git a/README.md b/README.md
index 2fd6b91..c54cd81 100644
--- a/README.md
+++ b/README.md
@@ -1 +1 @@
-Init commit
+Init commit 2
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

# .git/

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
