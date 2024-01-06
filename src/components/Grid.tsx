import { useEffect, useState } from "react";
import { ImageInterface, ImageQuery } from "../interfaces";
import { convertApiImages } from "../utils";
import Display from "./Display";
export default function Grid({ imageQuery }: {
  imageQuery: ImageQuery
}) {
  const [imageQueryAPI, setImageQueryApi] = useState<ImageInterface[]>([]);
  const imageQueryApi = async () => {
    const data = await convertApiImages();
    setImageQueryApi(data);
  };
  useEffect(() => {
    imageQueryApi();
  }, []);
  return (
    <>
      {imageQuery.search.trim().length > 0 && imageQuery.filtered.length < 1 && <div className="flex justify-center items-center text-center text-[var(--red-color)] text-2xl font-bold h-full w-full p-2">NO RESULTS FOUND!</div>}
      <div className="w-full grid gap-2 md:grid-cols-2 lg:grid-cols-3 place-items-center p-2 overflow-y-hidden">
        {imageQuery.search.trim().length > 0 ?
          (
            <Display images={imageQuery.filtered} />
          )
          :
          (
            <Display images={imageQuery.images} />
          )
        }
      </div>
    </>
  );
}
