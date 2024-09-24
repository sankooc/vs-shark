import React  from "react";
import './index.css';
// import docs from "./doc.svg";
import Docs from "./doc.svg";


const ErrPage = () => {
  return (<div className="error-page">
  <main>
<div>
      <Docs/>
</div>
<div>
  <p>We couldn’t find the page you were looking for. It may have been moved, or it just doesn’t exist.</p>
</div>
</main>
</div>)
}
export default ErrPage;