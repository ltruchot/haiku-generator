import { generate } from './../Cargo.toml';
import "./index.scss";
const generate_haiku = () => {
    const haikuList = `<ul>${
        generate()
            .map(el => `<li>${el}</li>`)
            .join("")
    }</ul>`;
    const haikuContainer = document.getElementById("haiku");
    haikuContainer.innerHTML = haikuList;
}
const btn = document.getElementById("btn");
btn.addEventListener("click", generate_haiku);