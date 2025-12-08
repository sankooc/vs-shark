interface MenuItemProps {
    label: string;
    shortcut?: string;
    onClick: () => void;
    closeMenu: () => void;
    disabled?: boolean;
}

const MenuItem = (props: MenuItemProps) => {
    const { label, shortcut, onClick, disabled = false } = props;
    const handleClick = (e: any) => {
        e.stopPropagation();
        if (!disabled) {
            onClick();
        }
        props.closeMenu();
    };

    return (
        <div
            className={`menu-item ${disabled ? 'disabled' : ''}`}
            onClick={handleClick}
        >
            <span className="menu-item-label">{label}</span>
            {shortcut && <span className="menu-item-shortcut">{shortcut}</span>}
        </div>
    );
};

export default MenuItem;