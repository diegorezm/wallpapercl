import { useWallpapersContext } from "@/providers/wallpaper-provider";
import { Input } from "./ui/input";

const SearchComponent = () => {
  const { filter } = useWallpapersContext();
  const onChange = (q: string) => {
    filter(q);
  };
  return (
    <div className="w-2/3">
      <Input
        onChange={(e) => onChange(e.target.value)}
        placeholder="Search for your wallpaper..."
      />
    </div>
  );
};

export default SearchComponent;
