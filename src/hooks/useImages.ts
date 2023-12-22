import { useEffect, useState } from "react";
import { ImageQuery } from "../interfaces";
import { getCachedData } from "../utils";

export const useImages = () => {
  const [imageQuery, setImageQuery] = useState<ImageQuery>({
    images: [],
    filtered: [],
    search: ""
  });

  useEffect(() => {
    async function getImageQuery() {
      try {
        const imageCache = await getCachedData();
        setImageQuery(query => ({ ...query, images: imageCache }));
      } catch (error) {
        console.error("Error fetching data:", error);
      }
    }
    getImageQuery(); 
  }, []); 

  return {imageQuery, setImageQuery};
};

