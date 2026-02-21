# Docker Swarm mode

## Common commands

| Action                    | Command                                                  |
| ------------------------- | -------------------------------------------------------- |
| Init swarm (manager node) | `docker swarm init`                                      |
| Deploy stack              | `docker stack deploy -c docker-compose.yml <stack_name>` |
| Remove stack              | `docker stack rm playground`                             |
| Check services            | `docker service ls`                                      |
| Check docker info         | `docker info`                                            |

1. Manager Node (control plane)
   A **manager node** is the brain of the Swarm Cluster (_decides what should run where_) and is responsible for:

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

  To _initialize a manager node_, run:

  ```
  docker swarm init
  ```

  _Possible command output_ is:

  ```
  Swarm initialized: current node (g2nx008im5dy2eiyex2c28xit) is now a manager.

  To add a worker to this swarm, run the following command:

      docker swarm join --token SWMTKN-1-5tn57ohzs9mwm1cwgl7dvt0qp0nha6h94miswjp0jdkpg38p4i-289gkk2ki4lnj168gvmn5e6en 192.168.1.106:2377
  ```

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

3. Networking

In Docker Swarm there are **three major networking concepts**.

- Overlay networks
- Service discovery (VIP vs DNSRR)
- Routing mesh (ingress network)

Overlay network is a **virtual network spanning multiple nodes.** Nodes communicate as if they are on the same LAN. Here **ingress** is also important as it is responsible for how traffic enters the cluster. There are two options, either use Swarm IPVS (Linux Kernel Load balancing) or use external load balancer that sits before the Docker Swarm (Nginx or HAProxy).

The advantages of **ingress** are:

- You can send traffic to any node (even if replicas of service run one node, but you call another node)
- Build-in load balancing
- Single Entry Point per Port (expose one port)
- Automatic failover

**VIP** (service-level load balancing) - virtual IP address is a single internal IP assigned to a service, representing all its replicas. Used for **container-to-container** communication. It is how Swarm does **internal load balancing**. It is set via `endpoint_mode: vip`.

4. Scheduler and placement decisions

Next:

6. Rolling Updates & Rollbacks
7. Secrets & Configs
8. Healthchecks
9. Volumes in Multi-Node Setup
10. Failure Scenarios
