import React, { useState } from "react";
import { IconField } from 'primereact/iconfield';
import { MultiSelect } from 'primereact/multiselect';

class Proto {
  label: string;
  placeholder?: string;
  _options: string[];
  select?: (event: string[]) => void;
}
const Mult = (props: Proto) => {
  const [value, setValue] = useState<string[]>([]);
  const options = props._options.map((k) => ({name: k, code: k}))
  const opt = {
    placeholder: props.placeholder || "Select",
    maxSelectedLabels: 10,
    optionLabel: "name",
    onChange: (e) => {
      setValue(e.value);
      const _filter = e.value.map(f => f.code);
      props.select(_filter);
    },
    label: props.label,
    options,
    value,
  };
  return (<div className="flex align-items-center gap-3">
  <label className="font-bold block mb-2">{props.label}</label>
  <IconField className="filter w-3 flex">
  <MultiSelect {...opt} className="p-inputtext-sm" />
</IconField>
</div>)
}

export default Mult;