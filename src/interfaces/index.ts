export interface ImageInterface {
  path: string,
  relativePath: string,
  fileName: string
}

export interface ImageQuery {
  images: ImageInterface[],
  filtered: ImageInterface[],
  defaultDir: string,
  search: string
}
