import { type Component } from 'solid-js';

interface CardProps {
  id: string;
  title: string;
  url: string;
}

const Card: Component<CardProps> = (props) => {
  return (
    <a class="block">
      <img
        alt={props.title}
        src={props.url}
        class="w-full object-cover max-h-64 sm:max-h-80 lg:max-h-96"
      />

      <h3 class="mt-4 text-lg font-bold text-gray-900 sm:text-xl">{props.title}</h3>

      <p class="mt-2 max-w-sm text-gray-700">
        Lorem ipsum dolor sit amet consectetur, adipisicing elit. Magni reiciendis sequi ipsam incidunt.
      </p>
    </a>
  );
};

export default Card;
