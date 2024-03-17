import { type Component, createSignal } from 'solid-js';

const Banner: Component = () => {
  const scrollToGallery = () => {
    const galleryElement = document.getElementById('gallery');
    galleryElement?.scrollIntoView({ behavior: 'smooth' });
  };

  return (
    <section class="bg-gray-50">
      <div class="mx-auto max-w-screen-xl px-4 py-32 lg:flex lg:h-screen lg:items-center">
        <div class="mx-auto max-w-xl text-center">
          <h1 class="text-3xl font-extrabold sm:text-5xl">
            Fast Web Apps with SolidJS.
            <strong class="font-extrabold text-blue-700 sm:block"> Powered by SolidJS and Bun. </strong>
          </h1>

          <p class="mt-4 sm:text-xl">
            Combine the speed of Solid for the frontend with the efficiency of Bun to create performant web applications.
          </p>

          <button
            class="mt-8 rounded bg-blue-500 px-6 py-2 text-white font-bold hover:bg-blue-700 focus:outline-none focus:ring"
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
