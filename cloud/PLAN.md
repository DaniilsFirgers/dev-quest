## Cloud + System Design Learning Plan (Weekly)

| Week(s) | Main Focus                     | Key Topics                                                                                                 | Hands-On Work                                                                                  | Questions You Should Be Able To Answer                                                                                           | Deliverable                                   |
| ------- | ------------------------------ | ---------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------- |
| 1–2     | Networking Foundations         | VPC, subnets, routing, NAT, Internet Gateway, DNS, TLS/HTTPS, load balancers, public vs private networking | Build app with public subnet + private DB subnet, configure load balancer + HTTPS              | Why are DBs private? How does a request travel through cloud networking? Where does TLS terminate? What does a load balancer do? | Architecture diagram of full system           |
| 3–4     | IAM + Secrets Management       | IAM roles vs users, least privilege, temporary credentials, service-to-service auth, secrets rotation      | Use Secrets Manager / Parameter Store, implement GitHub Actions OIDC, remove hardcoded secrets | Why are long-lived credentials dangerous? How does CI securely access cloud? What is least privilege?                            | Secure CI/CD pipeline (no static secrets)     |
| 5–6     | Observability                  | Logs vs metrics vs traces, structured logging, dashboards, alerting, SLOs/SLIs                             | Add centralized logging, metrics dashboards, distributed tracing, alerts for latency/errors    | How do you debug production issues? When use logs vs metrics vs traces? What should trigger alerts?                              | Production monitoring dashboard               |
| 7–8     | System Design Fundamentals     | Scaling, caching, queues, retries, idempotency, rate limiting, replication, eventual consistency           | Add Redis cache, background queue, retry logic, rate limiting                                  | What breaks at 10x traffic? What should be async? Why is idempotency important? Where are bottlenecks?                           | 10x traffic scaling plan document             |
| 9–10    | Infrastructure as Code         | Terraform, state, modules, environments, reproducibility, drift prevention                                 | Terraform full stack: networking, compute, DB, IAM, monitoring                                 | Why use IaC? What is Terraform state? How do you structure environments?                                                         | Reproducible infrastructure repo              |
| 11–12   | Advanced Architecture Thinking | Failure modes, resilience, autoscaling, deployment strategies, cost optimization, single points of failure | Simulate failures (DB outage, latency spikes), test deployments, improve resilience            | What fails first? What is blast radius? How do systems recover? Where are single points of failure?                              | Architecture review + resilience improvements |

---

### Week 1

TODO:
[X] Set up EC2 instance in AWS
[X] Set up SSH into EC2 instance
[X] Get basic api to the EC2
[X] Configure ALB (Application load balancing) in AWS
[X] ALB to DNS mapping for a proper domain name

1. Created an **EC2 instance** of instance type and configured a **security group** to allow access. _Purpose_: this is where the app runs.
2. Created an `express.js` with a `/health` endpoint for ALB check. The app shoudl listen on `0.0.0.0:3000`.
3. In EC2 security group allowed inbound SSH (port 22) from a single home IP._Purpose_: Security groups control **who can access the app and ports**.
4. Restricted inbound HTTP traffic to the app only from ALB via security group, accepting traffic on port 3000 only from the ALB’s security group, preventing direct public internet access to the EC2 instance.
5. Created a `target group`, which defines where the traffic goes. _Purpose_: This acts as a **list of backend servers for the load balancer**:
   - Target type: **Instance**
   - Protocol: HTTP
   - VPC: same as EC2
   - Health check at `/health` to check if app is alive
6. Created an Application load balancer. _Purpose_: ALB is the **public entry point** for my application:
   - Internet-facing ALB
   - Selected at least 2 Availability Zones
7. Everything was placed inside a **VPC (Virtual Private Cloud)**.
8. Got a proper domain mapping for ALB DNS (A alias)

