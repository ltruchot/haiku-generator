// What a page looks like, as comparable data: its DOM tree, the computed style
// and box of every element, and a full-page screenshot. Two builds render the
// same when the three are equal. Asset URLs are reduced to their file name
// without content hash, so a bundler change does not count as a difference.
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import type { Browser, Page } from "playwright";
import { type Viewport, open } from "./page.ts";
import { ROOT } from "./server.ts";

/** A haiku is random: every capture shows this one instead. */
const FIXED_HAIKU = "Les cerfs sont immenses<br>une lune rousse se lève<br>le silence du lac";

export const STATES = ["day", "night", "modal"] as const;
export type State = (typeof STATES)[number];

export type Fingerprint = {
  /** One line per element or text node, indented by depth. */
  readonly dom: readonly string[];
  /** Element path -> property -> value; the box is under `rect.*`. */
  readonly layout: Readonly<Record<string, Readonly<Record<string, string>>>>;
  readonly shot: Buffer;
};

/* Runs in the page: one function, with no reference to anything outside its body. */
// oxlint-disable-next-line max-lines-per-function
const read = (): Pick<Fingerprint, "dom" | "layout"> => {
  const HASH = /\.[\w-]{8}(?=\.[a-z\d]+$)/iu;
  const fileName = (path: string): string => (path.split(/[\\/]/u).at(-1) ?? "").replace(HASH, "");
  const assetUrl = (value: string): string =>
    /^(?:[a-z][a-z\d+.-]*:|#)/iu.test(value) ? value : `asset:${fileName(value)}`;
  // Scripts and style sheets are how a build delivers the page, not the page.
  // oxlint-disable-next-line unicorn/consistent-function-scoping -- must stay in the page function
  const isPlumbing = (element: Element): boolean =>
    element.tagName === "SCRIPT" ||
    element.tagName === "STYLE" ||
    (element.tagName === "LINK" &&
      /stylesheet|modulepreload/u.test(element.getAttribute("rel") ?? "")) ||
    (element.tagName === "title" && element.parentElement?.tagName === "svg");
  // The 2020 minifier rewrote the icons: path data reshaped (a thousandth of
  // a pixel), `role`, `<title>` and a redundant `viewBox` dropped. The source
  // keeps them (they name the links); the shapes are compared by the screenshots.
  // oxlint-disable-next-line unicorn/consistent-function-scoping -- must stay in the page function
  const isIconDetail = (element: Element, name: string): boolean =>
    (element.tagName === "svg" && ["role", "viewBox"].includes(name)) ||
    (element.tagName === "path" && name === "d");
  const attributes = (element: Element): string =>
    [...element.attributes]
      // Inline styles are compared as computed styles.
      .filter((attribute) => attribute.name !== "style" && !isIconDetail(element, attribute.name))
      .map((attribute) => {
        const value = ["href", "src"].includes(attribute.name)
          ? assetUrl(attribute.value)
          : attribute.value;
        return `${attribute.name}="${value.replaceAll(/\s+/gu, " ").trim()}"`;
      })
      .toSorted()
      .join(" ");

  const dom: string[] = [];
  const layout: Record<string, Record<string, string>> = {};
  const walk = (element: Element, path: string, depth: number): void => {
    const indent = "  ".repeat(depth);
    dom.push(`${indent}<${element.tagName.toLowerCase()} ${attributes(element)}>`);
    if (path.startsWith("html/body") && element.tagName !== "path") {
      const style = getComputedStyle(element);
      const box = element.getBoundingClientRect();
      const entry: Record<string, string> = {
        "rect.x": String(box.x),
        "rect.y": String(box.y),
        "rect.width": String(box.width),
        "rect.height": String(box.height),
      };
      for (let index = 0; index < style.length; index += 1) {
        const property = style.item(index);
        entry[property] = style
          .getPropertyValue(property)
          .replaceAll(
            /url\("([^"]*)"\)/gu,
            (_match, url: string) => `url(${fileName(new URL(url).pathname)})`,
          );
      }
      layout[path] = entry;
    }
    let index = 0;
    for (const child of element.childNodes) {
      if (child instanceof Element) {
        if (!isPlumbing(child)) {
          walk(child, `${path}/${child.tagName.toLowerCase()}[${index}]`, depth + 1);
          index += 1;
        }
      } else if (child.nodeType === Node.TEXT_NODE) {
        const text = (child.textContent ?? "").replaceAll(/\s+/gu, " ").trim();
        if (text !== "") dom.push(`${indent}  "${text}"`);
      }
    }
  };
  walk(document.documentElement, "html", 0);
  return { dom, layout };
};

const enter = async (page: Page, state: State): Promise<void> => {
  if (state === "night") await page.click("#btn-toggle-nightmode");
  if (state === "modal") await page.click("#question-link");
};

export const capture = async (
  browser: Browser,
  url: string,
  viewport: Viewport,
  state: State,
): Promise<Fingerprint> => {
  const { page, close } = await open(browser, url, viewport);
  try {
    await page.evaluate((html) => {
      const haiku = document.querySelector("#haiku");
      if (haiku !== null) haiku.innerHTML = html;
    }, FIXED_HAIKU);
    await enter(page, state);
    await page.waitForLoadState("networkidle");
    await page.evaluate(async () => {
      await document.fonts.ready;
      await Promise.all(
        [...document.images].map(async (image) => {
          try {
            await image.decode();
          } catch {
            // A broken image has nothing to wait for.
          }
        }),
      );
    });
    const shot = await page.screenshot({ fullPage: true, animations: "disabled", caret: "hide" });
    return { ...(await page.evaluate(read)), shot };
  } finally {
    await close();
  }
};

/** Differences between two fingerprints, readable, at most `limit`. */
export const differences = (
  expected: Fingerprint,
  actual: Fingerprint,
  limit = 25,
): readonly string[] => {
  const found: string[] = [];
  const lines = Math.max(expected.dom.length, actual.dom.length);
  for (let index = 0; index < lines; index += 1) {
    const [before, after] = [expected.dom[index], actual.dom[index]];
    if (before !== after) {
      found.push(
        `dom line ${index + 1}: ${before?.trim() ?? "(none)"} -> ${after?.trim() ?? "(none)"}`,
      );
    }
  }
  for (const path of new Set([...Object.keys(expected.layout), ...Object.keys(actual.layout)])) {
    const [before, after] = [expected.layout[path] ?? {}, actual.layout[path] ?? {}];
    for (const property of new Set([...Object.keys(before), ...Object.keys(after)])) {
      if (before[property] !== after[property]) {
        found.push(
          `${path} ${property}: ${before[property] ?? "(none)"} -> ${after[property] ?? "(none)"}`,
        );
      }
    }
  }
  if (!expected.shot.equals(actual.shot)) found.push("screenshots differ");
  return found.length > limit
    ? [...found.slice(0, limit), `... ${found.length - limit} more`]
    : found;
};

/** Keep a screenshot for the eye: .local/screenshots/<name>.png. */
export const keep = (name: string, shot: Buffer): void => {
  const dir = join(ROOT, ".local/screenshots");
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, `${name}.png`), shot);
};
