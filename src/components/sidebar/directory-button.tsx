import { useActions } from "@/hooks/use-actions";
import { Button } from "../ui/button";
import { Folder } from "lucide-react";
import { getStoredPath } from "@/utils";

const DirectoryButton = () => {
  const { changeDirectory } = useActions();
  const path = getStoredPath();
  return (
    <div className="flex items-center">
      <Button onClick={changeDirectory} variant={"outline"} className="mt-2">
        <Folder className="mr-2 size-5" /> {path}
      </Button>
    </div>
  );
};
export default DirectoryButton;
