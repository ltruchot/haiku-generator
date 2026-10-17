import "./index.scss";
import init, { generate } from "./wasm/haiku_generator.js";
import wasmUrl from "./wasm/haiku_generator_bg.wasm?url";

const byId = (id: string): HTMLElement => {
  const element = document.querySelector<HTMLElement>(`#${id}`);
  if (element === null) throw new Error(`no #${id} in the page`);
  return element;
};

// Nothing below runs before the wasm module answers, as in the 2020 build.
await init({ module_or_path: wasmUrl });

// haiku generator
const haiku = byId("haiku");
const generateHaiku = (): void => {
  haiku.innerHTML = generate().join("<br/>");
};
generateHaiku();
byId("btn-generate-haiku").addEventListener("click", generateHaiku);

// night mode toggle
const toggle = byId("btn-toggle-nightmode");
const profile = byId("profile");
toggle.addEventListener("click", () => {
  const { body } = document;
  if (body.classList.contains("text-gray-900")) {
    toggle.innerHTML = "☀️";
    body.classList.remove("text-gray-900");
    body.classList.add("text-gray-100");
    profile.classList.remove("bg-white");
    profile.classList.add("bg-gray-900");
  } else {
    toggle.innerHTML = "🌙";
    body.classList.remove("text-gray-100");
    body.classList.add("text-gray-900");
    profile.classList.remove("bg-gray-900");
    profile.classList.add("bg-white");
  }
});

// question toggle
const toggleModal = (): void => {
  const modal = document.querySelector(".modal");
  modal?.classList.toggle("opacity-0");
  modal?.classList.toggle("pointer-events-none");
  document.body.classList.toggle("modal-active");
};
byId("question-link").addEventListener("click", (event) => {
  event.preventDefault();
  toggleModal();
});
document.querySelector(".modal-overlay")?.addEventListener("click", toggleModal);
for (const closer of document.querySelectorAll(".modal-close")) {
  closer.addEventListener("click", toggleModal);
}
document.addEventListener("keydown", (event) => {
  const isEscape = event.key === "Escape" || event.key === "Esc";
  if (isEscape && document.body.classList.contains("modal-active")) toggleModal();
});
