import { invoke } from "@tauri-apps/api";
import { ImageInterface } from "../interfaces";
type props = {
  data: ImageInterface
}
export default function Image({ data }: props) {
  const handleButtonClick = async (image: string) => {
    return await invoke("change_bg_image", { imgPath: image });
  };
  return (
    <div className="w-full h-60 border-2 rounded border-[var(--purple-color)] hover:border-[var(--pink-color)] group">
      <button className="relative p-0 w-fit h-full hover:opacity-20 transition-all" onClick={() => handleButtonClick(data.relativePath)}>
        <img src={data.path} alt={data.fileName} className="w-full h-full object-cover" />
      </button>
    </div>
  );
}
