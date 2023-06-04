"use strict";(self.webpackChunksite=self.webpackChunksite||[]).push([[9057],{3905:(e,n,t)=>{t.d(n,{Zo:()=>l,kt:()=>g});var a=t(7294);function r(e,n,t){return n in e?Object.defineProperty(e,n,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[n]=t,e}function i(e,n){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);n&&(a=a.filter((function(n){return Object.getOwnPropertyDescriptor(e,n).enumerable}))),t.push.apply(t,a)}return t}function o(e){for(var n=1;n<arguments.length;n++){var t=null!=arguments[n]?arguments[n]:{};n%2?i(Object(t),!0).forEach((function(n){r(e,n,t[n])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):i(Object(t)).forEach((function(n){Object.defineProperty(e,n,Object.getOwnPropertyDescriptor(t,n))}))}return e}function p(e,n){if(null==e)return{};var t,a,r=function(e,n){if(null==e)return{};var t,a,r={},i=Object.keys(e);for(a=0;a<i.length;a++)t=i[a],n.indexOf(t)>=0||(r[t]=e[t]);return r}(e,n);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)t=i[a],n.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(r[t]=e[t])}return r}var s=a.createContext({}),d=function(e){var n=a.useContext(s),t=n;return e&&(t="function"==typeof e?e(n):o(o({},n),e)),t},l=function(e){var n=d(e.components);return a.createElement(s.Provider,{value:n},e.children)},c="mdxType",m={inlineCode:"code",wrapper:function(e){var n=e.children;return a.createElement(a.Fragment,{},n)}},u=a.forwardRef((function(e,n){var t=e.components,r=e.mdxType,i=e.originalType,s=e.parentName,l=p(e,["components","mdxType","originalType","parentName"]),c=d(t),u=r,g=c["".concat(s,".").concat(u)]||c[u]||m[u]||i;return t?a.createElement(g,o(o({ref:n},l),{},{components:t})):a.createElement(g,o({ref:n},l))}));function g(e,n){var t=arguments,r=n&&n.mdxType;if("string"==typeof e||r){var i=t.length,o=new Array(i);o[0]=u;var p={};for(var s in n)hasOwnProperty.call(n,s)&&(p[s]=n[s]);p.originalType=e,p[c]="string"==typeof e?e:r,o[1]=p;for(var d=2;d<i;d++)o[d]=t[d];return a.createElement.apply(null,o)}return a.createElement.apply(null,t)}u.displayName="MDXCreateElement"},5227:(e,n,t)=>{t.d(n,{ZP:()=>p});var a=t(7462),r=(t(7294),t(3905));const i={toc:[]},o="wrapper";function p(e){let{components:n,...t}=e;return(0,r.kt)(o,(0,a.Z)({},i,t,{components:n,mdxType:"MDXLayout"}),(0,r.kt)("table",null,(0,r.kt)("thead",{parentName:"table"},(0,r.kt)("tr",{parentName:"thead"},(0,r.kt)("th",{parentName:"tr",align:"left"},"Value"),(0,r.kt)("th",{parentName:"tr",align:"left"},"Property in package.json"))),(0,r.kt)("tbody",{parentName:"table"},(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"dev")),(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("a",{parentName:"td",href:"https://docs.npmjs.com/cli/v9/configuring-npm/package-json#devDependencies"},(0,r.kt)("inlineCode",{parentName:"a"},"devDependencies")))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"overrides")),(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("a",{parentName:"td",href:"https://docs.npmjs.com/cli/v9/configuring-npm/package-json#overrides"},(0,r.kt)("inlineCode",{parentName:"a"},"overrides")))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"peer")),(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("a",{parentName:"td",href:"https://docs.npmjs.com/cli/v9/configuring-npm/package-json#peerDependencies"},(0,r.kt)("inlineCode",{parentName:"a"},"peerDependencies")))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"pnpmOverrides")),(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("a",{parentName:"td",href:"https://pnpm.io/package_json#pnpmoverrides"},(0,r.kt)("inlineCode",{parentName:"a"},"pnpm.overrides")))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"prod")),(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("a",{parentName:"td",href:"https://docs.npmjs.com/cli/v9/configuring-npm/package-json#dependencies"},(0,r.kt)("inlineCode",{parentName:"a"},"dependencies")))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"resolutions")),(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("a",{parentName:"td",href:"https://docs.npmjs.com/cli/v9/configuring-npm/package-json#resolutions"},(0,r.kt)("inlineCode",{parentName:"a"},"resolutions")))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"workspace")),(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("a",{parentName:"td",href:"https://docs.npmjs.com/cli/v9/configuring-npm/package-json#version"},(0,r.kt)("inlineCode",{parentName:"a"},"version")))))))}p.isMDXComponent=!0},9872:(e,n,t)=>{t.d(n,{ZP:()=>s});var a=t(7462),r=(t(7294),t(3905)),i=t(7029);const o={toc:[]},p="wrapper";function s(e){let{components:n,...t}=e;return(0,r.kt)(p,(0,a.Z)({},o,t,{components:n,mdxType:"MDXLayout"}),(0,r.kt)(i.Z,{required:!0,mdxType:"Pills"}),(0,r.kt)("p",null,"An array of strings which should match the names of dependencies you've\ninstalled or otherwise referenced in your package.json files. This is used in\ncombination with the ",(0,r.kt)("a",{parentName:"p",href:"#packages-string"},(0,r.kt)("inlineCode",{parentName:"a"},"packages"))," property to determine which\ndependencies should belong to this version group."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-json",metastring:'title="Example of which strings are matched against"',title:'"Example',of:!0,which:!0,strings:!0,are:!0,matched:!0,'against"':!0},'{\n  "name": "not-here",\n  "dependencies": { "HERE": "0.0.0" },\n  "devDependencies": { "HERE": "0.0.0" },\n  "overrides": { "HERE": "0.0.0" },\n  "peerDependencies": { "HERE": "0.0.0" },\n  "pnpm": { "overrides": { "HERE": "0.0.0" } },\n  "resolutions": { "HERE": "0.0.0" }\n}\n')),(0,r.kt)("p",null,"The strings can any combination of exact matches or\n",(0,r.kt)("a",{parentName:"p",href:"https://github.com/isaacs/minimatch"},"minimatch")," glob patterns:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-json",metastring:'title="Examples of valid values"',title:'"Examples',of:!0,valid:!0,'values"':!0},'// match any dependency\n["**"]\n\n// match all dependencies with a certain scope\n["@aws-sdk/**"]\n\n// match specific dependencies by name\n["react", "react-dom"]\n')),(0,r.kt)("admonition",{type:"tip"},(0,r.kt)("p",{parentName:"admonition"},"You can extend syncpack to look in more places by defining your own\n",(0,r.kt)("a",{parentName:"p",href:"/config/custom-types"},(0,r.kt)("inlineCode",{parentName:"a"},"customTypes")),". If you do that, then the names of any\ndependencies found by your ",(0,r.kt)("inlineCode",{parentName:"p"},"customTypes")," can also be targeted using this field.")))}s.isMDXComponent=!0},8343:(e,n,t)=>{t.d(n,{ZP:()=>d});var a=t(7462),r=(t(7294),t(3905)),i=t(7029),o=t(5227);const p={toc:[]},s="wrapper";function d(e){let{components:n,...t}=e;return(0,r.kt)(s,(0,a.Z)({},p,t,{components:n,mdxType:"MDXLayout"}),(0,r.kt)(i.Z,{optional:!0,mdxType:"Pills"}),(0,r.kt)("p",null,"Can be used in combination with the ",(0,r.kt)("a",{parentName:"p",href:"#packages-string"},(0,r.kt)("inlineCode",{parentName:"a"},"packages"))," and\n",(0,r.kt)("a",{parentName:"p",href:"#dependencies-string"},(0,r.kt)("inlineCode",{parentName:"a"},"dependencies"))," properties to narrow further which\ndependencies should belong to this version group. When set, only dependencies\npresent in the named locations will be considered a match for this version\ngroup."),(0,r.kt)("p",null,"The possible values available by default are in the table below:"),(0,r.kt)(o.ZP,{mdxType:"DefaultDependencyTypes"}),(0,r.kt)("admonition",{type:"tip"},(0,r.kt)("p",{parentName:"admonition"},"If you define your own ",(0,r.kt)("a",{parentName:"p",href:"/config/custom-types"},(0,r.kt)("inlineCode",{parentName:"a"},"customTypes")),", their names can\nalso be used in addition to those in the table above.")),(0,r.kt)("p",null,"In this example we define that all dependencies within ",(0,r.kt)("inlineCode",{parentName:"p"},"peerDependencies")," in the\nrepo must use ",(0,r.kt)("inlineCode",{parentName:"p"},'"*"')," as its version number, regardless of what versions of the\nsame dependencies might be used in ",(0,r.kt)("inlineCode",{parentName:"p"},"dependencies")," or ",(0,r.kt)("inlineCode",{parentName:"p"},"devDependencies"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-json",metastring:'title="Ensure peerDependencies always use *"',title:'"Ensure',peerDependencies:!0,always:!0,use:!0,'*"':!0},'{\n  "versionGroups": [\n    {\n      "packages": ["**"],\n      "dependencies": ["**"],\n      "dependencyTypes": ["peer"],\n      "pinVersion": "*"\n    }\n  ]\n}\n')),(0,r.kt)("admonition",{type:"tip"},(0,r.kt)("p",{parentName:"admonition"},"Syncpack config files also support\n",(0,r.kt)("a",{parentName:"p",href:"https://jamiemason.github.io/syncpack/config-file#typescript-intellisense"},"TypeScript IntelliSense"),".")))}d.isMDXComponent=!0},5898:(e,n,t)=>{t.d(n,{ZP:()=>s});var a=t(7462),r=(t(7294),t(3905)),i=t(7029);const o={toc:[]},p="wrapper";function s(e){let{components:n,...t}=e;return(0,r.kt)(p,(0,a.Z)({},o,t,{components:n,mdxType:"MDXLayout"}),(0,r.kt)(i.Z,{optional:!0,mdxType:"Pills"}),(0,r.kt)("p",null,"A short name or description to be displayed in a header in syncpack's output,\nabove the dependencies which matched this group. If a ",(0,r.kt)("inlineCode",{parentName:"p"},"label")," is not set, then\nthe order which this group appears in your config will be used instead."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-json",metastring:'title="Example where a label is used to give context to other Devs"',title:'"Example',where:!0,a:!0,label:!0,is:!0,used:!0,to:!0,give:!0,context:!0,other:!0,'Devs"':!0},'{\n  "versionGroups": [\n    {\n      "label": "AWS dependencies must all share the same version",\n      "packages": ["**"],\n      "dependencies": ["@aws-sdk/**"],\n      "pinVersion": "3.272.0"\n    }\n  ]\n}\n')))}s.isMDXComponent=!0},8904:(e,n,t)=>{t.d(n,{ZP:()=>s});var a=t(7462),r=(t(7294),t(3905)),i=t(7029);const o={toc:[]},p="wrapper";function s(e){let{components:n,...t}=e;return(0,r.kt)(p,(0,a.Z)({},o,t,{components:n,mdxType:"MDXLayout"}),(0,r.kt)(i.Z,{required:!0,mdxType:"Pills"}),(0,r.kt)("p",null,"An array of strings which should match the ",(0,r.kt)("inlineCode",{parentName:"p"},"name")," properties of your\npackage.json files. This is used in combination with the\n",(0,r.kt)("a",{parentName:"p",href:"#dependencies-string"},(0,r.kt)("inlineCode",{parentName:"a"},"dependencies"))," property to determine which dependencies\nshould belong to this version group."),(0,r.kt)("p",null,"The strings can any combination of exact matches or\n",(0,r.kt)("a",{parentName:"p",href:"https://github.com/isaacs/minimatch"},"minimatch")," glob patterns:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-json"},'// match any package\n["**"]\n\n// match any package with a certain scope\n["@my-repo/**"]\n\n// match specific packages by name\n["my-server", "my-client"]\n')))}s.isMDXComponent=!0},7029:(e,n,t)=>{t.d(n,{Z:()=>i});var a=t(7294);const r={pill:"pill_lx6J",gray:"gray_oDNb",red:"red_TkSj",yellow:"yellow_uZjQ",green:"green_VuIk",blue:"blue_niGL",indigo:"indigo_t1co",purple:"purple_wWQZ",pink:"pink_hjd3"};function i(e){let{required:n,optional:t}=e;return a.createElement("p",null,n&&a.createElement("span",{className:`${r.pill} ${r.red}`},"Required"),t&&a.createElement("span",{className:`${r.pill} ${r.gray}`},"Optional"))}},3228:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>u,contentTitle:()=>c,default:()=>f,frontMatter:()=>l,metadata:()=>m,toc:()=>g});var a=t(7462),r=(t(7294),t(3905)),i=t(7029),o=t(8904),p=t(9872),s=t(8343),d=t(5898);const l={id:"standard",title:"Standard"},c="Standard",m={unversionedId:"config/version-groups/standard",id:"config/version-groups/standard",title:"Standard",description:"Defaults to highestSemver but can be optionally changed to lowestSemver.",source:"@site/docs/config/version-groups/standard.mdx",sourceDirName:"config/version-groups",slug:"/config/version-groups/standard",permalink:"/syncpack/config/version-groups/standard",draft:!1,editUrl:"https://github.com/JamieMason/syncpack/tree/master/site/docs/config/version-groups/standard.mdx",tags:[],version:"current",lastUpdatedBy:"Jamie Mason",lastUpdatedAt:1685898288,formattedLastUpdatedAt:"Jun 4, 2023",frontMatter:{id:"standard",title:"Standard"},sidebar:"docs",previous:{title:"Snapped To",permalink:"/syncpack/config/version-groups/snapped-to"}},u={},g=[{value:"<code>preferVersion</code> string",id:"preferversion-string",level:2},{value:"<code>packages</code> string[]",id:"packages-string",level:2},{value:"<code>dependencies</code> string[]",id:"dependencies-string",level:2},{value:"<code>dependencyTypes</code> string[]",id:"dependencytypes-string",level:2},{value:"<code>label</code> string",id:"label-string",level:2}],k={toc:g},h="wrapper";function f(e){let{components:n,...t}=e;return(0,r.kt)(h,(0,a.Z)({},k,t,{components:n,mdxType:"MDXLayout"}),(0,r.kt)("h1",{id:"standard"},"Standard"),(0,r.kt)("p",null,"Defaults to ",(0,r.kt)("inlineCode",{parentName:"p"},"highestSemver")," but can be optionally changed to ",(0,r.kt)("inlineCode",{parentName:"p"},"lowestSemver"),"."),(0,r.kt)("p",null,"To set this as your standard policy, create a version group which applies to\nevery dependency as the last item in your ",(0,r.kt)("inlineCode",{parentName:"p"},"versionGroups")," array. You can also\njust set this for some of the packages if you need to."),(0,r.kt)("admonition",{type:"info"},(0,r.kt)("ul",{parentName:"admonition"},(0,r.kt)("li",{parentName:"ul"},"One of the possible values for the ",(0,r.kt)("a",{parentName:"li",href:"/syncpack/config/version-groups"},(0,r.kt)("inlineCode",{parentName:"a"},"versionGroups")),"\nconfiguration array."),(0,r.kt)("li",{parentName:"ul"},"Learn more in our ",(0,r.kt)("a",{parentName:"li",href:"/syncpack/guide/version-groups"},"guide to Version Groups"),"."))),(0,r.kt)("h2",{id:"preferversion-string"},(0,r.kt)("inlineCode",{parentName:"h2"},"preferVersion")," string"),(0,r.kt)(i.Z,{optional:!0,mdxType:"Pills"}),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-json",metastring:'title="Choose the lowest valid semver version when fixing mismatches"',title:'"Choose',the:!0,lowest:!0,valid:!0,semver:!0,version:!0,when:!0,fixing:!0,'mismatches"':!0},'{\n  "versionGroups": [\n    {\n      "dependencies": ["**"],\n      "packages": ["**"],\n      "preferVersion": "lowestSemver"\n    }\n  ]\n}\n')),(0,r.kt)("h2",{id:"packages-string"},(0,r.kt)("inlineCode",{parentName:"h2"},"packages")," string[]"),(0,r.kt)(o.ZP,{mdxType:"Packages"}),(0,r.kt)("h2",{id:"dependencies-string"},(0,r.kt)("inlineCode",{parentName:"h2"},"dependencies")," string[]"),(0,r.kt)(p.ZP,{mdxType:"Dependencies"}),(0,r.kt)("h2",{id:"dependencytypes-string"},(0,r.kt)("inlineCode",{parentName:"h2"},"dependencyTypes")," string[]"),(0,r.kt)(s.ZP,{mdxType:"DependencyTypes"}),(0,r.kt)("h2",{id:"label-string"},(0,r.kt)("inlineCode",{parentName:"h2"},"label")," string"),(0,r.kt)(d.ZP,{mdxType:"Label"}))}f.isMDXComponent=!0}}]);