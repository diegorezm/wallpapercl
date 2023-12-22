import { ImageQuery } from "../interfaces";
import Search from "./Search";

interface props {
  imageQuery: ImageQuery,
  setImageQuery: React.Dispatch<React.SetStateAction<ImageQuery>>
}

export default function Navbar(props: props) {
  return (
    <nav className="flex flex-row justify-between items-center px-2 w-full relative">
      <Search {...props} />
    </nav>
  );
}
