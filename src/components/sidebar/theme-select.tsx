import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import useTheme, { THEMES, Theme } from "@/hooks/use-theme";
import { useState } from "react";

const ThemeSelect = () => {
  const { getTheme, setTheme } = useTheme();
  const theme = getTheme();
  const [selectedTheme, setSelectedTheme] = useState<Theme>(theme);

  const handleChange = (event: Theme) => {
    setSelectedTheme(event);
    setTheme(event);
  };

  return (
    <Select
      value={selectedTheme}
      onValueChange={(e) => handleChange(e as Theme)}
    >
      <SelectTrigger className="w-full">
        <SelectValue placeholder="Select a theme" />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectLabel>Themes</SelectLabel>
          {THEMES.map((theme) => (
            <SelectItem key={theme} value={theme}>
              {theme}
            </SelectItem>
          ))}
        </SelectGroup>
      </SelectContent>
    </Select>
  );
};

export default ThemeSelect;
