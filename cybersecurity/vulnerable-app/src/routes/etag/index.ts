import { Router, Request, Response } from "express";
import { readFileSync } from "fs";
import path from "path";

// Etag or Entity Tag is an https header taht acts as a version identifier for a resource.
// It lets clients make conditional requests and helps with caching and optimizing bandwidth.

// 1. Example response with ETag header

// HTTP/1.1 200 OK
// ETag: "686897696a7c876b7e"
// Content-Type: application/json

// 2. On susequent requests, the client can use the ETag value to check if the resource has changed:
// GET /resource HTTP/1.1
// If-None-Match: "686897696a7c876b7e"

// 3. If the resource hasn't changed, the server responds with a 304 Not Modified status:

// HTTP/1.1 304 Not Modified

// IMPORTANT: Cache-control max-age vs ETag

export const etagRouter = Router();

function createEtag(data: string): string {
  let hash = 0;
  for (let i = 0; i < data.length; i++) {
    const char = data.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash |= 0; // Convert to 32bit integer
  }
  return `"${hash.toString(16)}"`;
}

etagRouter.get("/", (req: Request, res: Response) => {
  const filePath = path.resolve("public/etag-info.txt");
  const fileBuffer = readFileSync(filePath);
  const etag = createEtag(fileBuffer.toString());

  if (req.headers["if-none-match"] === etag) {
    res.status(304).end();
    return;
  }

  res.setHeader("ETag", etag);
  res.sendFile(filePath);
});
