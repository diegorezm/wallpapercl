import { invoke } from "@tauri-apps/api";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import type { ImageInterface, ImageQuery } from "../interfaces";

type CacheData = {
  cachetime: number;
  images: ImageInterface[];
  images_directory: string;
};

type GetImagesFromDirectory = (path: string) => Promise<ImageInterface[]>
export const getImagesFromDirectory: GetImagesFromDirectory = async (path) => {
  try {
    type Response = {
      file_path: string,
      file_name: string
    }
    const images: ImageInterface[] = [];
    const paths = (await invoke("list_dir", {
      dirPath: path,
    })) as Response[];
    paths.forEach((val) => {
      const image: ImageInterface = {
        relativePath: val.file_path,
        path: convertFileSrc(val.file_path),
        fileName: val.file_name
      };
      images.push(image);
    });

    const jsonData = { images, images_directory: path };
    localStorage.setItem("images", JSON.stringify(jsonData));
    return images;
  } catch (error) {
    console.error("Error fetching paths:", error);
    return [];
  }
};

type GetCachedData = () => Promise<ImageInterface[]>;
export const getCachedData: GetCachedData = async () => {
  const cacheData = localStorage.getItem("images");
  if (cacheData) {
    const data: CacheData = JSON.parse(cacheData);
    return data.images;
  }
  const directory = getStoredPath();
  if (directory) {
    return await getImagesFromDirectory(directory);
  }
  return [];
};

type GetStoredPath = () => string
export const getStoredPath: GetStoredPath = () => {
  const cacheData = localStorage.getItem("images");
  if (cacheData) {
    const data: CacheData = JSON.parse(cacheData);
    return data.images_directory;
  }
  return "";
};

type SearchHandler = (data: ImageQuery) => ImageInterface[]
export const searchHandler: SearchHandler = (data: ImageQuery) => {
  const searchTerm = data.search.toLowerCase() || "";
  data.filtered = data.images.filter((item) => {
    return item.fileName.toLowerCase().includes(searchTerm);
  });
  return data.filtered;
};
