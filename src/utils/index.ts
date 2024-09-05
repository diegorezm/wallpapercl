import { ImageInterface } from "../types"
import { get_cached_dir, list_dir } from "./tauri-commands"

type CacheData = {
  images_directory: string
}
const CACHEKEY = "directory_path"

type GetImagesFromDirectory = (path: string) => Promise<ImageInterface[]>
export const getImagesFromDirectory: GetImagesFromDirectory = async (path) => {
  try {
    const directory = await list_dir(path)
    const jsonData = { images_directory: directory.dir_path }
    setCacheData(jsonData)
    return directory.dir_files
  } catch (error) {
    console.error("Error fetching paths:", error)
    return []
  }
}

type GetCachedData = () => Promise<ImageInterface[]>
export const getCachedData: GetCachedData = async () => {
  let directory = await get_cached_dir()
  if (directory === null && getStoredPath() !== "") {
    const path = getStoredPath()
    directory = await list_dir(path)
  }
  return directory.dir_files
}

export const setCacheData = (cache: CacheData) => {
  localStorage.setItem(CACHEKEY, JSON.stringify(cache))
}


type GetStoredPath = () => string
export const getStoredPath: GetStoredPath = () => {
  const cacheData = localStorage.getItem(CACHEKEY)
  if (cacheData) {
    const data: CacheData = JSON.parse(cacheData)
    return data.images_directory
  }
  return ""
}

type SetStoredPath = (path: string) => void
export const setStoredPath: SetStoredPath = (path) => {
  const cacheData = localStorage.getItem(CACHEKEY)
  if (cacheData) {
    const data: CacheData = JSON.parse(cacheData)
    data.images_directory = path
    localStorage.setItem("images", JSON.stringify(data))
  }
}


