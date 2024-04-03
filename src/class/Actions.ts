import { open } from "@tauri-apps/api/dialog"
import { getImagesFromDirectory, getStoredPath, setCacheData, setStoredPath } from "../utils";

export class Actions {
  static handleRefreshButton = async () => {
    const path = getStoredPath()
    const images = await getImagesFromDirectory(path)
    setCacheData({
      images_directory: path,
    })
    return images
  }

  static handleDirectoryButton = async () => {
    const selected = (await open({
      directory: true,
      multiple: false,
    })) as string;
    setStoredPath(selected)
    return selected
  }

}
