import {Mode} from "@/utils/tauri-commands"
import {createContext, useContext, useState} from "react"

export interface ModeContextType {
  mode: Mode
  setMode: (mode: Mode) => void
}

const ModeContext = createContext<ModeContextType | undefined>(undefined)

export const ModeProvider: React.FC<{children: React.ReactNode}> = ({children}) => {
  const [mode, setModeState] = useState<Mode>(() => {
    const m = localStorage.getItem("wallpaper-mode")
    if (m) {
      return m as Mode
    }
    return "Center"
  })

  const setMode = (mode: Mode) => {
    setModeState(mode)
    localStorage.setItem("wallpaper-mode", mode)
  }

  return (
    <ModeContext.Provider value={{mode, setMode}}>
      {children}
    </ModeContext.Provider>
  )
}

export const useModeContext = () => {
  const context = useContext(ModeContext)
  if (context === undefined) {
    throw new Error("useModeContext must be used within a ModeProvider")
  }
  return context
}
