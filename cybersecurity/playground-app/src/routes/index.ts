import { Router } from "express";
import { loadBalancingRouter } from "./auth/index.js";
import { etagRouter } from "./etag/index.js";

const apiRouter = Router();

apiRouter.use("/load-balancing", loadBalancingRouter);
apiRouter.use("/etag", etagRouter);

export default apiRouter;
