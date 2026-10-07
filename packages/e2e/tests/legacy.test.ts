// The 2020 build (fixtures/legacy-dist, served at / as it was on aiku.wtf) is
// the reference of every comparison. These tests pin what it does, and prove
// that a fingerprint is stable: the same build captured twice gives the same.
import { type Browser, chromium } from "playwright";
import { afterAll, beforeAll, describe, expect, it } from "vite-plus/test";
import { behaves } from "./behaviour.ts";
import { STATES, capture, differences, keep } from "./fingerprint.ts";
import { VIEWPORTS } from "./page.ts";
import { LEGACY, type Site, serve } from "./server.ts";

const CASES = VIEWPORTS.flatMap((viewport) => STATES.map((state) => ({ viewport, state })));

describe("the 2020 build", () => {
  let browser: Browser;
  let site: Site;

  beforeAll(async () => {
    browser = await chromium.launch();
    site = await serve({ dir: LEGACY, prefix: "/", headers: {} });
  });

  afterAll(async () => {
    await browser.close();
    site.close();
  });

  behaves(
    {
      otherSitesOnLoad: ["www.googletagmanager.com"],
      tweetSuffix: " \n\n par https://aiku.wtf",
      tabsAfterTweet: 1,
    },
    { browser: () => browser, url: () => site.url },
  );

  it.for(CASES)("renders the same twice: $viewport.name, $state", async ({ viewport, state }) => {
    const first = await capture(browser, site.url, viewport, state);
    const second = await capture(browser, site.url, viewport, state);
    keep(`legacy-${viewport.name}-${state}`, first.shot);
    expect(differences(first, second)).toStrictEqual([]);
    expect(first.dom.length).toBeGreaterThan(100);
  });
});
