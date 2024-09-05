import { open } from "@tauri-apps/api/dialog";
import {
  getImagesFromDirectory,
  getStoredPath,
  setCacheData,
  setStoredPath,
} from "@/utils";
import { useWallpapersContext } from "@/providers/wallpaper-provider";
import { toast } from "sonner";

export const useActions = () => {
  const { load } = useWallpapersContext();

  const revalidateCache = async () => {
    const path = getStoredPath();
    const images = await getImagesFromDirectory(path);
    setCacheData({
      images_directory: path,
    });
    load();
    toast.success(
      "Cache revalidated successfully! Your wallpapers have been updated.",
    );
    return images;
  };

  const changeDirectory = async () => {
    const selected = (await open({
      directory: true,
      multiple: false,
    })) as string;

    if (selected) {
      setStoredPath(selected);
      load(selected);
      toast.success("Wallpaper directory changed successfully!");
    } else {
      toast.error("No directory selected. Please choose a valid directory.");
    }

    return selected;
  };

  return {
    revalidateCache,
    changeDirectory,
  };
};
