import { invoke } from "@tauri-apps/api"
import { ImageInterface } from "../types"
import { getCachedData, getImagesFromDirectory } from "../utils"

export class WallpaperHandler {
  private wallpaperTemplate: HTMLDivElement
  private wallpaperContainer: HTMLDivElement
  private images: ImageInterface[] = []
  private _filteredImages: typeof this.images = []

  public set setImages(new_images: ImageInterface[]) {
    this.images = new_images
    this.renderImages()
  }

  public get getImages() {
    return this.images
  }


  public get filteredImages(): typeof this.images {
    return this._filteredImages
  }
  public set filteredImages(value: typeof this.images) {
    this._filteredImages = value
  }

  constructor() {
    this.wallpaperTemplate = document.querySelector("[wallpaper-template]") as HTMLDivElement
    this.wallpaperContainer = document.querySelector("[wallpaper-container]") as HTMLDivElement
  }

  public async loadImagesFromDirectory(path?: string) {
    if (path) {
      this.images = await getImagesFromDirectory(path)
    } else {
      this.images = await getCachedData()
    }
    this.renderImages()
  }

  public renderImages(imagesToRender: ImageInterface[] = this.images): void {
    this.wallpaperContainer.innerHTML = '';
    imagesToRender.forEach(image => {
      const template = this.wallpaperTemplate.cloneNode(true) as HTMLDivElement;
      const imageTag = template.querySelector("[wallpaper-image]") as HTMLImageElement;
      imageTag.src = image.path;
      imageTag.alt = image.fileName;
      imageTag.onclick = async () => {
        await invoke("change_wallpaper", { wallpaper: { file_path: image.relativePath, file_name: image.fileName } });
      }
      this.wallpaperContainer.append(template);
    });
  }

  public searchHandler(searchQuery: string) {
    const searchTerm = searchQuery.toLowerCase() || ""
    this.filteredImages = this.images.filter((item) => item.fileName.toLowerCase().includes(searchTerm))
    if (this.filteredImages.length === 1){
      this.wallpaperContainer.classList.add("fixed")
    }else{
      this.wallpaperContainer.classList.remove("fixed")
    }
    this.renderImages(this.filteredImages)
  }

}
