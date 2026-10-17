// One page of a site in a fresh browser context, hermetic: every request to
// another origin is recorded and aborted (analytics, share links).
import type { Browser, BrowserContext, Page } from "playwright";

export const VIEWPORTS = [
  { name: "mobile", width: 375, height: 812 },
  { name: "desktop", width: 1280, height: 800 },
] as const;
export type Viewport = (typeof VIEWPORTS)[number];

export type Opened = {
  readonly page: Page;
  readonly context: BrowserContext;
  /** URLs requested outside the site, in order. */
  readonly external: readonly string[];
  /** Console errors and uncaught exceptions. */
  readonly errors: readonly string[];
  readonly close: () => Promise<void>;
};

/** The three lines of the haiku on the page, once the wasm module has answered. */
export const haikuLines = async (page: Page): Promise<readonly string[]> => {
  await page.waitForFunction(
    () => document.querySelector("#haiku")?.innerHTML.split("<br>").length === 3,
  );
  const html = await page.locator("#haiku").innerHTML();
  return html.split("<br>");
};

export const open = async (browser: Browser, url: string, viewport: Viewport): Promise<Opened> => {
  const context = await browser.newContext({
    viewport: { width: viewport.width, height: viewport.height },
    deviceScaleFactor: 1,
    reducedMotion: "reduce",
    locale: "fr-FR",
    colorScheme: "light",
  });
  const external: string[] = [];
  const errors: string[] = [];
  const { origin } = new URL(url);
  await context.route(
    (target) => target.origin !== origin,
    async (route) => {
      external.push(route.request().url());
      await route.abort();
    },
  );
  const page = await context.newPage();
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(`${message.text()} (${message.location().url})`);
  });
  page.on("pageerror", (error) => {
    errors.push(error.message);
  });
  await page.goto(url, { waitUntil: "load" });
  await haikuLines(page);
  return {
    page,
    context,
    external,
    errors,
    close: async () => {
      await context.close();
    },
  };
};
