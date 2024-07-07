import React, { useEffect, useState } from "react";

function Page() {
    return (<div>
        <div className="ctl">
            <input type="file" id="files" name="avatar" />
        </div>
        <div className="f-g-1 flex flex-column">
            <div>
                <iframe id="main" src="frame.html"></iframe>
            </div>
            {/* <div className="flex flex-row">
                <iframe id="tree" src="tree.html"></iframe>
                <iframe id="hex" src="hex.html"></iframe>
            </div> */}
        </div>
    </div>
    );
}

export default Page;