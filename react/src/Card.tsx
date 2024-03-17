import React from 'react';

interface CardProps {
  id: string;
  title: string;
  url: string;
}

const Card: React.FC<CardProps> = ({ id, title, url }) => {
  return (
    <a className="block">
      <img
        alt={title}
        src={url}
        className="w-full object-cover max-h-64 sm:max-h-80 lg:max-h-96"
      />

      <h3 className="mt-4 text-lg font-bold text-gray-900 sm:text-xl">{title}</h3>

      <p className="mt-2 max-w-sm text-gray-700">
        Lorem ipsum dolor sit amet consectetur, adipisicing elit. Magni reiciendis sequi ipsam incidunt.
      </p>
    </a>
  );
};

export default Card;
