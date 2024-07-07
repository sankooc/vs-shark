npm run reset
rm -rf media/
mkdir media
cd ../webview/
npm run css
npm run build
cp src/common.ts  ../extension/src/common.ts
cp src/client.ts  ../extension/src/client.ts

cp dist/* ../extension/media/