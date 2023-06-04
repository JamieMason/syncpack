"use strict";(self.webpackChunksite=self.webpackChunksite||[]).push([[9066],{3905:(e,n,t)=>{t.d(n,{Zo:()=>s,kt:()=>m});var r=t(7294);function i(e,n,t){return n in e?Object.defineProperty(e,n,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[n]=t,e}function o(e,n){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);n&&(r=r.filter((function(n){return Object.getOwnPropertyDescriptor(e,n).enumerable}))),t.push.apply(t,r)}return t}function a(e){for(var n=1;n<arguments.length;n++){var t=null!=arguments[n]?arguments[n]:{};n%2?o(Object(t),!0).forEach((function(n){i(e,n,t[n])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):o(Object(t)).forEach((function(n){Object.defineProperty(e,n,Object.getOwnPropertyDescriptor(t,n))}))}return e}function c(e,n){if(null==e)return{};var t,r,i=function(e,n){if(null==e)return{};var t,r,i={},o=Object.keys(e);for(r=0;r<o.length;r++)t=o[r],n.indexOf(t)>=0||(i[t]=e[t]);return i}(e,n);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)t=o[r],n.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(i[t]=e[t])}return i}var p=r.createContext({}),l=function(e){var n=r.useContext(p),t=n;return e&&(t="function"==typeof e?e(n):a(a({},n),e)),t},s=function(e){var n=l(e.components);return r.createElement(p.Provider,{value:n},e.children)},d="mdxType",u={inlineCode:"code",wrapper:function(e){var n=e.children;return r.createElement(r.Fragment,{},n)}},f=r.forwardRef((function(e,n){var t=e.components,i=e.mdxType,o=e.originalType,p=e.parentName,s=c(e,["components","mdxType","originalType","parentName"]),d=l(t),f=i,m=d["".concat(p,".").concat(f)]||d[f]||u[f]||o;return t?r.createElement(m,a(a({ref:n},s),{},{components:t})):r.createElement(m,a({ref:n},s))}));function m(e,n){var t=arguments,i=n&&n.mdxType;if("string"==typeof e||i){var o=t.length,a=new Array(o);a[0]=f;var c={};for(var p in n)hasOwnProperty.call(n,p)&&(c[p]=n[p]);c.originalType=e,c[d]="string"==typeof e?e:i,a[1]=c;for(var l=2;l<o;l++)a[l]=t[l];return r.createElement.apply(null,a)}return r.createElement.apply(null,t)}f.displayName="MDXCreateElement"},7029:(e,n,t)=>{t.d(n,{Z:()=>o});var r=t(7294);const i={pill:"pill_lx6J",gray:"gray_oDNb",red:"red_TkSj",yellow:"yellow_uZjQ",green:"green_VuIk",blue:"blue_niGL",indigo:"indigo_t1co",purple:"purple_wWQZ",pink:"pink_hjd3"};function o(e){let{required:n,optional:t}=e;return r.createElement("p",null,n&&r.createElement("span",{className:`${i.pill} ${i.red}`},"Required"),t&&r.createElement("span",{className:`${i.pill} ${i.gray}`},"Optional"))}},6479:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>l,contentTitle:()=>c,default:()=>f,frontMatter:()=>a,metadata:()=>p,toc:()=>s});var r=t(7462),i=(t(7294),t(3905)),o=t(7029);const a={id:"indent",title:"indent"},c="indent string",p={unversionedId:"config/indent",id:"config/indent",title:"indent",description:"The character(s) to be used to indent your package.json files when writing to",source:"@site/docs/config/indent.mdx",sourceDirName:"config",slug:"/config/indent",permalink:"/syncpack/config/indent",draft:!1,editUrl:"https://github.com/JamieMason/syncpack/tree/master/site/docs/config/indent.mdx",tags:[],version:"current",lastUpdatedBy:"Jamie Mason",lastUpdatedAt:1685898288,formattedLastUpdatedAt:"Jun 4, 2023",frontMatter:{id:"indent",title:"indent"},sidebar:"docs",previous:{title:"filter",permalink:"/syncpack/config/filter"},next:{title:"semverGroups",permalink:"/syncpack/config/semver-groups"}},l={},s=[],d={toc:s},u="wrapper";function f(e){let{components:n,...t}=e;return(0,i.kt)(u,(0,r.Z)({},d,t,{components:n,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"indent-string"},(0,i.kt)("inlineCode",{parentName:"h1"},"indent")," string"),(0,i.kt)(o.Z,{optional:!0,mdxType:"Pills"}),(0,i.kt)("p",null,"The character(s) to be used to indent your package.json files when writing to\ndisk."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-json",metastring:'title=".syncpackrc"',title:'".syncpackrc"'},'{\n  "indent": "  "\n}\n')),(0,i.kt)("admonition",{type:"tip"},(0,i.kt)("p",{parentName:"admonition"},"Syncpack config files also support\n",(0,i.kt)("a",{parentName:"p",href:"https://jamiemason.github.io/syncpack/config-file#typescript-intellisense"},"TypeScript IntelliSense"),".")),(0,i.kt)("admonition",{type:"info"},(0,i.kt)("p",{parentName:"admonition"},"The ",(0,i.kt)("inlineCode",{parentName:"p"},"indent")," configuration in your ",(0,i.kt)("a",{parentName:"p",href:"/syncpack/config-file"},"config file")," can be\noverridden on an ad hoc basis using the ",(0,i.kt)("a",{parentName:"p",href:"/syncpack/option/indent"},(0,i.kt)("inlineCode",{parentName:"a"},"--indent")),"\noption.")))}f.isMDXComponent=!0}}]);