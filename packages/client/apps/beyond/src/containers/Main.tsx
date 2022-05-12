import React from 'react';

interface Props {
  children: React.ReactNode;
}

function Main({ children }: Props) {
  return (
    <main className="h-full w-full overflow-y-auto">
      <div className="mx-auto h-full overflow-hidden">{children}</div>
    </main>
  );
}

export default Main;
