import {invoke} from "@tauri-apps/api";
import {ImageInterface} from "../types";

export type Mode = "Center" | "Crop" | "Fit" | "Span" | "Stretch" | "Tile"

export const change_wallpaper = async (image: ImageInterface, mode: Mode = "Center") => {
  return await invoke("change_wallpaper", {
    wallpaper:
      {file_path: image.file_path, file_name: image.file_name, path: image.path}, mode
  });
}

type ListDirResponse = {
  dir_path: string,
  dir_files: ImageInterface[]
}

export const list_dir = async (path: String) => {
  return await invoke("list_dir", {
    path,
  }) as ListDirResponse
}

export const search = async (query: String) => {
  return await invoke("search", {query}) as ImageInterface[]
}

export const get_cached_dir = async () => {
  return await invoke("get_cached_dir") as ListDirResponse
}
