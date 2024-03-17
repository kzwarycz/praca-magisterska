import React from 'react';
import Card from './Card';

interface CardProps {
  id: string;
  title: string;
  url: string;
}

interface GalleryProps {
  cards: CardProps[];
}

const Gallery: React.FC<GalleryProps> = ({ cards }) => {
  return (
    <div id="gallery" className="gallery grid grid-cols-1 gap-4 lg:grid-cols-2 lg:gap-8 px-12">
      {cards.map((cardProps) => (
        <Card key={cardProps.id} {...cardProps} />
      ))}
    </div>
  );
};

export default Gallery;
