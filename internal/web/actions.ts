export type Wallpaper = {
  name: string;
  path: string;
  dataURL: string;
}

export type WallpaperMode = 'zoom' | 'stretch' | 'maximize' | 'center';

export type Config = {
  mode: WallpaperMode;
  currentDirectory: string;
  currentWallpaper: Wallpaper;
}

export async function fetchWallpapers() {
  type response = {
    name: string;
    path: string;
  }

  const response = await fetch('/api/wallpapers');
  if (!response.ok) {
    console.error(response.statusText);
    return []
  }
  const data = await response.json() as response[];
  return data.map((d) => {
    const url = `http://localhost:8080/images/${d.name}`;
    return {
      name: d.name,
      path: d.path,
      dataURL: url.toString(),
    }
  });
}

export async function applyWallpaper(wallpaper: Wallpaper) {
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

export async function fetchConfig() {
  const response = await fetch('/api/config');
  if (!response.ok) {
    console.error(response.statusText);
    return undefined
  }
  const data = await response.json();
  return data as Config
}


export async function changeMode(mode: WallpaperMode) {
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

// export async function changeDirectory(dir: string) {
//   const response = await fetch('/api/config', {
//     method: 'POST',
//     headers: {
//       'Content-Type': 'application/json',
//     },
//     body: JSON.stringify({currentDirectory: dir}),
//   });
//   if (!response.ok) {
//     console.error(response.statusText);
//     return false
//   }
//   return true
// }
