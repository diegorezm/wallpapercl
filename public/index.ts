type Wallpaper = {
  name: string;
  path: string;
  dataURL: string;
}

type WallpaperMode = 'zoom' | 'stretch' | 'maximize' | 'center';

type Config = {
  mode: WallpaperMode;
  currentDirectory: string;
  currentWallpaper: Wallpaper;
}

async function fetchWallpapers() {
  type response = {
    name: string;
    path: string;
  }

  const response = await fetch('/api/wallpapers', {
    method: "GET"
  });
  if (!response.ok) {
    console.error(response.statusText);
    return []
  }
  let data;
  try {
    data = await response.json() as response[];
    return data.map((d) => {
      const url = `${window.location.origin}/images/${d.name}`;
      return {
        name: d.name,
        path: d.path,
        dataURL: url.toString(),
      }
    });
  } catch (error) {
    console.error("Error parsing JSON:", error);
    return [];
  }
}

async function applyWallpaper(wallpaper: Wallpaper) {
  const response = await fetch('/api/wallpapers', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(wallpaper),
  });
  if (!response.ok) {
    console.error(response.statusText);
    return false
  }
  return true
}

async function fetchConfig() {
  const response = await fetch('/api/config');
  if (!response.ok) {
    console.error(response.statusText);
    return undefined
  }
  const data = await response.json();
  return data as Config
}


async function changeMode(mode: WallpaperMode) {
  const response = await fetch('/api/config', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({mode}),
  });
  if (!response.ok) {
    console.error(response.statusText);
    return false
  }
  return true
}

const wallapperCardTemplate = ({dataURL, name, path}: Wallpaper) => {
  return `
    <li class="card bg-base-200 shadow-lg rounded-md w-full h-96 md:w-[350px]  2xl:w-full  flex flex-col  max-w-full">
    <figure>
      <img src=${dataURL} alt=${name} className="h-full object-cover rounded-t-md" />
    </figure>
    <div class="card-body">
      <h5 class="card-title truncate">${name}</h5>
      <p class="truncate">${path}</p>
      <button class="btn btn-primary btn-sm rounded-md apply-btn" data-name="${name}" data-path="${path}" data-url="${dataURL}">
        Apply
      </button>
    </div>
  </li>
  `
}


const wallpaperListEl = document.getElementById("wallpaper-list") as HTMLUListElement
const searchInputEl = document.getElementById("search-input") as HTMLInputElement
const selectEl = document.getElementById("mode-selection") as HTMLSelectElement


const buildWallpaperList = (wallpapers: Wallpaper[]) => {
  wallpaperListEl.innerHTML = "";
  if (wallpapers.length === 0) {
    wallpaperListEl.innerHTML = "No wallpapers found.";
    return;
  }

  wallpapers.forEach((w) => {
    const li = document.createElement("li");
    li.innerHTML = wallapperCardTemplate(w);
    wallpaperListEl.appendChild(li);
  });

  // Attach event listeners for "Apply" buttons
  const applyButtons = wallpaperListEl.querySelectorAll(".apply-btn");
  applyButtons.forEach((button) => {
    button.addEventListener("click", async (e) => {
      const target = e.target as HTMLButtonElement;
      const wallpaper = {
        name: target.dataset.name!,
        path: target.dataset.path!,
        dataURL: target.dataset.url!,
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

window.onload = async () => {
  const modes = ['zoom', 'stretch', 'maximize', 'center'] as const;
  const config = await fetchConfig()

  modes.forEach((e) => {
    const option = document.createElement("option")
    option.value = e
    option.innerHTML = e
    if (e === config?.mode) {
      option.setAttribute("selected", "true")
    }
    selectEl.appendChild(option)
  })

  selectEl.addEventListener("change", () => {
    changeMode(selectEl.value as WallpaperMode)
  })

  const wallpapers = await fetchWallpapers()

  buildWallpaperList(wallpapers)

  searchInputEl.oninput = (e) => {
    e.preventDefault()
    if (searchInputEl.value !== "") {
      const w = wallpapers.filter((e) => e.name.includes(searchInputEl.value))
      buildWallpaperList(w)
    } else {
      buildWallpaperList(wallpapers)
    }
  }
}


