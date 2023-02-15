"use strict";(self.webpackChunksite=self.webpackChunksite||[]).push([[91],{3905:(e,t,n)=>{n.d(t,{Zo:()=>u,kt:()=>m});var r=n(7294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function c(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function i(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var s=r.createContext({}),p=function(e){var t=r.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):c(c({},t),e)),n},u=function(e){var t=p(e.components);return r.createElement(s.Provider,{value:t},e.children)},l="mdxType",f={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,s=e.parentName,u=i(e,["components","mdxType","originalType","parentName"]),l=p(n),d=o,m=l["".concat(s,".").concat(d)]||l[d]||f[d]||a;return n?r.createElement(m,c(c({ref:t},u),{},{components:n})):r.createElement(m,c({ref:t},u))}));function m(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,c=new Array(a);c[0]=d;var i={};for(var s in t)hasOwnProperty.call(t,s)&&(i[s]=t[s]);i.originalType=e,i[l]="string"==typeof e?e:o,c[1]=i;for(var p=2;p<a;p++)c[p]=n[p];return r.createElement.apply(null,c)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},8866:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>c,default:()=>f,frontMatter:()=>a,metadata:()=>i,toc:()=>p});var r=n(7462),o=(n(7294),n(3905));const a={id:"source-config",title:"source"},c=void 0,i={unversionedId:"source-config",id:"source-config",title:"source",description:"Patterns supported by glob to find",source:"@site/docs/source-config.md",sourceDirName:".",slug:"/source-config",permalink:"/syncpack/source-config",draft:!1,editUrl:"https://github.com/JamieMason/syncpack/tree/master/site/docs/source-config.md",tags:[],version:"current",lastUpdatedBy:"Jamie Mason",lastUpdatedAt:1676486913,formattedLastUpdatedAt:"Feb 15, 2023",frontMatter:{id:"source-config",title:"source"},sidebar:"docs",previous:{title:"sortFirst",permalink:"/syncpack/sort-first-config"},next:{title:"versionGroups",permalink:"/syncpack/version-groups-config"}},s={},p=[{value:"Default Value",id:"default-value",level:2}],u={toc:p},l="wrapper";function f(e){let{components:t,...n}=e;return(0,o.kt)(l,(0,r.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("p",null,"Patterns supported by ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/isaacs/node-glob"},"glob")," to find\npackage.json files you want to manage with syncpack."),(0,o.kt)("h2",{id:"default-value"},"Default Value"),(0,o.kt)("p",null,"Defaulted to match most Projects using Lerna or Yarn Workspaces"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-json"},'{\n  "source": ["package.json", "packages/*/package.json"]\n}\n')),(0,o.kt)("admonition",{type:"info"},(0,o.kt)("p",{parentName:"admonition"},"Your ",(0,o.kt)("inlineCode",{parentName:"p"},"source")," configuration in your ",(0,o.kt)("a",{parentName:"p",href:"/syncpack/config-file"},"config file")," can be\noverridden on an ad hoc basis using multiple ",(0,o.kt)("a",{parentName:"p",href:"/syncpack/source-option"},(0,o.kt)("inlineCode",{parentName:"a"},"--source")),"\noptions \u2013 one for each glob pattern.")))}f.isMDXComponent=!0}}]);