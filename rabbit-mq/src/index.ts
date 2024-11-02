import express from "express";
import Rabbit from "./rabbit/index.js";

const app = express();

app.use(express.json());
app.use(express.urlencoded({ extended: true }));

app.post("/api/v1/publish", async (req, res) => {
  try {
    const client = await Rabbit.initClient();
    const response = await client.publish("rpc.multiply", req.body);
    res.status(200).send(response);
  } catch (error) {
    res.status(500).send(error);
  }
});

app.listen(3000, async () => {
  await Rabbit.initialize();
  console.log("Server is running on port 3000");
});
