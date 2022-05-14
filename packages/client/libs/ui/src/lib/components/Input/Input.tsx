import classNames from 'classnames';
import type { ChangeEvent, FC } from 'react';

export interface InputProps {
  id?: string;
  className?: string;
  placeholder?: string;
  disabled?: boolean;
  name?: string;
  type?: string;
  hasError?: boolean;
  onChange?(value: ChangeEvent): void;
  value?: string;
  required?: boolean;
  autoComplete?: string;
}

export const Input: FC<InputProps> = ({
  id,
  className,
  disabled,
  placeholder,
  name,
  hasError,
  onChange,
  value,
  type = 'text',
  required,
  autoComplete,
}) => {
  return (
    <input
      id={id}
      className={classNames(
        'appearance-none rounded relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm',
        { 'border-2 border-red-500': hasError },
        className
      )}
      type={type}
      name={name}
      required={required}
      onChange={onChange}
      value={value}
      placeholder={placeholder}
      disabled={disabled}
      autoComplete={autoComplete}
    />
  );
};
