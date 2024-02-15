import { invoke } from "@tauri-apps/api"
import { convertFileSrc } from "@tauri-apps/api/tauri"
import { ImageInterface } from "../types"

type CacheData = {
  images: ImageInterface[]
  images_directory: string
}

type GetImagesFromDirectory = (path: string) => Promise<ImageInterface[]>
export const getImagesFromDirectory: GetImagesFromDirectory = async (path) => {
  try {
    type Wallpapers = {
      file_path: string,
      file_name: string
    }
    type Response = {
      dir_path: string,
      dir_files: Wallpapers[]
    }    
    const images: ImageInterface[] = []
    const directory = (await invoke("list_dir", {
      path,
    })) as Response

    directory.dir_files.forEach((val) => {
      const image: ImageInterface = {
        relativePath: val.file_path,
        path: convertFileSrc(val.file_path),
        fileName: val.file_name
      }
      images.push(image)
    })

    const jsonData = { images, images_directory: directory.dir_path }
    setCacheData(jsonData)
    return images
  } catch (error) {
    console.error("Error fetching paths:", error)
    return []
  }
}

type GetCachedData = () => Promise<ImageInterface[]>
export const getCachedData: GetCachedData = async () => {
  const cacheData = localStorage.getItem("images")
  if (cacheData) {
    const data: CacheData = JSON.parse(cacheData)
    return data.images
  }
  const directory = getStoredPath()
  if (directory) {
    return await getImagesFromDirectory(directory)
  }
  return []
}

export const setCacheData = (cache: CacheData) => {
  localStorage.setItem("images", JSON.stringify(cache))
}


type GetStoredPath = () => string
export const getStoredPath: GetStoredPath = () => {
  const cacheData = localStorage.getItem("images")
  if (cacheData) {
    const data: CacheData = JSON.parse(cacheData)
    return data.images_directory
  }
  return ""
}

type SetStoredPath = (path: string) => void
export const setStoredPath: SetStoredPath = (path) => {
  const cacheData = localStorage.getItem("images")
  if (cacheData) {
    const data: CacheData = JSON.parse(cacheData)
    data.images_directory = path
    localStorage.setItem("images", JSON.stringify(data))
  }
}


