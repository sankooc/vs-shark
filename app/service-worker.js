if(!self.define){let e,i={};const r=(r,n)=>(r=new URL(r+".js",n).href,i[r]||new Promise((i=>{if("document"in self){const e=document.createElement("script");e.src=r,e.onload=i,document.head.appendChild(e)}else e=r,importScripts(r),i()})).then((()=>{let e=i[r];if(!e)throw new Error(`Module ${r} didn’t register its module`);return e})));self.define=(n,f)=>{const l=e||("document"in self?document.currentScript.src:"")||location.href;if(i[l])return;let o={};const s=e=>r(e,l),t={module:{uri:l},exports:o,require:s};i[l]=Promise.all(n.map((e=>t[e]||s(e)))).then((e=>(f(...e),o)))}}define(["./workbox-099bf95e"],(function(e){"use strict";self.skipWaiting(),e.clientsClaim(),e.precacheAndRoute([{url:"./app.js.LICENSE.txt",revision:"bf9701145b8da8d7eefa99ff534a83e6"},{url:"./main.js",revision:"8d97c162b47106bc72d2687814bde5c3"},{url:"./main.js.LICENSE.txt",revision:"60f6bf9e100e456690e9ab6c9a37bfc2"},{url:"024b32cc7bf399b1a847.woff2",revision:null},{url:"7a24e5496365c81dc5d3.ttf",revision:null},{url:"8931fda1930c3bd21e96.woff",revision:null},{url:"a5c2a53d1ff7a9ff5933.ttf",revision:null},{url:"e86d58fa25c266aabfa1.wasm",revision:null},{url:"fe12718e70823bc5c078.svg",revision:null},{url:"ff0c4cd79b2ffca2de54.eot",revision:null}],{})}));
