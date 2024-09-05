import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import Image from "./image-component";
import { useWallpapersContext } from "@/providers/wallpaper-provider";

const ImageGallery = () => {
  const { wallpaper, filtered } = useWallpapersContext();
  return (
    <div className="w-full">
      <Card>
        <CardHeader>
          <CardTitle className="text-primary text-lg font-semibold">
            Image Gallery
          </CardTitle>
          <CardDescription className="text-md text-secondary-foreground">
            Browse through your collection of stunning wallpapers. Use the
            search feature to find specific images quickly.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <section className="grid grid-cols-1 md:grid-cols-2 2xl:grid-cols-2 gap-2">
            {(filtered.length > 0 ? filtered : wallpaper).map((e, i) => (
              <Image key={i + 1} image={e} />
            ))}
          </section>
        </CardContent>
      </Card>
    </div>
  );
};

export default ImageGallery;
