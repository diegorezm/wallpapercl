/**
 * @typedef {Object} Wallpaper
 * @property {string} name - The name of the wallpaper.
 * @property {string} path - The file path of the wallpaper.
 * @property {string} dataURL - The URL to access the wallpaper image.
 */

/**
 * @typedef {'zoom' | 'stretch' | 'maximize' | 'center'} WallpaperMode
 */

/**
 * @typedef {Object} Config
 * @property {WallpaperMode} mode - The display mode of the wallpaper.
 * @property {string} currentDirectory - The current directory of wallpapers.
 * @property {Wallpaper} currentWallpaper - The currently applied wallpaper.
 */

/**
 * Fetches the list of available wallpapers from the API.
 * @returns {Promise<Wallpaper[]>} A promise that resolves to an array of Wallpaper objects.
 */
async function fetchWallpapers() {
  try {
    const response = await fetch('/api/wallpapers', {method: "GET"});
    if (!response.ok) {
      console.error(response.statusText);
      return [];
    }
    const data = await response.json();
    return data.map(d => ({
      name: d.name,
      path: d.path,
      dataURL: `${window.location.origin}/images/${d.name}`,
    }));
  } catch (error) {
    console.error("Error parsing JSON:", error);
    return [];
  }
}

/**
 * Applies a wallpaper by sending it to the API.
 * @param {Wallpaper} wallpaper - The wallpaper to be applied.
 * @returns {Promise<boolean>} A promise that resolves to `true` if successful, `false` otherwise.
 */
async function applyWallpaper(wallpaper) {
  try {
    const response = await fetch('/api/wallpapers', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify(wallpaper),
    });
    if (!response.ok) {
      console.error(response.statusText);
      return false;
    }
    return true;
  } catch (error) {
    console.error("Error applying wallpaper:", error);
    return false;
  }
}

/**
 * Fetches the configuration settings from the API.
 * @returns {Promise<Config | undefined>} A promise that resolves to the Config object or `undefined` if an error occurs.
 */
async function fetchConfig() {
  try {
    const response = await fetch('/api/config');
    if (!response.ok) {
      console.error(response.statusText);
      return undefined;
    }
    return await response.json();
  } catch (error) {
    console.error("Error fetching config:", error);
    return undefined;
  }
}

/**
 * Changes the wallpaper mode by sending it to the API.
 * @param {WallpaperMode} mode - The mode to be applied.
 * @returns {Promise<boolean>} A promise that resolves to `true` if successful, `false` otherwise.
 */
async function changeMode(mode) {
  try {
    const response = await fetch('/api/config', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify({mode}),
    });
    if (!response.ok) {
      console.error(response.statusText);
      return false;
    }
    return true;
  } catch (error) {
    console.error("Error changing mode:", error);
    return false;
  }
}

/**
 * Generates an HTML template for a wallpaper card.
 * @param {Wallpaper} wallpaper - The wallpaper data.
 * @returns {string} The HTML string for the wallpaper card.
 */
const wallapperCardTemplate = ({dataURL, name, path}) => `
  <li class="card bg-base-200 shadow-lg rounded-md w-full h-96 md:w-[350px]  2xl:w-full flex flex-col max-w-full">
    <figure>
      <img src="${dataURL}" alt="${name}" className="object-cover h-full rounded-t-md" />
    </figure>
    <div class="card-body">
      <h5 class="truncate card-title">${name}</h5>
      <p class="truncate">${path}</p>
      <button class="btn btn-primary btn-sm rounded-md apply-btn" data-name="${name}" data-path="${path}" data-url="${dataURL}">
        Apply
      </button>
    </div>
  </li>
`;

/**
 * Builds and renders the wallpaper list.
 * @param {Wallpaper[]} wallpapers - Array of wallpapers to display.
 */
const buildWallpaperList = wallpapers => {
  wallpaperListEl.innerHTML = "";
  if (wallpapers.length === 0) {
    wallpaperListEl.innerHTML = "No wallpapers found.";
    return;
  }
  wallpapers.forEach(w => {
    const li = document.createElement("li");
    li.innerHTML = wallapperCardTemplate(w);
    wallpaperListEl.appendChild(li);
  });

  // Attach event listeners to "Apply" buttons
  const applyButtons = wallpaperListEl.querySelectorAll(".apply-btn");
  applyButtons.forEach(button => {
    button.addEventListener("click", async e => {
      const target = e.target;
      const wallpaper = {
        name: target.dataset.name,
        path: target.dataset.path,
        dataURL: target.dataset.url,
      };
      const success = await applyWallpaper(wallpaper);
      if (success) {
        console.log(`Applied wallpaper: ${wallpaper.name}`);
      } else {
        console.error("Failed to apply wallpaper");
      }
    });
  });
};

// DOM elements
const wallpaperListEl = document.getElementById("wallpaper-list");
const searchInputEl = document.getElementById("search-input");
const selectEl = document.getElementById("mode-selection");

// Initialize the application
window.onload = async () => {
  const modes = ['zoom', 'stretch', 'maximize', 'center'];
  const config = await fetchConfig();

  modes.forEach(mode => {
    const option = document.createElement("option");
    option.value = mode;
    option.innerHTML = mode;
    if (mode === config?.mode) {
      option.setAttribute("selected", "true");
    }
    selectEl.appendChild(option);
  });

  selectEl.addEventListener("change", () => {
    changeMode(selectEl.value);
  });

  const wallpapers = await fetchWallpapers();
  buildWallpaperList(wallpapers);

  searchInputEl.oninput = e => {
    e.preventDefault();
    const filteredWallpapers = searchInputEl.value
      ? wallpapers.filter(w => w.name.includes(searchInputEl.value))
      : wallpapers;
    buildWallpaperList(filteredWallpapers);
  };
};
