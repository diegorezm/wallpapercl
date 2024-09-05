import { ImageInterface } from "@/types";
import { getCachedData, getImagesFromDirectory } from "@/utils";
import { search } from "../utils/tauri-commands";
import { useState } from "react";

export const useWallpapers = () => {
  const [wallpaper, setWallpaper] = useState<ImageInterface[]>([]);
  const [filtered, setFiltered] = useState<ImageInterface[]>([]);

  const load = async (path?: string) => {
    if (path) {
      const images = await getImagesFromDirectory(path);
      setWallpaper(images);
    } else {
      const images = await getCachedData();
      setWallpaper(images);
    }
  };

  const filter = async (q?: string) => {
    try {
      if (q?.trim()) {
        const searchTerm = q.toLowerCase();
        const images = await search(searchTerm);
        setFiltered(images);
      } else {
        setFiltered([]);
      }
    } catch (error) {
      console.error("Failed to filter images:", error);
      setFiltered([]);
    }
  };

  return {
    wallpaper,
    filtered,
    load,
    filter,
  };
};

