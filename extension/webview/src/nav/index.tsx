import React from 'react';
import { createRoot } from 'react-dom/client';
import Application from './app';
import 'primeflex/primeflex.css';
import 'primeicons/primeicons.css';
import './app.css';
const container = document.getElementById('app') as HTMLElement;
const root = createRoot(container);
root.render(<Application/>);