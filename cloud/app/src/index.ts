import Fastify from "fastify";

const app = Fastify({ logger: true });

const QUOTES = [
  "Code is like humor. When you have to explain it, it's bad. – Cory House",
  "First, solve the problem. Then, write the code. – John Johnson",
  "Any fool can write code that a computer can understand. Good programmers write code that humans can understand. – Martin Fowler",
  "Simplicity is the soul of efficiency. – Austin Freeman",
  "Make it work, make it right, make it fast. – Kent Beck",
];

app.get("/health", async (_req, reply) => {
  return reply.status(200).send({ status: "ok" });
});

app.get("/quote", async (_req, reply) => {
  const quote = QUOTES[Math.floor(Math.random() * QUOTES.length)];
  return reply.send({ quote });
});

app.listen({ port: 3000, host: "0.0.0.0" }, (err) => {
  if (err) {
    app.log.error(err);
    process.exit(1);
  }
});
