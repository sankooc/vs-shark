declare module '*.less' {
    const resource: {[key: string]: string};
    export = resource;
}
declare module '*.css' {
    interface IClassNames {
      [className: string]: string
    }
    const classNames: IClassNames;
    export = classNames;
}
declare module "*.scss" {
    const content: {[className: string]: string};
    export = content;
}