import React, { useState, useRef, useEffect } from 'react';

interface MenuHeaderProps {
    title: string;
    children: React.ReactNode;
    closeAllMenus: () => void;
}

const MenuHeader = ({ title, children, closeAllMenus }: MenuHeaderProps) => {
    const [isOpen, setIsOpen] = useState(false);
    const menuRef = useRef<HTMLDivElement>(null);

    const toggleMenu = () => {
        if (!isOpen) {
            closeAllMenus();
        }
        setIsOpen(!isOpen);
    };

    useEffect(() => {
        const handleOutsideClick = (event: any) => {
            if (menuRef.current && !menuRef.current.contains(event.target)) {
                setIsOpen(false);
            }
        };

        if (isOpen) {
            document.addEventListener('mousedown', handleOutsideClick);
        } else {
            document.removeEventListener('mousedown', handleOutsideClick);
        }

        return () => {
            document.removeEventListener('mousedown', handleOutsideClick);
        };
    }, [isOpen]);

    return (
        <div className="menu-header" ref={menuRef}>
            <div
                className={`menu-header-title ${isOpen ? 'active' : ''}`}
                onClick={toggleMenu}
            >
                {title}
            </div>
            {isOpen && (
                <div className="dropdown-menu" onClick={() => setIsOpen(false)}>
                    {React.Children.map(children, child => {
                        if (React.isValidElement(child)) {
                            return React.cloneElement(child, {
                                closeMenu: () => {setIsOpen(false)}
                            });
                        }
                        return child;
                    })}
                </div>
            )}
        </div>
    );
};

export default MenuHeader;