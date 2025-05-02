import React, { useState, useEffect } from "react";
import "./IframeWithPlaceholder.scss";

interface IframeWithPlaceholderProps {
  src: string;
  className?: string;
  placeholderContent?: React.ReactNode;
  frameref: React.RefObject<HTMLIFrameElement | null>;
  onLoad?: () => void;
}

export const IframeWithPlaceholder: React.FC<IframeWithPlaceholderProps> = ({
  src,
  className,
  placeholderContent,
  frameref,
  onLoad,
}) => {
  const [isLoading, setIsLoading] = useState(true);
  // const iframe = useRef<HTMLIFrameElement>(null);
  useEffect(() => {
    const iframe = frameref.current;
    if (!iframe) return;

    const handleLoad = () => {
      setIsLoading(false);
      onLoad?.();
    };
    iframe.addEventListener("load", handleLoad);
    return () => iframe.removeEventListener("load", handleLoad);
  }, [onLoad]);

  return (
    <div className={`iframe-container ${className || ""}`}>
      {isLoading && (
        <div className="iframe-placeholder">
          {placeholderContent || (
            <div className="default-placeholder">
              <div className="loading-spinner"></div>
              <div className="loading-text">Loading...</div>
            </div>
          )}
        </div>
      )}
      <iframe
        ref={frameref}
        src={src}
        className={`iframe-content ${isLoading ? "loading" : "loaded"}`}
      />
    </div>
  );
};
