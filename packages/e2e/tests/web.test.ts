// Today's page (apps/web/dist, built for its deploy path) against the 2020
// build: same behaviour, same fingerprint. A difference here is a regression
// unless it is one of the accepted changes listed in CLAUDE.md.
import { existsSync } from "node:fs";
import { join } from "node:path";
import { type Browser, chromium } from "playwright";
import { afterAll, beforeAll, describe, expect, it } from "vite-plus/test";
import { behaves } from "./behaviour.ts";
import { STATES, capture, differences, keep } from "./fingerprint.ts";
import { VIEWPORTS, open } from "./page.ts";
import { BASE, LEGACY, type Site, WEB, serve } from "./server.ts";

const CASES = VIEWPORTS.flatMap((viewport) => STATES.map((state) => ({ viewport, state })));
const [, DESKTOP] = VIEWPORTS;

type Compared = {
  readonly browser: () => Browser;
  readonly before: () => string;
  readonly after: () => string;
};

/* Runs in the 2020 page before it is captured: what was removed since, on purpose. */
const accepted = (): void => {
  const shareLinks = 'a[href*="facebook.com/sharer"], #tweeter-link, a[href*="short-edition.com"]';
  for (const link of document.querySelectorAll(shareLinks)) link.remove();
};

const rendersLikeBefore = (given: Compared): void => {
  it.for(CASES)(
    "renders like the 2020 build: $viewport.name, $state",
    async ({ viewport, state }) => {
      const before = await capture(given.browser(), given.before(), viewport, state, accepted);
      const after = await capture(given.browser(), given.after(), viewport, state);
      keep(`web-${viewport.name}-${state}`, after.shot);
      expect(differences(before, after)).toStrictEqual([]);
    },
  );
};

describe("today's page", () => {
  let browser: Browser;
  let legacy: Site;
  let site: Site;

  beforeAll(async () => {
    if (!existsSync(join(WEB, "index.html"))) {
      throw new Error("apps/web/dist missing: run vp run ready (or vp run web#ready)");
    }
    browser = await chromium.launch();
    legacy = await serve({ dir: LEGACY, prefix: "/", headers: {} });
    site = await serve({ dir: WEB, prefix: BASE, headers: {} });
  });

  afterAll(async () => {
    await browser.close();
    legacy.close();
    site.close();
  });

  behaves({ otherSitesOnLoad: [] }, { browser: () => browser, url: () => site.url });

  it("loads without an error in the console", async () => {
    const { page, errors, close } = await open(browser, site.url, DESKTOP);
    await page.waitForLoadState("networkidle");
    await close();
    expect(errors).toStrictEqual([]);
  });

  it("is served under its deploy path", async () => {
    const { page, close } = await open(browser, site.url, DESKTOP);
    const urls = await page.evaluate(() =>
      performance
        .getEntriesByType("resource")
        .map((entry) => new URL(entry.name))
        .filter((url) => url.origin === location.origin)
        .map((url) => url.pathname),
    );
    await close();
    expect(urls.length).toBeGreaterThan(5);
    expect(urls.filter((path) => !path.startsWith(BASE))).toStrictEqual([]);
  });

  rendersLikeBefore({ browser: () => browser, before: () => legacy.url, after: () => site.url });
});
