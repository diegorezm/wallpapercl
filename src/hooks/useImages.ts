import { useEffect, useState } from "react";
import { ImageQuery } from "../interfaces";
import { getCachedData, getStoredPath } from "../utils";

export const useImages = () => {
  const [imageQuery, setImageQuery] = useState<ImageQuery>({
    images: [],
    filtered: [],
    search: "",
    defaultDir: ""
  });

  useEffect(() => {
    async function getImageQuery() {
      try {
        const imageCache = await getCachedData();
        const storedPath = getStoredPath();
        setImageQuery(query => ({ ...query, images: imageCache, defaultDir: storedPath }));
      } catch (error) {
        console.error("Error fetching data:", error);
      }
    }
    getImageQuery();
  }, []);

  return { imageQuery, setImageQuery };
};

