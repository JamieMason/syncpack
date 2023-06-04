"use strict";(self.webpackChunksite=self.webpackChunksite||[]).push([[4086],{3905:(e,t,r)=>{r.d(t,{Zo:()=>p,kt:()=>m});var n=r(7294);function i(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function o(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function a(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?o(Object(r),!0).forEach((function(t){i(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):o(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function s(e,t){if(null==e)return{};var r,n,i=function(e,t){if(null==e)return{};var r,n,i={},o=Object.keys(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||(i[r]=e[r]);return i}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(i[r]=e[r])}return i}var l=n.createContext({}),c=function(e){var t=n.useContext(l),r=t;return e&&(r="function"==typeof e?e(t):a(a({},t),e)),r},p=function(e){var t=c(e.components);return n.createElement(l.Provider,{value:t},e.children)},u="mdxType",f={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},d=n.forwardRef((function(e,t){var r=e.components,i=e.mdxType,o=e.originalType,l=e.parentName,p=s(e,["components","mdxType","originalType","parentName"]),u=c(r),d=i,m=u["".concat(l,".").concat(d)]||u[d]||f[d]||o;return r?n.createElement(m,a(a({ref:t},p),{},{components:r})):n.createElement(m,a({ref:t},p))}));function m(e,t){var r=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var o=r.length,a=new Array(o);a[0]=d;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s[u]="string"==typeof e?e:i,a[1]=s;for(var c=2;c<o;c++)a[c]=r[c];return n.createElement.apply(null,a)}return n.createElement.apply(null,r)}d.displayName="MDXCreateElement"},7029:(e,t,r)=>{r.d(t,{Z:()=>o});var n=r(7294);const i={pill:"pill_lx6J",gray:"gray_oDNb",red:"red_TkSj",yellow:"yellow_uZjQ",green:"green_VuIk",blue:"blue_niGL",indigo:"indigo_t1co",purple:"purple_wWQZ",pink:"pink_hjd3"};function o(e){let{required:t,optional:r}=e;return n.createElement("p",null,t&&n.createElement("span",{className:`${i.pill} ${i.red}`},"Required"),r&&n.createElement("span",{className:`${i.pill} ${i.gray}`},"Optional"))}},1209:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>c,contentTitle:()=>s,default:()=>d,frontMatter:()=>a,metadata:()=>l,toc:()=>p});var n=r(7462),i=(r(7294),r(3905)),o=r(7029);const a={id:"sort-first",title:"sortFirst"},s="sortFirst string[]",l={unversionedId:"config/sort-first",id:"config/sort-first",title:"sortFirst",description:"When using the format command, determines which fields within package.json",source:"@site/docs/config/sort-first.mdx",sourceDirName:"config",slug:"/config/sort-first",permalink:"/syncpack/config/sort-first",draft:!1,editUrl:"https://github.com/JamieMason/syncpack/tree/master/site/docs/config/sort-first.mdx",tags:[],version:"current",lastUpdatedBy:"Jamie Mason",lastUpdatedAt:1685898288,formattedLastUpdatedAt:"Jun 4, 2023",frontMatter:{id:"sort-first",title:"sortFirst"},sidebar:"docs",previous:{title:"sortAz",permalink:"/syncpack/config/sort-az"},next:{title:"source",permalink:"/syncpack/config/source"}},c={},p=[{value:"Default Value",id:"default-value",level:2}],u={toc:p},f="wrapper";function d(e){let{components:t,...r}=e;return(0,i.kt)(f,(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"sortfirst-string"},(0,i.kt)("inlineCode",{parentName:"h1"},"sortFirst")," string[]"),(0,i.kt)(o.Z,{optional:!0,mdxType:"Pills"}),(0,i.kt)("p",null,"When using the ",(0,i.kt)("inlineCode",{parentName:"p"},"format")," command, determines which fields within package.json\nfiles should appear at the top, and in what order."),(0,i.kt)("h2",{id:"default-value"},"Default Value"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-json",metastring:'title=".syncpackrc"',title:'".syncpackrc"'},'{\n  "sortFirst": ["name", "description", "version", "author"]\n}\n')),(0,i.kt)("admonition",{type:"tip"},(0,i.kt)("p",{parentName:"admonition"},"Syncpack config files also support\n",(0,i.kt)("a",{parentName:"p",href:"https://jamiemason.github.io/syncpack/config-file#typescript-intellisense"},"TypeScript IntelliSense"),".")))}d.isMDXComponent=!0}}]);