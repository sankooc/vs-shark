import { useCallback } from 'react';
import MenuHeader from './MenuHeader';
import MenuItem from './MenuItem';
import './MenuBar.css';
import { HeaderProps } from '../app';


const MenuBar = (props: HeaderProps) => {
  const { pFile, triggerNewFile, triggerReset } = props;
  const closeAllMenus = useCallback(() => {
  }, []);

  const handleAction = (action: string) => {
    console.log(`do: ${action}`);
    switch(action){
      case 'open' : {
        triggerNewFile();
        return ;
      }
      case 'close': {
        triggerReset();
      }
    }
  };

  return (
    <div className="menu-bar">
      <MenuHeader title="File" closeAllMenus={closeAllMenus}>
        {pFile ? <MenuItem
          label="Close"
          onClick={() => handleAction('close')}
        /> : <MenuItem
          label="Open"
          shortcut="Cmd+O"
          onClick={() => handleAction('open')}
        />}
        
        {/* <MenuItem 
          label="Open Recent" 
          shortcut="Cmd+R" 
          onClick={() => handleAction('Save')} 
        /> */}
      </MenuHeader>
      <MenuHeader title="Help" closeAllMenus={closeAllMenus}>
        <MenuItem
          label="About"
          onClick={() => handleAction('About')}
        />
      </MenuHeader>

    </div>
  );
};

export default MenuBar;