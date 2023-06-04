"use strict";(self.webpackChunksite=self.webpackChunksite||[]).push([[9619],{3905:(e,t,n)=>{n.d(t,{Zo:()=>s,kt:()=>m});var r=n(7294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function a(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function c(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},i=Object.keys(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var p=r.createContext({}),l=function(e){var t=r.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):a(a({},t),e)),n},s=function(e){var t=l(e.components);return r.createElement(p.Provider,{value:t},e.children)},d="mdxType",u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},f=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,i=e.originalType,p=e.parentName,s=c(e,["components","mdxType","originalType","parentName"]),d=l(n),f=o,m=d["".concat(p,".").concat(f)]||d[f]||u[f]||i;return n?r.createElement(m,a(a({ref:t},s),{},{components:n})):r.createElement(m,a({ref:t},s))}));function m(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var i=n.length,a=new Array(i);a[0]=f;var c={};for(var p in t)hasOwnProperty.call(t,p)&&(c[p]=t[p]);c.originalType=e,c[d]="string"==typeof e?e:o,a[1]=c;for(var l=2;l<i;l++)a[l]=n[l];return r.createElement.apply(null,a)}return r.createElement.apply(null,n)}f.displayName="MDXCreateElement"},7029:(e,t,n)=>{n.d(t,{Z:()=>i});var r=n(7294);const o={pill:"pill_lx6J",gray:"gray_oDNb",red:"red_TkSj",yellow:"yellow_uZjQ",green:"green_VuIk",blue:"blue_niGL",indigo:"indigo_t1co",purple:"purple_wWQZ",pink:"pink_hjd3"};function i(e){let{required:t,optional:n}=e;return r.createElement("p",null,t&&r.createElement("span",{className:`${o.pill} ${o.red}`},"Required"),n&&r.createElement("span",{className:`${o.pill} ${o.gray}`},"Optional"))}},5948:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>c,default:()=>f,frontMatter:()=>a,metadata:()=>p,toc:()=>s});var r=n(7462),o=(n(7294),n(3905)),i=n(7029);const a={id:"indent",title:"--indent"},c="--indent string",p={unversionedId:"option/indent",id:"option/indent",title:"--indent",description:"The character(s) to be used to indent your package.json files when writing to",source:"@site/docs/option/indent.mdx",sourceDirName:"option",slug:"/option/indent",permalink:"/syncpack/option/indent",draft:!1,editUrl:"https://github.com/JamieMason/syncpack/tree/master/site/docs/option/indent.mdx",tags:[],version:"current",lastUpdatedBy:"Jamie Mason",lastUpdatedAt:1685898288,formattedLastUpdatedAt:"Jun 4, 2023",frontMatter:{id:"indent",title:"--indent"},sidebar:"docs",previous:{title:"--filter",permalink:"/syncpack/option/filter"},next:{title:"--semver-range",permalink:"/syncpack/option/semver-range"}},l={},s=[],d={toc:s},u="wrapper";function f(e){let{components:t,...n}=e;return(0,o.kt)(u,(0,r.Z)({},d,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"--indent-string"},(0,o.kt)("inlineCode",{parentName:"h1"},"--indent")," string"),(0,o.kt)(i.Z,{optional:!0,mdxType:"Pills"}),(0,o.kt)("p",null,"The character(s) to be used to indent your package.json files when writing to\ndisk. This can be used to override your ",(0,o.kt)("a",{parentName:"p",href:"/syncpack/config/indent"},(0,o.kt)("inlineCode",{parentName:"a"},"indent")),"\nconfiguration on an ad hoc basis."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre"},'syncpack format --indent "  "\n')))}f.isMDXComponent=!0}}]);