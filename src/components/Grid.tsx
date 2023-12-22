import { ImageQuery } from "../interfaces";
import Display from "./Display";
export default function Grid({ imageQuery }: {
  imageQuery: ImageQuery
}) {
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
