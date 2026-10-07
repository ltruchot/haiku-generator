// What the page does, whatever build serves it. The few points where today's
// page is allowed to differ from the 2020 build are the fields of `Spec`.
import type { Browser, Page } from "playwright";
import { expect, it } from "vite-plus/test";
import { VIEWPORTS, haikuLines, open } from "./page.ts";

export type Spec = {
  /** Hosts of other sites requested on load (analytics). */
  readonly otherSitesOnLoad: readonly string[];
};

export type Given = { readonly browser: () => Browser; readonly url: () => string };

const [, DESKTOP] = VIEWPORTS;
const MODAL_HIDDEN = ["opacity-0", "pointer-events-none"];

/** "visible,clickable,body marked": the three go together. */
const modalState = async (page: Page): Promise<string> => {
  const hidden = await page
    .locator("#answers-block")
    .evaluate(
      (modal, classes) => classes.map((name) => modal.classList.contains(name)),
      MODAL_HIDDEN,
    );
  const active = await page
    .locator("body")
    .evaluate((body) => body.classList.contains("modal-active"));
  return [...hidden.map((value) => !value), active].join(",");
};

const composes = (given: Given): void => {
  it("composes a haiku of three lines on load", async () => {
    const { page, close } = await open(given.browser(), given.url(), DESKTOP);
    const lines = await haikuLines(page);
    await close();
    expect(lines).toHaveLength(3);
    expect(lines.map((line) => line.trim())).not.toContain("");
    expect(lines.join("\n")).not.toContain("#");
  });

  it("composes another haiku on demand", async () => {
    const { page, close } = await open(given.browser(), given.url(), DESKTOP);
    const seen = new Set([(await haikuLines(page)).join("/")]);
    // oxlint-disable no-await-in-loop -- one click after the other, on purpose
    for (let click = 0; click < 5; click += 1) {
      await page.click("#btn-generate-haiku");
      seen.add((await haikuLines(page)).join("/"));
    }
    // oxlint-enable no-await-in-loop
    await close();
    expect(seen.size).toBeGreaterThan(1);
  });
};

const switchesToNight = (given: Given): void => {
  it("switches to night mode and back", async () => {
    const { page, close } = await open(given.browser(), given.url(), DESKTOP);
    const look = async (): Promise<readonly unknown[]> => [
      await page.locator("#btn-toggle-nightmode").textContent(),
      await page.locator("body").evaluate((body) => body.classList.contains("text-gray-100")),
      await page
        .locator("#profile")
        .evaluate((profile) => profile.classList.contains("bg-gray-900")),
    ];
    const day = await look();
    await page.click("#btn-toggle-nightmode");
    const night = await look();
    await page.click("#btn-toggle-nightmode");
    const back = await look();
    await close();
    expect(day).toStrictEqual(["🌙", false, false]);
    expect(night).toStrictEqual(["☀️", true, true]);
    expect(back).toStrictEqual(day);
  });
};

const explains = (given: Given): void => {
  it("opens the explanations and closes them every way", async () => {
    const { page, close } = await open(given.browser(), given.url(), DESKTOP);
    const [OPEN, CLOSED] = ["true,true,true", "false,false,false"];
    const results: string[] = [await modalState(page)];
    await page.keyboard.press("Escape");
    results.push(await modalState(page));
    const closers = await page.locator(".modal-close").count();
    // oxlint-disable no-await-in-loop -- one closing control after the other, on purpose
    for (let closer = 0; closer < closers; closer += 1) {
      await page.click("#question-link");
      results.push(await modalState(page));
      await page.locator(".modal-close").nth(closer).click();
      results.push(await modalState(page));
    }
    // oxlint-enable no-await-in-loop
    await page.click("#question-link");
    await page.locator(".modal-overlay").click({ position: { x: 5, y: 5 } });
    results.push(await modalState(page));
    await page.click("#question-link");
    await page.keyboard.press("Escape");
    results.push(await modalState(page));
    await close();
    expect(closers).toBe(3);
    expect(results).toStrictEqual([
      CLOSED,
      CLOSED,
      OPEN,
      CLOSED,
      OPEN,
      CLOSED,
      OPEN,
      CLOSED,
      CLOSED,
      CLOSED,
    ]);
  });
};

/** 2020 only: the share links are gone from today's page. */
export const tweets = (given: Given): void => {
  it("tweets the haiku", async () => {
    const { page, context, external, close } = await open(given.browser(), given.url(), DESKTOP);
    await page.waitForLoadState("networkidle");
    const before = external.length;
    const text = await page.locator("#haiku").innerText();
    await page.click("#tweeter-link");
    await expect.poll(() => external.length).toBeGreaterThan(before);
    const tabs = context.pages().length;
    await close();
    const tweet = new URL(String(external.at(-1)));
    expect(`${tweet.origin}${tweet.pathname}`).toBe("https://twitter.com/intent/tweet");
    expect(tweet.searchParams.get("text")).toBe(`${text} \n\n par https://aiku.wtf`);
    // The link says target="_blank", the script navigates the page itself.
    expect(tabs).toBe(1);
  });
};

const talksToOtherSites = (spec: Spec, given: Given): void => {
  it("asks nothing of other sites on load, analytics aside", async () => {
    const { page, external, close } = await open(given.browser(), given.url(), DESKTOP);
    await page.waitForLoadState("networkidle");
    await close();
    expect(external.map((url) => new URL(url).hostname)).toStrictEqual(spec.otherSitesOnLoad);
  });
};

export const behaves = (spec: Spec, given: Given): void => {
  composes(given);
  switchesToNight(given);
  explains(given);
  talksToOtherSites(spec, given);
};
