import { Router } from "express";
import { authRouter } from "./auth/index.js";
import { etagRouter } from "./etag/index.js";

const apiRouter = Router();

apiRouter.use("/auth", authRouter);
apiRouter.use("/etag", etagRouter);

export default apiRouter;
