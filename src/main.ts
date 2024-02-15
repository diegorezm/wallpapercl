import { WallpaperHandler } from "./class/WallpaperHandler"
import { Actions } from "./class/Actions"

const openFoulderButton = document.querySelector("[open-folder-button]") as HTMLButtonElement
const refreshButton = document.querySelector("[refresh-button]") as HTMLButtonElement

const wallpaperSearch = document.querySelector("[wallpaper-search]") as HTMLInputElement

const wallpaperHanlder = new WallpaperHandler()
wallpaperHanlder.loadImagesFromDirectory()

wallpaperSearch.addEventListener("input", (e) => {
  const searchQuery = (e.target as HTMLInputElement)?.value;
  wallpaperHanlder.searchHandler(searchQuery)
  if (searchQuery.length === 0 || !searchQuery) {
    wallpaperHanlder.renderImages()
    wallpaperHanlder.filteredImages = []
  }
})

openFoulderButton.onclick = async () => {
  const directory = await Actions.handleDirectoryButton()
  wallpaperHanlder.loadImagesFromDirectory(directory)
}

refreshButton.onclick = async () => {
  const new_images = await Actions.handleRefreshButton()
  wallpaperHanlder.setImages = new_images
}
