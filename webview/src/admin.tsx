import { createRoot } from 'react-dom/client'
import './index.css'
import AdminApp from './AdminApp.tsx'

createRoot(document.getElementById('root')!).render(
    <AdminApp />
)