```
                    ┌─────────────────────────────┐
                    │        Internet             │
                    │   (Browser / Client)        │
                    └────────────┬────────────────┘
                                 │ HTTP :80
                                 ▼
        ┌────────────────────────────────────────────┐
        │   Application Load Balancer (ALB)          │
        │   - Public entry point                     │
        │   - Security Group: allow 80 from internet │
        │   - Health checks enabled                  │
        └────────────────────┬───────────────────────┘
                             │ forwards traffic
                             │ only if healthy
                             ▼
        ┌────────────────────────────────────────────┐
        │           Target Group (Instance)          │
        │   - Port: 3000                             │
        │   - Health check: /health                  │
        │   - Contains EC2 instance(s)               │
        └────────────────────┬───────────────────────┘
                             │ private traffic (SG rule)
                             ▼
        ┌────────────────────────────────────────────┐
        │        EC2 Instance (Ubuntu + Node.js)     │
        │   - Runs API on port 3000                  │
        │   - NOT publicly accessible                │
        │   - Only ALB can reach it                  │
        └────────────────────────────────────────────┘
```

- To copy the app to the remote EC2 instance use:

```
rsync -avz --exclude node_modules cloud/app/ ec2-user@<your-ec2-ip>:~/app/
```

### Week 2

TODO:

- [x] Add HTTPS via AWS certificate manager
- [ ] Different types of records (in hosted zones)
- [ ] Add a second EC2 instance for manual scaling
- [ ] Add auto scaling via Auto Scaling Group
- [x] CloudWatch for observability
- [ ] Network fundamentals

1. Configured **AWS Certificate Manager (ACM)**. Certificate must be in the same region as ALB.
2. Configured ALB HTTPS. Added an HTTPS listener and attached ACM certificate. Added HTTP (80) -> redirect to HTTPS (443)
3. Configured a mini dashboard in CloudWatch to see ReqeustsCounts, TargetResponseTime, HTTP_Code_Target_4XX_Count. Attached an alert rule on HTTP_Code_Target_4XX_Count via **CloudWatch Alarms** to send an email upon a violation.
4. Configured IAM for logging of EC2 instance - **what AWS services it can access**. Instead of access keys, EC2 used **IAM Role**. EC2 instance automatically receives credentials via the metadata service.
5. Added a logging pipeline in which a Cloudwatch agent reads reads logs from `.json` files and forwards it to the CloudWatch service.

```
                    ┌─────────────────────────────┐
                    │        Internet             │
                    │   (Browser / Client)        │
                    └────────────┬────────────────┘
                                 │ HTTP :80
                                 ▼
        ┌────────────────────────────────────────────┐
        │   Application Load Balancer (ALB)          │
        │   - Public entry point                     │
        │   - Health checks (/health)                │
        │   - Routes traffic to EC2                  │
        └────────────────────┬───────────────────────┘
                             │ private traffic
                             ▼
        ┌────────────────────────────────────────────┐
        │           EC2 Instance (Ubuntu)            │
        │--------------------------------------------│
        │  Node.js Application (Pino Logger)         │
        │        │                                   │
        │        ▼                                   │
        │   PM2 Process Manager                      │
        │   - keeps app alive                        │
        │   - restarts on crash                      │
        │   - captures stdout/stderr                 │
        │        │                                   │
        │        ▼                                   │
        │   Log Files (~/.pm2/logs/)                 │
        └────────────┬───────────────────────────────┘
                     │
                     ▼
        ┌────────────────────────────────────────────┐
        │ CloudWatch Agent                           │
        │ - reads PM2 log files                      │
        │ - ships logs to AWS                        │
        └────────────┬───────────────────────────────┘
                     │ IAM Role (no credentials)
                     ▼
        ┌────────────────────────────────────────────┐
        │ IAM Role (EC2 Instance Role)               │
        │ - temporary credentials                    │
        │ - permissions to CloudWatch Logs           │
        │ - no access keys stored                    │
        └────────────┬───────────────────────────────┘
                     ▼
        ┌────────────────────────────────────────────┐
        │ CloudWatch Logs                            │
        │ - log groups (node-app)                    │
        │ - log streams (instance_id)                │
        │ - searchable logs                          │
        └────────────────────────────────────────────┘
```
