import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";
import { Settings } from "lucide-react";
import ThemeSelect from "./theme-select";
import RefreshButton from "./refresh-button";
import DirectoryButton from "./directory-button";

const Sidebar = () => {
  return (
    <Sheet>
      <SheetTrigger className="mb-1">
        <Settings />
      </SheetTrigger>
      <SheetContent className="space-y-4">
        <SheetHeader>
          <SheetTitle className="text-3xl">Configuration</SheetTitle>
        </SheetHeader>
        <div>
          <SheetTitle className="text-lg">Refresh</SheetTitle>
          <RefreshButton />
        </div>

        <div>
          <SheetTitle className="text-lg">Change directory</SheetTitle>
          <DirectoryButton />
        </div>
        <div>
          <SheetTitle className="text-lg">Theme</SheetTitle>
          <ThemeSelect />
        </div>
      </SheetContent>
    </Sheet>
  );
};
export default Sidebar;
