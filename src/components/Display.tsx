import { ImageInterface } from '../interfaces';
import Image from './Image';

interface DisplayProps {
  images: ImageInterface[];
  displayMode: boolean;
}

export default function Display({ images , displayMode}: DisplayProps) {
  return (
    <>
      {images.map((val) => (
        <Image key={val.fileName} data={val} displayMode={displayMode}/>
      ))}
    </>
  );
}

