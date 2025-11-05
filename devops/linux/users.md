# Users

| Range | Purpose                                            |
| ----- | -------------------------------------------------- |
| 0     | `root`                                             |
| 1-999 | `daemon`, `bin`, `sys` - system/serice accounts    |
| 1000+ | Normal (human) users like `ubuntu`, `alice`, `bob` |
| 65534 | `nobody` user (unpriviledged)                      |

**Why separate ranges?**

- System deamons need account to run under, but they do not need to overlap with normal users.
- Otherwise a normal user could interfere with system processes.

To view users info use `cat /etc/passwd`:

```
hplip:x:127:7:HPLIP system user,,,:/run/hplip:/bin/false
gdm:x:128:134:Gnome Display Manager:/var/lib/gdm3:/bin/false
daniiils:x:1000:1000:Daniiils,,,:/home/daniiils:/bin/bash
mongodb:x:129:65534::/home/mongodb:/usr/sbin/nologin
sshd:x:130:65534::/run/sshd:/usr/sbin/nologin
postfix:x:131:140::/var/spool/postfix:/usr/sbin/nologin
epmd:x:132:142::/run/epmd:/usr/sbin/nologin
rabbitmq:x:133:143:RabbitMQ messaging server,,,:/var/lib/rabbitmq:/usr/sbin/nologin

```

## Add user

- To add user use the following command:

```
sudo adduser <USERNAME>
```

By default each user **gets own unique primary group** named the same as user:

```
user:   alice
group:  alice
UID:    1001
GID:    1001

```
