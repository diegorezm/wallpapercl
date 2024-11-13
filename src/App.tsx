import Container from "./components/container";
import ImageGallery from "./components/image-gallery";
import SearchComponent from "./components/search-component";
import Sidebar from "./components/sidebar";
import {ModeProvider} from "./providers/mode-provider";
import {WallpapersProvider} from "./providers/wallpaper-provider";

function App() {
  return (
    <WallpapersProvider>
      <ModeProvider>
        <Container>
          <div className="w-full flex justify-between items-center">
            <div></div>
            <SearchComponent />
            <Sidebar />
          </div>
          <ImageGallery />
        </Container>
      </ModeProvider>
    </WallpapersProvider>
  );
}

export default App;
