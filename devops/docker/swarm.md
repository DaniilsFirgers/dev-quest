# Docker Swarm mode

1. Manager Node (control plane)
   A **manager node** is the brain of the Swarm Cluster (_decides what shoudl run where_) and is responsible for:

- Maintaining cluster state
- Scheduling services and tasks
- Handling updates and scaling

  They are usually set to **drain mode**, so application containers are run by **workers** instead.

  **Raft consensus algorithm** is used to agree on schedule decisions, store cluster state and maintain consitency. Cluster state includes info about:

- Which nodes exist
- Which services exist
- Desired replica count
- Secrets and configs

  Typically you have **3 or 5 managers**. **1 is bad**, because if it fails then it is readoonly for existing containers. **2 is also bad**, because majority is 2 and if 1 dies, no majority is possible, and again read-only.

  Swarm chooses **consistence over availability**, which is part of CAP theorem. Chooses _Consistency over availability during partitions_.

2. Worker Node (data plane)
   Is a machine that runs containers, executes what managers decide, but does NOT participate in consensus and does NOT store cluster state.

   If a worker node dies, then tasks on that worker are marked as failed and manager reschedules them in healthy workers. That is **task rescheduling**.

   To check worker node state use:

   ```
   docker node ls

   ```

   Possible output:

   ```
   ID                            HOSTNAME        STATUS    AVAILABILITY   MANAGER STATUS
   x1y2z3abc                     manager-1       Ready     Active         Leader
   a4b5c6def                     worker-1        Ready     Active
   g7h8i9jkl                     worker-2        Ready     Active

   ```
