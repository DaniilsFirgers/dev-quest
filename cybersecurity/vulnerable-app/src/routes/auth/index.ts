import { Router, Request, Response } from "express";

export const authRouter = Router();

authRouter.get("/", (req: Request, res: Response) => {
  res.json({ status: "ok" });
});
