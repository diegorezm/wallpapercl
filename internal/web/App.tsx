import {useEffect, useState} from "react";
import {changeMode, fetchConfig, fetchWallpapers, type Wallpaper, type WallpaperMode} from "./actions";
import {WallpaperCard} from "./components/wallpaper-card";

export default function App() {
  const [wallpapers, setWallpapers] = useState<Wallpaper[]>([]);
  const [search, setSearch] = useState("");
  const [mode, setMode] = useState<WallpaperMode>('zoom');
  const modes = ['zoom', 'stretch', 'maximize', 'center'] as const;

  useEffect(() => {
    fetchWallpapers().then((w) => {
      setWallpapers(w);
    });
    fetchConfig().then((c) => {
      setMode(c?.mode ?? 'zoom');
    });
  }, []);

  const onModeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    console.log(e.target.value);
    setMode(e.target.value as WallpaperMode);
    changeMode(mode)
  };

  const filteredWallpapers = wallpapers.filter((w) => w.name.toLowerCase().includes(search.toLowerCase()));

  return (
    <div className="flex flex-col items-center justify-center min-h-screen py-2 px-4 gap-6">

      <div className="w-full flex flex-col items-start justify-start">
        <h1 className="text-2xl font-bold">Wallpaper CL</h1>
      </div>

      <div className="w-full flex items-center justify-center join">
        <input type="text" placeholder="Search..." className="input input-bordered w-full max-w-2xl join-item" value={search} onChange={e => setSearch(e.target.value)} />
        <select className="select select-bordered join-item" defaultValue={mode} onChange={onModeChange}>
          {
            modes.map((m) => (
              <option key={m} value={m}>{m}</option>
            ))
          }
        </select>

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
