import { open } from "@tauri-apps/api/dialog";
import {
  getImagesFromDirectory,
  getStoredPath,
  setCacheData,
  setStoredPath,
} from "@/utils";
import { useWallpapersContext } from "@/providers/wallpaper-provider";

export const useActions = () => {
  const { load } = useWallpapersContext();
  const revalidateCache = async () => {
    const path = getStoredPath();
    const images = await getImagesFromDirectory(path);
    setCacheData({
      images_directory: path,
    });
    load();
    return images;
  };

  const changeDirectory = async () => {
    const selected = (await open({
      directory: true,
      multiple: false,
    })) as string;
    setStoredPath(selected);
    load(selected);
    return selected;
  };

  return {
    revalidateCache,
    changeDirectory,
  };
};

