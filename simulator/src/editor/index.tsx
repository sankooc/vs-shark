import React from 'react';
import App from './app';
import { createRoot } from 'react-dom/client';
import { error } from '../connect';
const container = document.getElementById('root') as HTMLElement;
const root = createRoot(container);
root.render(<App/>);


window.onerror = (event: Event | string, source?: string, lineno?: number, colno?: number, er?: Error) => {
    error({source, lineno, colno});
};