import { useActions } from "@/hooks/use-actions";
import { Button } from "../ui/button";
import { RefreshCw } from "lucide-react";

const RefreshButton = () => {
  const { revalidateCache } = useActions();
  return (
    <div className="flex items-center">
      <Button onClick={revalidateCache} variant={"outline"} className="mt-2">
        <RefreshCw className="size-5 mr-2" /> refresh
      </Button>
    </div>
  );
};
export default RefreshButton;
