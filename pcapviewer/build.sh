cd ../simulator/
npx lessc src/editor/app.less src/editor/app.css
npm run build
cp src/common.ts  ../pcapviewer/src/common.ts
cp src/client.ts  ../pcapviewer/src/client.ts
cp dist/app.js ../pcapviewer/media/app.js
cp dist/hex.js ../pcapviewer/media/hex.js
cp src/editor/app.css ../pcapviewer/media/app.css