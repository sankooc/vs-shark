npm run reset
rm -rf media/
mkdir media
cd ../webview/
npm run css
npm run build

cp dist/* ../extension/media/