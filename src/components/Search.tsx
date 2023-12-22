import { useEffect, useState } from "react";
import { ImageQuery } from "../interfaces";
import { searchHandler } from "../utils";
interface props {
  imageQuery: ImageQuery,
  setImageQuery: React.Dispatch<React.SetStateAction<ImageQuery>>
}
export default function Search(props: props) {
  const [searchQuery, setSearchQuery] = useState("");
  const { imageQuery, setImageQuery } = props;

  const handleSearchQuery = () => {
    setImageQuery((query) => ({ ...query, search: searchQuery }));
    const queueImages = searchHandler({ ...imageQuery, search: searchQuery });
    setImageQuery((query) => ({ ...query, filtered: queueImages }));
  };

  useEffect(() => {
    if (searchQuery.length === 0) {
      setImageQuery((query) => ({ ...query, search: "", filtered: [] }));
    } else {
      handleSearchQuery();
    }
  }, [searchQuery]);
  return (
    <form className="flex w-full justify-center p-2 gap-2 text-lg" onSubmit={(e) => e.preventDefault()}>
      <input
        className="bg-[var(--crust)] border-2 border-gray-700 text-[var(--foreground)] placeholder:text-gray-600 focus:outline-none focus:border-[var(--pink-color)] rounded-sm text-center w-1/2 h-8"
        type="text"
        placeholder="Search here..."
        value={searchQuery}
        onChange={e => setSearchQuery(e.target.value)}
      />
    </form>
  );
}
