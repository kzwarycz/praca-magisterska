import React from 'react';

const Banner: React.FC = () => {
  const scrollToGallery = () => {
    const galleryElement = document.getElementById('gallery');
    galleryElement?.scrollIntoView({ behavior: 'smooth' });
  };

  return (
    <section className="bg-gray-50">
      <div className="mx-auto max-w-screen-xl px-4 py-32 lg:flex lg:h-screen lg:items-center">
        <div className="mx-auto max-w-xl text-center">
          <h1 className="text-3xl font-extrabold sm:text-5xl">
            Web Apps with React.
            <strong className="font-extrabold text-blue-700 sm:block"> Build components with React and Node. </strong>
          </h1>

          <p className="mt-4 sm:text-xl">
            Combine the flexibility of React with the efficiency of Node.js runtime to create scalable and performant web applications.
          </p>

          <button
            className="mt-8 rounded bg-blue-500 px-6 py-2 text-white font-bold hover:bg-blue-700 focus:outline-none focus:ring"
            onClick={scrollToGallery}
          >
            View Gallery
          </button>
        </div>
      </div>
    </section>
  );
};

export default Banner;
