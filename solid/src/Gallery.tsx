import { type Component, For } from 'solid-js';
import Card from './Card';

interface CardProps {
  id: string;
  title: string;
  url: string;
}

interface GalleryProps {
  cards: CardProps[];
}

const Gallery: Component<GalleryProps> = (props) => {
  return (
    <div id="gallery" class="gallery grid grid-cols-1 gap-4 lg:grid-cols-2 lg:gap-8 px-12">
      <For each={props.cards}>{(cardProps) => <Card {...cardProps} />}</For>
    </div>
  );
};

export default Gallery;
