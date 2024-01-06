interface Props {
  setDisplayMode: (mode: boolean) => void,
  displayMode: boolean
}

export default function ModeSwitcher({ setDisplayMode, displayMode }: Props) {
  return (
    <div className="flex flex-row m-0 p-1 w-full">

      <button
        className={`bg-gradient-to-b flex py-1 justify-center items-center ${displayMode === false ? "from-[var(--pink-color)] to-[var(--purple-color)]" : "bg-[var(--crust)] text-[var(--foreground)]"} w-1/2 text-[var(--crust)] rounded`} onClick={() => setDisplayMode(false)}>
        Local images
      </button>
      <button className={`bg-gradient-to-b py-1 flex justify-center items-center ${displayMode ? "from-[var(--pink-color)] to-[var(--purple-color)]" : "bg-[var(--crust)] text-[var(--foreground)]"} w-1/2 text-[var(--crust)] rounded`} onClick={() => setDisplayMode(true)}>
        Api
      </button>

    </div>
  );
}
