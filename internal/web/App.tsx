import {useEffect, useState} from "react";
import {applyWallpaper, fetchWallpapers, type Wallpaper} from "./actions";
import {WallpaperCard} from "./components/wallpaper-card";

export default function App() {
  const [wallpapers, setWallpapers] = useState<Wallpaper[]>([]);
  const [search, setSearch] = useState("");

  useEffect(() => {
    fetchWallpapers().then((w) => {
      setWallpapers(w);
    });
  }, []);

  const filteredWallpapers = wallpapers.filter((w) => w.name.toLowerCase().includes(search.toLowerCase()));

  return (
    <div className="flex flex-col items-center justify-center min-h-screen py-2 px-4 gap-6">

      <div className="w-full flex flex-col items-start justify-start">
        <h1 className="text-2xl font-bold">Wallpaper CL</h1>
      </div>

      <div className="w-full flex flex-col items-center justify-center">
        <input type="text" placeholder="Search..." className="input input-bordered w-full max-w-2xl" value={search} onChange={e => setSearch(e.target.value)} />
      </div>

      <ul className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 place-items-center w-full mx-auto border border-neutral rounded-lg p-4">
        {
          search === ""
            ? wallpapers.map((w) => (
              <WallpaperCard key={w.name} {...w} />
            ))
            : filteredWallpapers.map((w) => (
              <WallpaperCard key={w.name} {...w} />
            ))
        }
      </ul>
    </div>
  );
}
