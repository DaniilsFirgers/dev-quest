import { Router, Request, Response } from "express";
import os from "os";

export const loadBalancingRouter = Router();

loadBalancingRouter.get("/", (req: Request, res: Response) => {
  res.json({ hostname: os.hostname() });
});
