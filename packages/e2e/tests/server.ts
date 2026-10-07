// Serves one directory under a URL prefix, with the given response headers:
// the 2020 build at /, today's build under its deploy path with the headers of
// render.yaml, so a Content-Security-Policy violation shows up here first.
import { once } from "node:events";
import { createReadStream, existsSync, statSync } from "node:fs";
import { createServer } from "node:http";
import { extname, join, normalize, sep } from "node:path";

export const ROOT = join(import.meta.dirname, "../../..");
export const LEGACY = join(import.meta.dirname, "../fixtures/legacy-dist");

const TYPES: Readonly<Record<string, string>> = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript",
  ".css": "text/css",
  ".wasm": "application/wasm",
  ".svg": "image/svg+xml",
  ".jpg": "image/jpeg",
  ".png": "image/png",
  ".ico": "image/x-icon",
  ".webmanifest": "application/manifest+json",
  ".map": "application/json",
  ".txt": "text/plain; charset=utf-8",
};

export type Mount = {
  readonly dir: string;
  /** Starts and ends with a slash: `/`, `/loic-truchot/projects/haiku-generator/`. */
  readonly prefix: string;
  readonly headers: Readonly<Record<string, string>>;
};

export type Site = { readonly url: string; readonly close: () => void };

const fileOf = (mount: Mount, pathname: string): string | undefined => {
  if (!pathname.startsWith(mount.prefix)) return undefined;
  const rest = pathname.slice(mount.prefix.length);
  const file = normalize(
    join(mount.dir, rest === "" || rest.endsWith("/") ? `${rest}index.html` : rest),
  );
  const inside = file.startsWith(mount.dir + sep);
  return inside && existsSync(file) && statSync(file).isFile() ? file : undefined;
};

/** Start the server; `url`: `http://127.0.0.1:<port><prefix>`. */
export const serve = async (mount: Mount): Promise<Site> => {
  const server = createServer((req, res) => {
    const file = fileOf(mount, decodeURIComponent(new URL(req.url ?? "/", "http://x").pathname));
    if (file === undefined) {
      res.writeHead(404).end();
      return;
    }
    res.writeHead(200, {
      ...mount.headers,
      "Content-Type": TYPES[extname(file)] ?? "application/octet-stream",
    });
    createReadStream(file).pipe(res);
  });
  server.listen(0, "127.0.0.1");
  await once(server, "listening");
  const address = server.address();
  if (address === null || typeof address === "string") throw new Error("no server port");
  return {
    url: `http://127.0.0.1:${address.port}${mount.prefix}`,
    close: () => {
      server.close();
    },
  };
};
