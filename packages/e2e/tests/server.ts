// Serves one directory under a URL prefix, with the given response headers:
// the 2020 build at /, today's build under its deploy path with the headers of
// render.yaml, so a Content-Security-Policy violation shows up here first.
import { once } from "node:events";
import { createReadStream, existsSync, statSync } from "node:fs";
import { type RequestListener, createServer } from "node:http";
import { extname, join, normalize, sep } from "node:path";

export const ROOT = join(import.meta.dirname, "../../..");
export const LEGACY = join(import.meta.dirname, "../fixtures/legacy-dist");
/** Today's page, built for its deploy path by the `ready` of apps/web. */
export const WEB = join(ROOT, "apps/web/dist");
export const BASE = "/loic-truchot/projects/haiku-generator/";

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

const handler =
  (mount: Mount): RequestListener =>
  (req, res) => {
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
  };

// IPv6 loopback second: it still has ports when a runaway process has taken
// every IPv4 one (`listen EADDRINUSE` on port 0).
const HOSTS = ["127.0.0.1", "[::1]"] as const;

const listen = async (mount: Mount, host: string): Promise<Site | undefined> => {
  const server = createServer(handler(mount));
  server.listen(0, host.replaceAll(/[[\]]/gu, ""));
  try {
    await once(server, "listening");
  } catch {
    return undefined;
  }
  const address = server.address();
  if (address === null || typeof address === "string") throw new Error("no server port");
  return {
    url: `http://${host}:${address.port}${mount.prefix}`,
    close: () => {
      server.close();
    },
  };
};

/** Start the server; `url`: `http://<loopback>:<port><prefix>`. */
export const serve = async (mount: Mount): Promise<Site> => {
  const [first, second] = HOSTS;
  const site = (await listen(mount, first)) ?? (await listen(mount, second));
  if (site === undefined) throw new Error("no free port on the loopback interfaces");
  return site;
};
