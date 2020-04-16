import { generate} from './../Cargo.toml';
import "./index.scss";
import { version } from "./package.json";
console.log(`parcel build: ${version}`);

// haiku generator
const generate_haiku = () => {
    const haiku = generate().join("<br/>");
    const haikuContainer = document.getElementById("haiku");
    haikuContainer.innerHTML = haiku;
}
generate_haiku();
const btn = document.getElementById("btn-generate-haiku");
btn.addEventListener("click", generate_haiku);

// night mode toggle
const toggle = document.getElementById('btn-toggle-nightmode');
const body = document.body;
const profile = document.getElementById('profile');

toggle.addEventListener('click', () => {

  if (body.classList.contains('text-gray-900')) {
    toggle.innerHTML = "☀️";
    body.classList.remove('text-gray-900');
    body.classList.add('text-gray-100');
    profile.classList.remove('bg-white');
    profile.classList.add('bg-gray-900');
  } else
  {
    toggle.innerHTML = "🌙";
    body.classList.remove('text-gray-100');
    body.classList.add('text-gray-900');
    profile.classList.remove('bg-gray-900');			
    profile.classList.add('bg-white');
    
  }
});

// twitter copy
const tweet = document.getElementById('tweeter-link');
tweet.addEventListener("click", (e) => {
    e.preventDefault();
    const href = tweet.getAttribute("href");
    const root = href.split("?")[0];
    const url = root + "?text=" +encodeURIComponent(haiku.innerText + " \n\n par https://aiku.wtf");
    window.location = url;
    return false;
});