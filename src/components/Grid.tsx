import { useEffect, useState } from "react";
import { ImageInterface, ImageQuery } from "../interfaces";
import { convertApiImages } from "../utils";
import Display from "./Display";
interface Props {
  imageQuery: ImageQuery,
  displayMode: boolean
}
export default function Grid({ imageQuery, displayMode }: Props) {
  const [imageQueryAPI, setImageQueryApi] = useState<ImageInterface[]>([]);
  const getImagesFromApi = async () => {
    const data = await convertApiImages();
    setImageQueryApi(data);
  };
  useEffect(() => {
    getImagesFromApi();
  }, []);
  return (
    <>
      {imageQuery.search.trim().length > 0 && imageQuery.filtered.length < 1 && <div className="flex justify-center items-center text-center text-[var(--red-color)] text-2xl font-bold h-full w-full p-2">NO RESULTS FOUND!</div>}
      <div className="w-full grid gap-2 md:grid-cols-2 lg:grid-cols-3 place-items-center p-2 overflow-y-hidden">
        {imageQuery.search.trim().length > 0 ?
          (
            <Display images={imageQuery.filtered} displayMode={displayMode}/>
          )
          :
          (
            <Display images={displayMode ? imageQueryAPI : imageQuery.images} displayMode={displayMode}/>
          )
        }
      </div>
    </>
  );
}
