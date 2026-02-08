import express from "express";
import routes from "./routes/index.js";

const app = express();

app.set("etag", false); // Disable default ETag generation
app.use(express.json());
app.use("/api", routes);

export default app;
