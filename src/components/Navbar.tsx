
import { IoFolderOpenOutline, IoRefresh } from "react-icons/io5";
import { open } from "@tauri-apps/api/dialog";
import { ImageInterface, ImageQuery } from "../interfaces";
import Search from "./Search";
import { getImagesFromDirectory } from "../utils";

interface props {
  imageQuery: ImageQuery,
  setImageQuery: React.Dispatch<React.SetStateAction<ImageQuery>>
}

export default function Navbar(props: props) {
  const { imageQuery, setImageQuery } = props;
  const handleRefreshButton = async () => {
    const baseDir = imageQuery.defaultDir;
    const images = await getImagesFromDirectory(baseDir);
    setImageQuery(query => ({ ...query, images }));
    window.location.reload();
  };
  const handleDirectoryClick = async () => {
    const selected = (await open({
      directory: true,
      multiple: false,
    })) as string;
    imageQuery.defaultDir = selected;
    const images = await getImagesFromDirectory(imageQuery.defaultDir) as ImageInterface[];
    setImageQuery(query => ({...query, images}));
  };
  return (
    <nav className="flex flex-row justify-between items-center px-2 w-full relative">
      <button className="bg-gradient-to-b flex justify-center items-center from-[var(--pink-color)] to-[var(--purple-color)] w-[12%] md:w-[8%] lg:w-[5%] h-8 px-2 py-2 text-[var(--crust)] rounded" onClick={handleDirectoryClick}>
        <span className="text-2xl text-center">
          <IoFolderOpenOutline />
        </span>
      </button>
      <Search {...props} />
      <button className="bg-gradient-to-b flex justify-center items-center from-[var(--pink-color)] to-[var(--purple-color)] w-[12%] md:w-[8%] lg:w-[5%] h-8 px-2 py-2 text-[var(--crust)] rounded" onClick={handleRefreshButton}>
        <span className="text-2xl text-center">
          <IoRefresh />
        </span>
      </button>
    </nav>
  );
}
