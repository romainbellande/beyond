import { FC, ReactNode } from 'react';
import classNames, { Argument } from 'classnames';

type ButtonSize = 'sm' | 'md' | 'lg' | 'xl' | 'pagination';

type Layout = 'primary' | 'link' | 'outline';

export interface ButtonProps {
  type?: 'button' | 'submit';
  className?: string;
  onClick?(): void;
  disabled?: boolean;
  size?: ButtonSize;
  outline?: boolean;
  layout?: Layout;
  icon?: ReactNode;
  iconDirection?: 'left' | 'right';
}

const sizeStyleMap: Record<ButtonSize, string> = {
  sm: 'px-3 py-1 space-x-3',
  md: 'px-4 py-2 space-x-4',
  lg: 'px-5 py-3 space-x-5',
  xl: 'px-6 py-4 space-x-6',
  pagination: 'py-2 px-4',
};

export const Button: FC<ButtonProps> = ({
  type = 'button',
  children,
  onClick,
  className,
  disabled = false,
  size = 'md',
  layout = 'primary',
  icon,
  iconDirection = 'left',
}) => {
  const layoutStyleMap: Record<Layout, Argument[]> = {
    primary: [
      'bg-indigo-600',
      disabled
        ? 'bg-indigo-300'
        : 'hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500',
    ],
    outline: [
      'border border-indigo-500',
      disabled
        ? 'border-0 bg-indigo-100 text-indigo-300 cursor-disabled'
        : 'hover:border-indigo-400 hover:text-indigo-400 text-indigo-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500',
    ],
    link: [
      'text-indigo-400',
      disabled ? 'text-indigo-300' : 'hover:bg-indigo-100',
    ],
  };

  const defaultClasses: Argument[] = layoutStyleMap[layout || 'primary'];
  const Icon = () => <span className="w-5 h-5">{icon}</span>;

  return (
    <button
      className={classNames(
        className,
        ...defaultClasses,
        sizeStyleMap[size || 'md'],
        'group rounded-lg font-semibold flex justify-center items-center h-fit w-full relative',
        { 'cursor-not-allowed': disabled }
      )}
      disabled={disabled}
      data-testid="button"
      type={type}
      onClick={onClick}
    >
      {icon ? (
        <span
          className={classNames(
            'w-5 h-5 absolute top-1/2 -translate-y-1/2 flex items-center group-hover:text-indigo-400',
            iconDirection === 'left' ? 'left-3' : 'right-3'
          )}
        >
          {icon}
        </span>
      ) : null}
      <span className="whitespace-nowrap capitalize-first ">{children}</span>
    </button>
  );
};
