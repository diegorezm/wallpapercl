import React, { createContext, useContext, ReactNode, useEffect } from "react";
import { useWallpapers } from "@/hooks/use-wallpaper";
import { ImageInterface } from "@/types";

interface WallpapersContextType {
  wallpaper: ImageInterface[];
  filtered: ImageInterface[];
  load: (path?: string) => Promise<void>;
  filter: (q?: string) => Promise<void>;
}

const WallpapersContext = createContext<WallpapersContextType | undefined>(
  undefined,
);

export const WallpapersProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const wallpapers = useWallpapers();
  useEffect(() => {
    wallpapers.load();
  }, []);

  return (
    <WallpapersContext.Provider value={wallpapers}>
      {children}
    </WallpapersContext.Provider>
  );
};

export const useWallpapersContext = () => {
  const context = useContext(WallpapersContext);
  if (context === undefined) {
    throw new Error(
      "useWallpapersContext must be used within a WallpapersProvider",
    );
  }
  return context;
};
