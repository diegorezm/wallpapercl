import Grid from "./components/Grid";
import Navbar from "./components/Navbar";
import { useImages } from "./hooks/useImages";
export default function App() {
  const { imageQuery, setImageQuery } = useImages();
  return (
    <main className="w-full flex flex-col justify-center items-center p-2">
      <section className="w-full">
        <Navbar imageQuery={imageQuery} setImageQuery={setImageQuery} />
      </section>
      <section className="flex flex-col items-center justify-center m-2 border-2 border-[var(--pink-color)] rounded">
        <Grid imageQuery={imageQuery} />
      </section>
    </main>
  );
}

