// Today's page (apps/web/dist, built for its deploy path) against the 2020
// build: same behaviour, same fingerprint. A difference here is a regression
// unless it is one of the accepted changes listed in CLAUDE.md.
import { existsSync } from "node:fs";
import { join } from "node:path";
import { type Browser, chromium } from "playwright";
import { afterAll, beforeAll, describe, expect, it } from "vite-plus/test";
import { type Given, behaves } from "./behaviour.ts";
import { STATES, capture, differences, keep } from "./fingerprint.ts";
import { VIEWPORTS, open } from "./page.ts";
import { BASE, LEGACY, type Site, WEB, renderHeaders, serve } from "./server.ts";

const CASES = VIEWPORTS.flatMap((viewport) => STATES.map((state) => ({ viewport, state })));
const [, DESKTOP] = VIEWPORTS;

type Compared = {
  readonly browser: () => Browser;
  readonly before: () => string;
  readonly after: () => string;
};

/* Runs in the 2020 page before it is captured: what changed since, on purpose. */
const accepted = (): void => {
  const shareLinks = 'a[href*="facebook.com/sharer"], #tweeter-link, a[href*="short-edition.com"]';
  for (const link of document.querySelectorAll(shareLinks)) link.remove();
  // The image of a shared link: aiku.wtf is gone, the page has a new address.
  document
    .querySelector('meta[property="og:image"]')
    ?.setAttribute(
      "content",
      "https://developers.gods.academy/loic-truchot/projects/haiku-generator/robot-profil.jpg",
    );
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

const deploys = (given: Given): void => {
  it("loads without an error in the console", async () => {
    const { page, errors, close } = await open(given.browser(), given.url(), DESKTOP);
    await page.waitForLoadState("networkidle");
    await close();
    expect(errors).toStrictEqual([]);
  });

  it("is served with the security headers of render.yaml", async () => {
    const { page, close } = await open(given.browser(), given.url(), DESKTOP);
    const response = await page.request.get(given.url());
    await close();
    expect(response.headers()["content-security-policy"]).toContain("'wasm-unsafe-eval'");
    expect(Object.keys(renderHeaders())).toHaveLength(6);
  });

  it("is served under its deploy path", async () => {
    const { page, close } = await open(given.browser(), given.url(), DESKTOP);
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

  it("publishes the image of a shared link at its og:image address", async () => {
    const { page, close } = await open(given.browser(), given.url(), DESKTOP);
    const image = await page.locator('meta[property="og:image"]').getAttribute("content");
    const response = await page.request.get(new URL("robot-profil.jpg", given.url()).href);
    await close();
    expect(image).toBe(`https://developers.gods.academy${BASE}robot-profil.jpg`);
    expect(response.status()).toBe(200);
  });
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
    site = await serve({ dir: WEB, prefix: BASE, headers: renderHeaders() });
  });

  afterAll(async () => {
    await browser.close();
    legacy.close();
    site.close();
  });

  behaves({ otherSitesOnLoad: [] }, { browser: () => browser, url: () => site.url });

  deploys({ browser: () => browser, url: () => site.url });
  rendersLikeBefore({ browser: () => browser, before: () => legacy.url, after: () => site.url });
});
