import { type Component, createEffect, createSignal } from 'solid-js';
import Banner from './Banner';
import Gallery from './Gallery';
import Footer from './Footer';

interface ImageData {
  id: string;
  title: string;
  url: string;
}

const App: Component = () => {
  const [images, setImages] = createSignal<ImageData[]>([]);

  createEffect(() => {
    fetch('/image_data.json')
      .then((response) => response.json())
      .then((data: ImageData[]) => {
        setImages(data);
      })
      .catch((error) => console.error('Failed to load image data', error));
  });

  return (
    <>
      <Banner />
      <div>
        <Gallery cards={images()} />
      </div>
      <Footer />
    </>
  );
};

export default App;
