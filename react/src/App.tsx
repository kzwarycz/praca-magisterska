import React, { useEffect, useState } from 'react';
import Banner from './Banner';
import Gallery from './Gallery';
import Footer from './Footer';

interface ImageData {
  id: string;
  title: string;
  url: string;
}

const App: React.FC = () => {
  const [images, setImages] = useState<ImageData[]>([]);

  useEffect(() => {
    fetch('/image_data.json')
      .then((response) => response.json())
      .then((data: ImageData[]) => {
        setImages(data);
      })
      .catch((error) => console.error('Failed to load image data', error));
  }, []);

  return (
    <>
      <Banner />
      <div>
        <Gallery cards={images} />
      </div>
      <Footer />
    </>
  );
};

export default App;
