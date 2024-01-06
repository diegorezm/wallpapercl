import { invoke } from "@tauri-apps/api";
import { ImageInterface } from "../interfaces";
import { useImages } from "../hooks/useImages";
interface Props {
  data: ImageInterface,
  displayMode: boolean
}
export default function Image({ data, displayMode }: Props) {
  const { imageQuery } = useImages();
  const handleButtonClick = async (image: string) => {
    const changebg = await invoke("change_bg_image", { imgPath: image });
    const downloadImg = await invoke("download_wallpaper", {folder: imageQuery.defaultDir, path: image});
    return displayMode ? downloadImg: changebg;
  };
  return (
    <div className="w-full h-60 border-2 rounded border-[var(--purple-color)] hover:border-[var(--pink-color)] group">
      <button className="relative p-0 w-fit h-full hover:opacity-20 transition-all" onClick={() => handleButtonClick(data.relativePath)}>
        <img src={data.path} alt={data.fileName} className="w-full h-full object-cover" />
      </button>
    </div>
  );
}
