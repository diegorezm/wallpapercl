export type Wallpaper = {
  name: string;
  path: string;
  dataURL: string;
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
