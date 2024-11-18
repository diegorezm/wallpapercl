import {applyWallpaper, type Wallpaper} from "../actions";

export const WallpaperCard = ({name, path, dataURL}: Wallpaper) => (
  <li className="card bg-base-200 shadow-lg w-full">
    <figure>
      <img src={dataURL} alt={name} className="w-full h-52 object-cover" />
    </figure>
    <div className="card-body">
      <h5 className="card-title truncate">{name}</h5>
      <p className="truncate">{path}</p>
      <button
        className="btn btn-primary btn-sm"
        onClick={() => applyWallpaper({
          name,
          path,
          dataURL
        })}
      >
        Apply
      </button>
    </div>
  </li>
);
