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
    primary: ['bg-slate-600', disabled ? 'bg-slate-300' : 'hover:bg-slate-500'],
    outline: [
      'border border-slate-500',
      disabled
        ? 'border-0 bg-slate-100 text-slate-300 cursor-disabled'
        : 'hover:border-slate-400 hover:text-slate-400 text-slate-500',
    ],
    link: [
      'text-slate-400',
      disabled ? 'text-slate-300' : 'hover:bg-slate-100',
    ],
  };

  const defaultClasses: Argument[] = layoutStyleMap[layout || 'primary'];
  const Icon = () => <span className="w-5 h-5">{icon}</span>;
  const hasIcon = Boolean(icon);

  return (
    <button
      className={classNames(
        className,
        ...defaultClasses,
        sizeStyleMap[size || 'md'],
        'rounded-lg font-semibold flex items-center h-fit w-fit',
        { 'cursor-not-allowed': disabled }
      )}
      disabled={disabled}
      data-testid="button"
      type={type}
      onClick={onClick}
    >
      {hasIcon && iconDirection === 'left' ? <Icon /> : null}
      <span className="whitespace-nowrap capitalize-first ">{children}</span>
      {hasIcon && iconDirection === 'right' ? <Icon /> : null}
    </button>
  );
};
