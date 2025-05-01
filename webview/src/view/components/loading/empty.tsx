import "./index.scss";
function Empty() {
  const text = "Select pcap file from menu";
  return (
    <div className="web-main flex align-items-center justify-content-center flex-column">
      <i className="pi pi-inbox"></i>
      <div>{text}</div>
    </div>
  );
}

export default Empty;
