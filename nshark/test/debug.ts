// import fs from "node:fs";
// import path from 'node:path';
// // import network from '../src/networkLayer';
// // import transfer from '../src/transportLayer';
// // import application from '../src/application';
// import { AbstractReaderCreator } from '../src/io';
// import { BasicElement } from "../src/common";
// import { readLoc } from './misc';


// const start = () => {
//     let root = './temp';
//     let dirs = fs.readdirSync(root)
//     let deep = 'root';
//     let param = '';
//     while (true) {
//         if (!dirs.length) {
//             process.exit(0);
//         }
//         if (dirs.indexOf('bin.hex') > -1) {
//             break;
//         }
//         deep = param
//         param = dirs[0];
//         root = path.join(root, param)
//         dirs = fs.readdirSync(root)
//     }
//     const filepath = path.join(root, 'bin.hex');
//     readLoc(filepath, (arr) => {
//         const creator = new AbstractReaderCreator()
//         const ele = new BasicElement('debug:', creator, arr.length, arr);
//         let visitor;
//         switch (deep) {
//             case 'data':
//                 // visitor = network.createVisitor(param);
//                 break;
//             case 'ip':
//                 // visitor = transfer.createVisitor(parseInt(param))
//                 break;
//             case 'udp':
//                 {
//                     const ports = param.split('-')
//                     const [ sourcePort, destPort ] = ports.map((v) => parseInt(v) )
//                     // visitor = application.createVisitor(sourcePort, destPort)
//                 }
//                 break;
//             case 'tcp':
//                 {
//                     const ports = param.split('-')
//                     const [ sourcePort, destPort ] = ports.map((v) => parseInt(v) )
//                     // visitor = application.createVisitor(sourcePort, destPort)
//                 }
//                 break;
//             default:
//                 console.log('path', root);
//                 console.log('deep', deep);
//                 console.log('param', param);
//                 return;

//         }
//         visitor.visit(ele)
//     })

// }


// start();