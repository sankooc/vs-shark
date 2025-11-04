import { teamsDarkTheme, teamsLightTheme, Theme, webDarkTheme } from "@fluentui/react-components";

const mapper: Partial<Record<keyof Theme, string>> = {
    "colorNeutralForeground1": "--vscode-foreground",
    "colorNeutralForeground2": "--vscode-descriptionForeground",
    "colorNeutralForeground3": "--vscode-editorLineNumber-foreground",
    "colorNeutralForeground4": "--vscode-editorLineNumber-inactiveForeground",
    "colorNeutralForegroundDisabled": "--vscode-disabledForeground",
    "colorNeutralForegroundInvertedDisabled": "--vscode-disabledForeground",

    "colorNeutralForegroundStaticInverted": "--vscode-editor-background",
    "colorNeutralForegroundInverted": "--vscode-editor-background",
    "colorNeutralForegroundInvertedHover": "--vscode-editor-background",
    "colorNeutralForegroundInvertedPressed": "--vscode-editor-background",
    "colorNeutralForegroundInvertedSelected": "--vscode-editor-background",
    "colorNeutralForegroundInverted2": "--vscode-editor-background",

    "colorNeutralForegroundOnBrand": "--vscode-editor-foreground",
    "colorNeutralForegroundInvertedLink": "--vscode-textLink-foreground",
    "colorNeutralForegroundInvertedLinkHover": "--vscode-textLink-activeForeground",
    "colorNeutralForegroundInvertedLinkPressed": "--vscode-textLink-activeForeground",
    "colorNeutralForegroundInvertedLinkSelected": "--vscode-textLink-activeForeground",

    "colorNeutralBackground1": "--vscode-editor-background",
    "colorNeutralBackground1Hover": "--vscode-editorHoverWidget-background",
    "colorNeutralBackground1Pressed": "--vscode-editor-background",
    "colorNeutralBackground1Selected": "--vscode-editor-background",

    "colorNeutralBackground2": "--vscode-editorWidget-background",
    "colorNeutralBackground2Hover": "--vscode-editorHoverWidget-background",
    "colorNeutralBackground2Pressed": "--vscode-editorWidget-background",
    "colorNeutralBackground2Selected": "--vscode-editorWidget-background",

    "colorNeutralBackground3": "--vscode-sideBar-background",
    "colorNeutralBackground3Hover": "--vscode-sideBarSectionHeader-background",
    "colorNeutralBackground3Pressed": "--vscode-sideBar-background",
    "colorNeutralBackground3Selected": "--vscode-sideBarSectionHeader-background",

    "colorNeutralBackground4": "--vscode-panel-background",
    "colorNeutralBackground4Hover": "--vscode-panelSectionHeader-background",
    "colorNeutralBackground4Pressed": "--vscode-panel-background",
    "colorNeutralBackground4Selected": "--vscode-panelSectionHeader-background",

    "colorNeutralBackground5": "--vscode-editorGroupHeader-tabsBackground",
    "colorNeutralBackground5Hover": "--vscode-tab-activeBackground",
    "colorNeutralBackground5Pressed": "--vscode-tab-inactiveBackground",
    "colorNeutralBackground5Selected": "--vscode-tab-activeBackground",

    "colorNeutralBackgroundInverted": "--vscode-editor-foreground",
    "colorNeutralBackgroundStatic": "--vscode-editor-background",
    "colorNeutralBackgroundAlpha": "--vscode-editorHoverWidget-background",
    "colorNeutralBackgroundAlpha2": "--vscode-editorSuggestWidget-background",

    "colorNeutralCardBackground": "--vscode-editorWidget-background",
    "colorNeutralCardBackgroundHover": "--vscode-editorHoverWidget-background",
    "colorNeutralCardBackgroundPressed": "--vscode-editorWidget-background",
    "colorNeutralCardBackgroundSelected": "--vscode-editorWidget-background",
    "colorNeutralCardBackgroundDisabled": "--vscode-editor-background",

    "colorNeutralStrokeAccessible": "--vscode-contrastBorder",
    "colorNeutralStrokeAccessibleHover": "--vscode-focusBorder",
    "colorNeutralStrokeAccessiblePressed": "--vscode-focusBorder",
    "colorNeutralStrokeAccessibleSelected": "--vscode-focusBorder",

    "colorNeutralStroke1": "--vscode-editorGroup-border",
    "colorNeutralStroke1Hover": "--vscode-tab-border",
    "colorNeutralStroke1Pressed": "--vscode-tab-activeBorder",
    "colorNeutralStroke1Selected": "--vscode-tab-activeBorder",

    "colorNeutralStroke2": "--vscode-editorHoverWidget-border",
    "colorNeutralStroke3": "--vscode-editorWidget-border",
    "colorNeutralStrokeSubtle": "--vscode-editorIndentGuide-background",

    "colorNeutralStrokeDisabled": "--vscode-disabledForeground",
    "colorNeutralStrokeInvertedDisabled": "--vscode-disabledForeground",

    "colorNeutralStencil1": "--vscode-editorGutter-background",
    "colorNeutralStencil2": "--vscode-sideBar-background",
    "colorBackgroundOverlay": "--vscode-widget-shadow",
    "colorScrollbarOverlay": "--vscode-scrollbarSlider-background",

    "colorStrokeFocus1": "--vscode-focusBorder",
    "colorStrokeFocus2": "--vscode-editor-background",

    "colorBrandBackground": "--vscode-button-background",
    "colorBrandBackgroundHover": "--vscode-button-hoverBackground",
    "colorBrandBackgroundPressed": "--vscode-button-activeBackground",
    "colorBrandBackgroundSelected": "--vscode-button-hoverBackground",
    "colorBrandBackgroundStatic": "--vscode-tab-activeBackground",
    "colorBrandForeground1": "--vscode-button-foreground",
    "colorBrandForeground2": "--vscode-button-hoverForeground",
    "colorBrandForegroundLink": "--vscode-textLink-foreground",
    "colorBrandForegroundLinkHover": "--vscode-textLink-activeForeground",
    "colorBrandForegroundLinkPressed": "--vscode-textLink-activeForeground",
    "colorBrandForegroundLinkSelected": "--vscode-textLink-activeForeground",
    "colorBrandForegroundInverted": "--vscode-editor-foreground",
    "colorBrandForegroundInvertedHover": "--vscode-editorHoverWidget-foreground",
    "colorBrandForegroundInvertedPressed": "--vscode-editorHoverWidget-foreground",
    "colorBrandForegroundOnLight": "--vscode-editor-foreground",
    "colorBrandForegroundOnLightHover": "--vscode-editorHoverWidget-foreground",
    "colorBrandForegroundOnLightPressed": "--vscode-editorHoverWidget-foreground",
    "colorBrandForegroundOnLightSelected": "--vscode-editorHoverWidget-foreground",
    "colorBrandStroke1": "--vscode-focusBorder",
    "colorBrandStroke2": "--vscode-button-border",
    "colorBrandStroke2Hover": "--vscode-button-hoverBorder",
    "colorBrandStroke2Pressed": "--vscode-button-activeBorder",
    "colorBrandStroke2Contrast": "--vscode-button-border",
    "colorStatusSuccessBackground1": "--vscode-testing-iconPassed",
    "colorStatusSuccessBackground2": "--vscode-testing-iconPassed",
    "colorStatusSuccessBackground3": "--vscode-testing-iconPassed",
    "colorStatusSuccessForeground1": "--vscode-testing-iconPassed",
    "colorStatusSuccessForeground2": "--vscode-testing-iconPassed",
    "colorStatusSuccessForeground3": "--vscode-testing-iconPassed",
    "colorStatusSuccessBorderActive": "--vscode-testing-iconPassed",
    "colorStatusSuccessForegroundInverted": "--vscode-testing-iconPassed",
    "colorStatusSuccessBorder1": "--vscode-testing-iconPassed",
    "colorStatusSuccessBorder2": "--vscode-testing-iconPassed",

    "colorStatusWarningBackground1": "--vscode-testing-iconQueued",
    "colorStatusWarningBackground2": "--vscode-testing-iconQueued",
    "colorStatusWarningBackground3": "--vscode-testing-iconQueued",
    "colorStatusWarningForeground1": "--vscode-testing-iconQueued",
    "colorStatusWarningForeground2": "--vscode-testing-iconQueued",
    "colorStatusWarningForeground3": "--vscode-testing-iconQueued",
    "colorStatusWarningBorderActive": "--vscode-testing-iconQueued",
    "colorStatusWarningForegroundInverted": "--vscode-testing-iconQueued",
    "colorStatusWarningBorder1": "--vscode-testing-iconQueued",
    "colorStatusWarningBorder2": "--vscode-testing-iconQueued",

    "colorStatusDangerBackground1": "--vscode-testing-iconFailed",
    "colorStatusDangerBackground2": "--vscode-testing-iconFailed",
    "colorStatusDangerBackground3": "--vscode-testing-iconFailed",
    "colorStatusDangerForeground1": "--vscode-testing-iconFailed",
    "colorStatusDangerForeground2": "--vscode-testing-iconFailed",
    "colorStatusDangerForeground3": "--vscode-testing-iconFailed",
    "colorStatusDangerBorderActive": "--vscode-testing-iconFailed",
    "colorStatusDangerForegroundInverted": "--vscode-testing-iconFailed",
    "colorStatusDangerBorder1": "--vscode-testing-iconFailed",
    "colorStatusDangerBorder2": "--vscode-testing-iconFailed",
    "colorStatusDangerBackground3Hover": "--vscode-testing-iconFailed",
    "colorStatusDangerBackground3Pressed": "--vscode-testing-iconFailed",
}


export const buildTheme = () => {
    const isWebview = typeof acquireVsCodeApi === 'function';
    if (!isWebview) {
        import('../../scss/var.scss');
        return webDarkTheme;
    }
    const isDark =
        window.matchMedia &&
        window.matchMedia('(prefers-color-scheme: dark)').matches;
    const base = isDark ? teamsDarkTheme : teamsLightTheme;
    return _buildTheme(base);
}

export const _buildTheme = (base: Theme): Theme => {
    const rs = { ...base };
    const style = getComputedStyle(document.documentElement);
    // const computed = getComputedStyle(document.documentElement);
    // const allVars = {};
    // for (const key in style) {
    //     if (typeof style[key] === 'string' && key.startsWith('--vscode-')) {
    //     }
    // }
    for (const key in mapper) {
        const typedKey = key as keyof Theme;
        const cssVar = mapper[typedKey];
        const value = style.getPropertyValue(cssVar!);
        if (value) {
            (rs as any)[typedKey] = value;
        }
    }
    return rs;
}