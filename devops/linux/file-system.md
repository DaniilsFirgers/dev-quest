# Linux file system commands

| Command   | Description                                                               |
| --------- | ------------------------------------------------------------------------- |
| `pwd`     | Print current working directory                                           |
| `ls`      | List files                                                                |
| `ls -a`   | List all files (hidden as well .)                                         |
| `ls-l`    | List files and directories in **long listing format**                     |
| `ls -la`  | List files and directories **together with hidden** + long listing format |
| `ls -ln`  | List file and directories **with numeric IDs**                            |
| `ls -i`   | List files with inode numbers (stores metadata)                           |
| `ls -lid` | Show inodes for directories                                               |

# File permission and ownership

- `umask` defines the default permission bits that **will be masked off**. A template for what permission bit should NOT be given automatically. For example, when new file is being created a default mode is _666_, then if `umask` is 0002 it becomes _664_. For directories it is _777_ and then would become _775_.

- Permission string has the following format:

```
-rw-r--r--
│└────────┬────────┘
│         │
│         └── 9 permission bits (3 groups × 3 bits)
└──────────── file type indicator
```

**File types**

| **Type** | **Explanation**                                                 |
| -------- | --------------------------------------------------------------- |
| `-`      | Regular file                                                    |
| `d`      | Directory                                                       |
| `l`      | Symbolic link                                                   |
| `s`      | Socket (processes talk to each other)                           |
| `b`      | Block device (transfers data in blocks like `dev/sda`)          |
| `c`      | Character device (stream of bytes like `dev/tty` or `dev/null`) |
| `p`      | Named pipe (Inter process communication)                        |

**Permission types**

| **Type** | **Explanation**                                                        |
| -------- | ---------------------------------------------------------------------- |
| `r`      | Read                                                                   |
| `w`      | Write                                                                  |
| `x`      | Execute (directories you can enter, for file you can run as a program) |
| `-`      | No permission                                                          |

**Modifying permissions**

_Owner or root can change permissions!_

1. Using symbolic mode, we specify which permission set you want to change and then use `+` to add permission and `-` to remove permission.

`chmod u+x test.txt` or `chmod g+r test.txt` or `chmod o-r test.txt`

2. Using numerical mode we can set permissions for every user simultaneously using `4` for **read (r)**, `2` for **write (w)** and `1` for **exexute (x)**. For example `755` means you give full access to the user, read and execute for the group and other.

```
sudo chown [OPTIONS] NEW_OWNER[:NEW_GROUP] FILE
```

- `sudo chown daniils test.txt` - change file owner to daniils
- `sudo chown daniils:devs test.txt` - change both user and group
- `sudo chown :devs test.txt` - change only group
- `sudo chown -R daniils:devs /home/daniils/Downloads` - recursively change ownership of all files in the directory
- `sudo chown --reference=/home/daniils/tempalate.txt newfile.txt` - copy ownership from another file

# Hierarchy

1. `/dev`
