import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import {useModeContext} from "@/providers/mode-provider";
import {Mode} from "@/utils/tauri-commands";
import {useState} from "react";

const ModeSelect = () => {
  const {mode, setMode} = useModeContext();
  const [selectedMode, setSelectedMode] = useState<Mode>(mode);
  const modes = ["Center", "Crop", "Fit", "Span", "Stretch", "Tile"] as const;

  const handleModeChange = (mode: Mode) => {
    setSelectedMode(mode);
    setMode(mode);
  };

  return (
    <Select value={selectedMode} onValueChange={(value) => handleModeChange(value as Mode)}>
      <SelectTrigger>
        <SelectValue placeholder="Mode" />
      </SelectTrigger>
      <SelectContent>
        {modes.map((mode) => (
          <SelectItem key={mode} value={mode}>
            {mode}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};

export default ModeSelect;
