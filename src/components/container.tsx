type Props = {
  children?: React.ReactNode;
};

const Container = ({ children }: Props) => {
  return (
    <main className="flex flex-col items-center justify-center gap-4 w-full h-full p-6">
      {children}
    </main>
  );
};

export default Container;
