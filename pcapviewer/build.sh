npm run reset
rm -rf media/
mkdir media
cd ../simulator/
npm run css
npm run build
cp src/common.ts  ../pcapviewer/src/common.ts
cp src/client.ts  ../pcapviewer/src/client.ts

cp dist/* ../pcapviewer/media/