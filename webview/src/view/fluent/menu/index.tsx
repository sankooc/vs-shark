import * as React from "react";
import type { JSXElement } from "@fluentui/react-components";
import {
    MenuList,
    MenuItem,
    MenuPopover,
    MenuTrigger,
    Menu,
    Button,
} from "@fluentui/react-components";
import {
    EditRegular,
    EditFilled,
    bundleIcon,
    CutRegular,
    CutFilled,
    ClipboardPasteRegular,
    ClipboardPasteFilled,
    DeleteFilled,
    DeleteRegular,
    FolderOpenRegular,
} from "@fluentui/react-icons";

const EditIcon = bundleIcon(EditFilled, EditRegular);
const CutIcon = bundleIcon(CutFilled, CutRegular);
const PasteIcon = bundleIcon(ClipboardPasteFilled, ClipboardPasteRegular);
const DeleteIcon = bundleIcon(DeleteFilled, DeleteRegular);

export default function MultilineItems(): JSXElement {
    if (import.meta.env.DEV) {
        console.log(import.meta.env.VITE_BUILD_ALL);
    }
    return (
        <div className="flex flex-row items-center">
            {/* <img src="icon32.png" alt="File menu" /> */}
        <Menu>
            <MenuTrigger>
                <Button shape="square" size="medium" appearance="transparent" icon={<FolderOpenRegular />}>File</Button>
            </MenuTrigger>
            <MenuPopover>
                <MenuList>
                    <MenuItem subText="Open File" icon={<CutIcon />}>
                        Open
                    </MenuItem>
                    <MenuItem subText="Close File" icon={<PasteIcon />}>
                        Close
                    </MenuItem>
                    <MenuItem subText="Edit file" icon={<EditIcon />} disabled>
                        Edit
                    </MenuItem>
                    <MenuItem subText="Delete file" icon={<DeleteIcon />}>
                        Delete
                    </MenuItem>
                </MenuList>
            </MenuPopover>
        </Menu>
            
        </div>
    );
};