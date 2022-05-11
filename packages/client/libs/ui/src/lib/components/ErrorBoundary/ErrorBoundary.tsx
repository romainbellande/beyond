import { Component, ReactNode } from 'react';

interface State {
  hasError: boolean;
}

export interface ErrorBoundaryProps {
  children: ReactNode | JSX.Element;
}

export class ErrorBoundary extends Component {
  override state: State = { hasError: false };

  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error: any) {
    // Update state so the next render will show the fallback UI.
    return { hasError: true };
  }

  override componentDidCatch(error: any, errorInfo: any) {
    // You can also log the error to an error reporting service
    this.setState({ hasError: true });
    console.error(error, errorInfo);
  }

  override render() {
    if (this.state.hasError) {
      // You can render any custom fallback UI
      return <h1>Something went wrong.</h1>;
    }

    return this.props.children;
  }
}
