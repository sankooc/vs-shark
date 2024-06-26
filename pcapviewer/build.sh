npm run reset
cd ../simulator/
npm run css
npm run build
cp src/common.ts  ../pcapviewer/src/common.ts
cp src/client.ts  ../pcapviewer/src/client.ts
cp src/constans.ts  ../pcapviewer/src/constans.ts
cp dist/app.js ../pcapviewer/media/app.js
cp dist/hex.js ../pcapviewer/media/hex.js
cp src/editor/app.css ../pcapviewer/media/app.css