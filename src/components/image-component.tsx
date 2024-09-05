import { ImageInterface } from "@/types";
import { invoke } from "@tauri-apps/api";

type Props = {
  image: ImageInterface;
};
const Image = ({ image }: Props) => {
  const onClick = async () => {
    await invoke("change_wallpaper", {
      wallpaper: {
        file_path: image.file_path,
        file_name: image.file_name,
        path: image.path,
      },
    });
  };
  return (
    <div className="hover:cursor-pointer" onClick={onClick}>
      <img
        src={image.path}
        alt={image.file_name}
        className="rounded-md object-cover"
      />
      <p className="text-center text-secondary-foreground text-md font-bold">
        {image.file_name}
      </p>
    </div>
  );
};

export default Image;
