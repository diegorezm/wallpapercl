import { ImageInterface } from '../interfaces';
import Image from './Image';

interface DisplayProps {
  images: ImageInterface[];
}

export default function Display({ images }: DisplayProps) {
  return (
    <>
      {images.map((val) => (
        <Image key={val.fileName} data={val} />
      ))}
    </>
  );
}

