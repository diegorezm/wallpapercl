import Container from "./components/container";
import ImageGallery from "./components/image-gallery";
import SearchComponent from "./components/search-component";
import Sidebar from "./components/sidebar";
import { WallpapersProvider } from "./providers/wallpaper-provider";

function App() {
  return (
    <WallpapersProvider>
      <Container>
        <div className="w-full flex justify-between items-center">
          <div></div>
          <SearchComponent />
          <Sidebar />
        </div>
        <ImageGallery />
      </Container>
    </WallpapersProvider>
  );
}

export default App;
