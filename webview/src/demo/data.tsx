import React from "react";
import { Button } from 'primereact/button';
import { DataTable } from 'primereact/datatable';
import { Column } from 'primereact/column';
import { Row } from 'primereact/row';


const Data = () => {
  const customers = [];
  for(let i = 0 ; i< 1000; i+= 1){
    customers.push({
      name: `in:${i}`,
      country: 'country',
      company: 'compd',
      representative: '12334'
    });
  }
  const onSelectionChange= (e) => {
    console.log(e);
  }
  return (<>
    <DataTable value={customers} selectionMode="single" rowClassName={(data) => "acdcd" } onSelectionChange={onSelectionChange} virtualScrollerOptions={{ itemSize: 20 }} scrollable showGridlines scrollHeight="70vh" size="small" className="w-full"
    currentPageReportTemplate="{first} to {last} of {totalRecords}">
        <Column field="name" header="Name" style={{ width: '10%' }} className="name-fact"></Column>
        <Column field="country" header="Country" style={{ width: '25%' }}></Column>
        <Column field="company" header="Company" style={{ width: '25%' }}></Column>
        <Column field="representative" header="Representative" style={{ width: '25%' }}></Column>
    </DataTable>
  </>
  );
};


export default Data;