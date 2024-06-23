import React from 'react';
import { createRoot } from 'react-dom/client';
import HexView from './app';
import './app.css'
const container = document.getElementById('app') as HTMLElement;
const root = createRoot(container);
root.render(<HexView/>);