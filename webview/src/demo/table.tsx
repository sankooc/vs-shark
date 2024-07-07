import React from "react";
import { Splitter, SplitterPanel } from 'primereact/splitter';
import { Panel } from 'primereact/panel';
import { TabView, TabPanel } from 'primereact/tabview';
import { ScrollPanel } from 'primereact/scrollpanel';
import { PanelMenu } from 'primereact/panelmenu';
import { Badge } from 'primereact/badge';
import BasicDemo from './tree';
import Data from './data';
import 'primeflex/primeflex.css';
import 'primeicons/primeicons.css';
// import 'primereact/resources/themes/md-dark-indigo/theme.css';
// import 'primereact/resources/themes/bootstrap4-light-blue/theme.css';

function Table() {
  const items = [];
  const mitems = [
    {
      label: 'Overview (12)',
      icon: 'fa-light fa-chevron-down',
    },
    {
      label: 'Cloud',
      icon: 'pi pi-cloud',
      items: [
        {
          label: 'Upload',
          icon: 'pi pi-cloud-upload'
        },
        {
          label: 'Download',
          icon: 'pi pi-cloud-download'
        },
        {
          label: 'Sync',
          icon: 'pi pi-refresh'
        }
      ]
    },
    {
      label: 'Devices',
      icon: 'pi pi-desktop',
      items: [
        {
          label: 'Phone',
          icon: 'pi pi-mobile'
        },
        {
          label: 'Desktop',
          icon: 'pi pi-desktop'
        },
        {
          label: 'Tablet',
          icon: 'pi pi-tablet'
        }
      ]
    }
  ];
  const upSize = 30;
  return (<>
    <div className="card h-full">
      <div className="flex flex-column md:flex-row h-full">
        <div className="w-2 flex flex-column">
          <PanelMenu model={mitems} className="w-full" style={{ fontSize: '1rem' }} />
        </div>
        <div className="w-full h-full flex align-items-center justify-content-center">

          <Splitter layout="vertical" className="h-full w-full">
            <SplitterPanel className="flex align-items-center justify-content-center" size={70}>
              <Data></Data>
            </SplitterPanel>
            <SplitterPanel className="flex align-items-center justify-content-center" size={30} minSize={20}>
              <Splitter className="w-full">
                <SplitterPanel className="flex align-items-center justify-content-center" size={50} style={{height: '28vh', overflow: 'auto'}}>
                  <BasicDemo></BasicDemo>
                </SplitterPanel>
                <SplitterPanel className="flex align-items-center justify-content-center" size={50} minSize={20}>
                </SplitterPanel>
              </Splitter>

            </SplitterPanel>
          </Splitter>
        </div>
      </div>
    </div>
  </>
  );
}

export default Table;