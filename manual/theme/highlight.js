/*!
  Highlight.js v11.1.0 (git: 83ad2fbd99)
  (c) 2006-2024 Ivan Sagalaev and other contributors
  License: BSD-3-Clause
 */
var hljs=function(){"use strict";var e={exports:{}};function t(e){
return e instanceof Map?e.clear=e.delete=e.set=()=>{
throw Error("map is read-only")}:e instanceof Set&&(e.add=e.clear=e.delete=()=>{
throw Error("set is read-only")
}),Object.freeze(e),Object.getOwnPropertyNames(e).forEach((n=>{var r=e[n]
;"object"!=typeof r||Object.isFrozen(r)||t(r)})),e}
e.exports=t,e.exports.default=t;var n=e.exports;class r{constructor(e){
void 0===e.data&&(e.data={}),this.data=e.data,this.isMatchIgnored=!1}
ignoreMatch(){this.isMatchIgnored=!0}}function i(e){
return e.replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&#x27;")
}function s(e,...t){const n=Object.create(null);for(const t in e)n[t]=e[t]
;return t.forEach((e=>{for(const t in e)n[t]=e[t]})),n}const o=e=>!!e.kind
;class a{constructor(e,t){
this.buffer="",this.classPrefix=t.classPrefix,e.walk(this)}addText(e){
this.buffer+=i(e)}openNode(e){if(!o(e))return;let t=e.kind
;t=e.sublanguage?"language-"+t:((e,{prefix:t})=>{if(e.includes(".")){
const n=e.split(".")
;return[`${t}${n.shift()}`,...n.map(((e,t)=>`${e}${"_".repeat(t+1)}`))].join(" ")
}return`${t}${e}`})(t,{prefix:this.classPrefix}),this.span(t)}closeNode(e){
o(e)&&(this.buffer+="</span>")}value(){return this.buffer}span(e){
this.buffer+=`<span class="${e}">`}}class c{constructor(){this.rootNode={
children:[]},this.stack=[this.rootNode]}get top(){
return this.stack[this.stack.length-1]}get root(){return this.rootNode}add(e){
this.top.children.push(e)}openNode(e){const t={kind:e,children:[]}
;this.add(t),this.stack.push(t)}closeNode(){
if(this.stack.length>1)return this.stack.pop()}closeAllNodes(){
for(;this.closeNode(););}toJSON(){return JSON.stringify(this.rootNode,null,4)}
walk(e){return this.constructor._walk(e,this.rootNode)}static _walk(e,t){
return"string"==typeof t?e.addText(t):t.children&&(e.openNode(t),
t.children.forEach((t=>this._walk(e,t))),e.closeNode(t)),e}static _collapse(e){
"string"!=typeof e&&e.children&&(e.children.every((e=>"string"==typeof e))?e.children=[e.children.join("")]:e.children.forEach((e=>{
c._collapse(e)})))}}class l extends c{constructor(e){super(),this.options=e}
addKeyword(e,t){""!==e&&(this.openNode(t),this.addText(e),this.closeNode())}
addText(e){""!==e&&this.add(e)}addSublanguage(e,t){const n=e.root
;n.kind=t,n.sublanguage=!0,this.add(n)}toHTML(){
return new a(this,this.options).value()}finalize(){return!0}}function _(e){
return e?"string"==typeof e?e:e.source:null}function d(...e){
return e.map((e=>_(e))).join("")}function u(...e){return"("+((e=>{
const t=e[e.length-1]
;return"object"==typeof t&&t.constructor===Object?(e.splice(e.length-1,1),t):{}
})(e).capture?"":"?:")+e.map((e=>_(e))).join("|")+")"}function g(e){
return RegExp(e.toString()+"|").exec("").length-1}
const h=/\[(?:[^\\\]]|\\.)*\]|\(\??|\\([1-9][0-9]*)|\\./
;function p(e,{joinWith:t}){let n=0;return e.map((e=>{n+=1;const t=n
;let r=_(e),i="";for(;r.length>0;){const e=h.exec(r);if(!e){i+=r;break}
i+=r.substring(0,e.index),
r=r.substring(e.index+e[0].length),"\\"===e[0][0]&&e[1]?i+="\\"+(Number(e[1])+t):(i+=e[0],
"("===e[0]&&n++)}return i})).map((e=>`(${e})`)).join(t)}
const A="[a-zA-Z]\\w*",b="[a-zA-Z_]\\w*",M="\\b\\d+(\\.\\d+)?",R="(-?)(\\b0[xX][a-fA-F0-9]+|(\\b\\d+(\\.\\d*)?|\\.\\d+)([eE][-+]?\\d+)?)",E="\\b(0b[01]+)",O={
begin:"\\\\[\\s\\S]",relevance:0},f={scope:"string",begin:"'",end:"'",
illegal:"\\n",contains:[O]},C={scope:"string",begin:'"',end:'"',illegal:"\\n",
contains:[O]},T=(e,t,n={})=>{const r=s({scope:"comment",begin:e,end:t,
contains:[]},n);r.contains.push({scope:"doctag",
begin:"[ ]*(?=(TODO|FIXME|NOTE|BUG|OPTIMIZE|HACK|XXX):)",
end:/(TODO|FIXME|NOTE|BUG|OPTIMIZE|HACK|XXX):/,excludeBegin:!0,relevance:0})
;const i=u("I","a","is","so","us","to","at","if","in","it","on",/[A-Za-z]+['](d|ve|re|ll|t|s|n)/,/[A-Za-z]+[-][a-z]+/,/[A-Za-z][a-z]{2,}/)
;return r.contains.push({begin:d(/[ ]+/,"(",i,/[.]?[:]?([.][ ]|[ ])/,"){3}")}),r
},N=T("//","$"),m=T("/\\*","\\*/"),D=T("#","$");var B=Object.freeze({
__proto__:null,MATCH_NOTHING_RE:/\b\B/,IDENT_RE:A,UNDERSCORE_IDENT_RE:b,
NUMBER_RE:M,C_NUMBER_RE:R,BINARY_NUMBER_RE:E,
RE_STARTERS_RE:"!|!=|!==|%|%=|&|&&|&=|\\*|\\*=|\\+|\\+=|,|-|-=|/=|/|:|;|<<|<<=|<=|<|===|==|=|>>>=|>>=|>=|>>>|>>|>|\\?|\\[|\\{|\\(|\\^|\\^=|\\||\\|=|\\|\\||~",
SHEBANG:(e={})=>{const t=/^#![ ]*\//
;return e.binary&&(e.begin=d(t,/.*\b/,e.binary,/\b.*/)),s({scope:"meta",begin:t,
end:/$/,relevance:0,"on:begin":(e,t)=>{0!==e.index&&t.ignoreMatch()}},e)},
BACKSLASH_ESCAPE:O,APOS_STRING_MODE:f,QUOTE_STRING_MODE:C,PHRASAL_WORDS_MODE:{
begin:/\b(a|an|the|are|I'm|isn't|don't|doesn't|won't|but|just|should|pretty|simply|enough|gonna|going|wtf|so|such|will|you|your|they|like|more)\b/
},COMMENT:T,C_LINE_COMMENT_MODE:N,C_BLOCK_COMMENT_MODE:m,HASH_COMMENT_MODE:D,
NUMBER_MODE:{scope:"number",begin:M,relevance:0},C_NUMBER_MODE:{scope:"number",
begin:R,relevance:0},BINARY_NUMBER_MODE:{scope:"number",begin:E,relevance:0},
REGEXP_MODE:{begin:/(?=\/[^/\n]*\/)/,contains:[{scope:"regexp",begin:/\//,
end:/\/[gimuy]*/,illegal:/\n/,contains:[O,{begin:/\[/,end:/\]/,relevance:0,
contains:[O]}]}]},TITLE_MODE:{scope:"title",begin:A,relevance:0},
UNDERSCORE_TITLE_MODE:{scope:"title",begin:b,relevance:0},METHOD_GUARD:{
begin:"\\.\\s*[a-zA-Z_]\\w*",relevance:0},END_SAME_AS_BEGIN:e=>Object.assign(e,{
"on:begin":(e,t)=>{t.data._beginMatch=e[1]},"on:end":(e,t)=>{
t.data._beginMatch!==e[1]&&t.ignoreMatch()}})});function S(e,t){
"."===e.input[e.index-1]&&t.ignoreMatch()}function L(e,t){
void 0!==e.className&&(e.scope=e.className,delete e.className)}function y(e,t){
t&&e.beginKeywords&&(e.begin="\\b("+e.beginKeywords.split(" ").join("|")+")(?!\\.)(?=\\b|\\s)",
e.__beforeBegin=S,e.keywords=e.keywords||e.beginKeywords,delete e.beginKeywords,
void 0===e.relevance&&(e.relevance=0))}function w(e,t){
Array.isArray(e.illegal)&&(e.illegal=u(...e.illegal))}function I(e,t){
if(e.match){
if(e.begin||e.end)throw Error("begin & end are not supported with match")
;e.begin=e.match,delete e.match}}function x(e,t){
void 0===e.relevance&&(e.relevance=1)}const P=(e,t)=>{if(!e.beforeMatch)return
;if(e.starts)throw Error("beforeMatch cannot be used with starts")
;const n=Object.assign({},e);Object.keys(e).forEach((t=>{delete e[t]
})),e.keywords=n.keywords,
e.begin=d(n.beforeMatch,d("(?=",n.begin,")")),e.starts={relevance:0,
contains:[Object.assign(n,{endsParent:!0})]},e.relevance=0,delete n.beforeMatch
},F=["of","and","for","in","not","or","if","then","parent","list","value"]
;function U(e,t,n="keyword"){const r=Object.create(null)
;return"string"==typeof e?i(n,e.split(" ")):Array.isArray(e)?i(n,e):Object.keys(e).forEach((n=>{
Object.assign(r,U(e[n],t,n))})),r;function i(e,n){
t&&(n=n.map((e=>e.toLowerCase()))),n.forEach((t=>{const n=t.split("|")
;r[n[0]]=[e,v(n[0],n[1])]}))}}function v(e,t){
return t?Number(t):(e=>F.includes(e.toLowerCase()))(e)?0:1}const k={},H=e=>{
console.error(e)},j=(e,...t)=>{console.log("WARN: "+e,...t)},G=(e,t)=>{
k[`${e}/${t}`]||(console.log(`Deprecated as of ${e}. ${t}`),k[`${e}/${t}`]=!0)
},K=Error();function z(e,t,{key:n}){let r=0;const i=e[n],s={},o={}
;for(let e=1;e<=t.length;e++)o[e+r]=i[e],s[e+r]=!0,r+=g(t[e-1])
;e[n]=o,e[n]._emit=s,e[n]._multi=!0}function W(e){(e=>{
e.scope&&"object"==typeof e.scope&&null!==e.scope&&(e.beginScope=e.scope,
delete e.scope)})(e),"string"==typeof e.beginScope&&(e.beginScope={
_wrap:e.beginScope}),"string"==typeof e.endScope&&(e.endScope={_wrap:e.endScope
}),(e=>{if(Array.isArray(e.begin)){
if(e.skip||e.excludeBegin||e.returnBegin)throw H("skip, excludeBegin, returnBegin not compatible with beginScope: {}"),
K
;if("object"!=typeof e.beginScope||null===e.beginScope)throw H("beginScope must be object"),
K;z(e,e.begin,{key:"beginScope"}),e.begin=p(e.begin,{joinWith:""})}})(e),(e=>{
if(Array.isArray(e.end)){
if(e.skip||e.excludeEnd||e.returnEnd)throw H("skip, excludeEnd, returnEnd not compatible with endScope: {}"),
K
;if("object"!=typeof e.endScope||null===e.endScope)throw H("endScope must be object"),
K;z(e,e.end,{key:"endScope"}),e.end=p(e.end,{joinWith:""})}})(e)}function V(e){
function t(t,n){return RegExp(_(t),"m"+(e.case_insensitive?"i":"")+(n?"g":""))}
class n{constructor(){
this.matchIndexes={},this.regexes=[],this.matchAt=1,this.position=0}
addRule(e,t){
t.position=this.position++,this.matchIndexes[this.matchAt]=t,this.regexes.push([t,e]),
this.matchAt+=g(e)+1}compile(){0===this.regexes.length&&(this.exec=()=>null)
;const e=this.regexes.map((e=>e[1]));this.matcherRe=t(p(e,{joinWith:"|"
}),!0),this.lastIndex=0}exec(e){this.matcherRe.lastIndex=this.lastIndex
;const t=this.matcherRe.exec(e);if(!t)return null
;const n=t.findIndex(((e,t)=>t>0&&void 0!==e)),r=this.matchIndexes[n]
;return t.splice(0,n),Object.assign(t,r)}}class r{constructor(){
this.rules=[],this.multiRegexes=[],
this.count=0,this.lastIndex=0,this.regexIndex=0}getMatcher(e){
if(this.multiRegexes[e])return this.multiRegexes[e];const t=new n
;return this.rules.slice(e).forEach((([e,n])=>t.addRule(e,n))),
t.compile(),this.multiRegexes[e]=t,t}resumingScanAtSamePosition(){
return 0!==this.regexIndex}considerAll(){this.regexIndex=0}addRule(e,t){
this.rules.push([e,t]),"begin"===t.type&&this.count++}exec(e){
const t=this.getMatcher(this.regexIndex);t.lastIndex=this.lastIndex
;let n=t.exec(e)
;if(this.resumingScanAtSamePosition())if(n&&n.index===this.lastIndex);else{
const t=this.getMatcher(0);t.lastIndex=this.lastIndex+1,n=t.exec(e)}
return n&&(this.regexIndex+=n.position+1,
this.regexIndex===this.count&&this.considerAll()),n}}
if(e.compilerExtensions||(e.compilerExtensions=[]),
e.contains&&e.contains.includes("self"))throw Error("ERR: contains `self` is not supported at the top-level of a language.  See documentation.")
;return e.classNameAliases=s(e.classNameAliases||{}),function n(i,o){const a=i
;if(i.isCompiled)return a
;[L,I,W,P].forEach((e=>e(i,o))),e.compilerExtensions.forEach((e=>e(i,o))),
i.__beforeBegin=null,[y,w,x].forEach((e=>e(i,o))),i.isCompiled=!0;let c=null
;return"object"==typeof i.keywords&&i.keywords.$pattern&&(i.keywords=Object.assign({},i.keywords),
c=i.keywords.$pattern,
delete i.keywords.$pattern),c=c||/\w+/,i.keywords&&(i.keywords=U(i.keywords,e.case_insensitive)),
a.keywordPatternRe=t(c,!0),
o&&(i.begin||(i.begin=/\B|\b/),a.beginRe=t(i.begin),i.end||i.endsWithParent||(i.end=/\B|\b/),
i.end&&(a.endRe=t(i.end)),
a.terminatorEnd=_(i.end)||"",i.endsWithParent&&o.terminatorEnd&&(a.terminatorEnd+=(i.end?"|":"")+o.terminatorEnd)),
i.illegal&&(a.illegalRe=t(i.illegal)),
i.contains||(i.contains=[]),i.contains=[].concat(...i.contains.map((e=>(e=>(e.variants&&!e.cachedVariants&&(e.cachedVariants=e.variants.map((t=>s(e,{
variants:null},t)))),e.cachedVariants?e.cachedVariants:Y(e)?s(e,{
starts:e.starts?s(e.starts):null
}):Object.isFrozen(e)?s(e):e))("self"===e?i:e)))),i.contains.forEach((e=>{n(e,a)
})),i.starts&&n(i.starts,o),a.matcher=(e=>{const t=new r
;return e.contains.forEach((e=>t.addRule(e.begin,{rule:e,type:"begin"
}))),e.terminatorEnd&&t.addRule(e.terminatorEnd,{type:"end"
}),e.illegal&&t.addRule(e.illegal,{type:"illegal"}),t})(a),a}(e)}function Y(e){
return!!e&&(e.endsWithParent||Y(e.starts))}const $=i,X=s,Z=Symbol("nomatch")
;var J=(e=>{const t=Object.create(null),i=Object.create(null),s=[];let o=!0
;const a="Could not find the language '{}', did you forget to load/include a language module?",c={
disableAutodetect:!0,name:"Plain text",contains:[]};let _={
ignoreUnescapedHTML:!1,noHighlightRe:/^(no-?highlight)$/i,
languageDetectRe:/\blang(?:uage)?-([\w-]+)\b/i,classPrefix:"hljs-",
cssSelector:"pre code",languages:null,__emitter:l};function d(e){
return _.noHighlightRe.test(e)}function u(e,t,n){let r="",i=""
;"object"==typeof t?(r=e,
n=t.ignoreIllegals,i=t.language):(G("10.7.0","highlight(lang, code, ...args) has been deprecated."),
G("10.7.0","Please use highlight(code, options) instead.\nhttps://github.com/highlightjs/highlight.js/issues/2277"),
i=e,r=t),void 0===n&&(n=!0);const s={code:r,language:i};O("before:highlight",s)
;const o=s.result?s.result:g(s.language,s.code,n)
;return o.code=s.code,O("after:highlight",o),o}function g(e,n,i,s){
const c=Object.create(null);function l(){if(!N.keywords)return void D.addText(B)
;let e=0;N.keywordPatternRe.lastIndex=0;let t=N.keywordPatternRe.exec(B),n=""
;for(;t;){n+=B.substring(e,t.index)
;const i=f.case_insensitive?t[0].toLowerCase():t[0],s=(r=i,N.keywords[r]);if(s){
const[e,r]=s
;if(D.addText(n),n="",c[i]=(c[i]||0)+1,c[i]<=7&&(S+=r),e.startsWith("_"))n+=t[0];else{
const n=f.classNameAliases[e]||e;D.addKeyword(t[0],n)}}else n+=t[0]
;e=N.keywordPatternRe.lastIndex,t=N.keywordPatternRe.exec(B)}var r
;n+=B.substr(e),D.addText(n)}function d(){null!=N.subLanguage?(()=>{
if(""===B)return;let e=null;if("string"==typeof N.subLanguage){
if(!t[N.subLanguage])return void D.addText(B)
;e=g(N.subLanguage,B,!0,m[N.subLanguage]),m[N.subLanguage]=e._top
}else e=h(B,N.subLanguage.length?N.subLanguage:null)
;N.relevance>0&&(S+=e.relevance),D.addSublanguage(e._emitter,e.language)
})():l(),B=""}function u(e,t){let n=1;for(;void 0!==t[n];){if(!e._emit[n]){n++
;continue}const r=f.classNameAliases[e[n]]||e[n],i=t[n]
;r?D.addKeyword(i,r):(B=i,l(),B=""),n++}}function p(e,t){
return e.scope&&"string"==typeof e.scope&&D.openNode(f.classNameAliases[e.scope]||e.scope),
e.beginScope&&(e.beginScope._wrap?(D.addKeyword(B,f.classNameAliases[e.beginScope._wrap]||e.beginScope._wrap),
B=""):e.beginScope._multi&&(u(e.beginScope,t),B="")),N=Object.create(e,{parent:{
value:N}}),N}function A(e,t,n){let i=((e,t)=>{const n=e&&e.exec(t)
;return n&&0===n.index})(e.endRe,n);if(i){if(e["on:end"]){const n=new r(e)
;e["on:end"](t,n),n.isMatchIgnored&&(i=!1)}if(i){
for(;e.endsParent&&e.parent;)e=e.parent;return e}}
if(e.endsWithParent)return A(e.parent,t,n)}function b(e){
return 0===N.matcher.regexIndex?(B+=e[0],1):(w=!0,0)}function R(e){
const t=e[0],r=n.substr(e.index),i=A(N,e,r);if(!i)return Z;const s=N
;N.endScope&&N.endScope._wrap?(d(),
D.addKeyword(t,N.endScope._wrap)):N.endScope&&N.endScope._multi?(d(),
u(N.endScope,e)):s.skip?B+=t:(s.returnEnd||s.excludeEnd||(B+=t),
d(),s.excludeEnd&&(B=t));do{
N.scope&&!N.isMultiClass&&D.closeNode(),N.skip||N.subLanguage||(S+=N.relevance),
N=N.parent}while(N!==i.parent)
;return i.starts&&p(i.starts,e),s.returnEnd?0:t.length}let E={};function O(t,s){
const a=s&&s[0];if(B+=t,null==a)return d(),0
;if("begin"===E.type&&"end"===s.type&&E.index===s.index&&""===a){
if(B+=n.slice(s.index,s.index+1),!o){const t=Error(`0 width match regex (${e})`)
;throw t.languageName=e,t.badRule=E.rule,t}return 1}
if(E=s,"begin"===s.type)return(e=>{
const t=e[0],n=e.rule,i=new r(n),s=[n.__beforeBegin,n["on:begin"]]
;for(const n of s)if(n&&(n(e,i),i.isMatchIgnored))return b(t)
;return n.skip?B+=t:(n.excludeBegin&&(B+=t),
d(),n.returnBegin||n.excludeBegin||(B=t)),p(n,e),n.returnBegin?0:t.length})(s)
;if("illegal"===s.type&&!i){
const e=Error('Illegal lexeme "'+a+'" for mode "'+(N.scope||"<unnamed>")+'"')
;throw e.mode=N,e}if("end"===s.type){const e=R(s);if(e!==Z)return e}
if("illegal"===s.type&&""===a)return 1
;if(y>1e5&&y>3*s.index)throw Error("potential infinite loop, way more iterations than matches")
;return B+=a,a.length}const f=M(e)
;if(!f)throw H(a.replace("{}",e)),Error('Unknown language: "'+e+'"')
;const C=V(f);let T="",N=s||C;const m={},D=new _.__emitter(_);(()=>{const e=[]
;for(let t=N;t!==f;t=t.parent)t.scope&&e.unshift(t.scope)
;e.forEach((e=>D.openNode(e)))})();let B="",S=0,L=0,y=0,w=!1;try{
for(N.matcher.considerAll();;){
y++,w?w=!1:N.matcher.considerAll(),N.matcher.lastIndex=L
;const e=N.matcher.exec(n);if(!e)break;const t=O(n.substring(L,e.index),e)
;L=e.index+t}return O(n.substr(L)),D.closeAllNodes(),D.finalize(),T=D.toHTML(),{
language:e,value:T,relevance:S,illegal:!1,_emitter:D,_top:N}}catch(t){
if(t.message&&t.message.includes("Illegal"))return{language:e,value:$(n),
illegal:!0,relevance:0,_illegalBy:{message:t.message,index:L,
context:n.slice(L-100,L+100),mode:t.mode,resultSoFar:T},_emitter:D};if(o)return{
language:e,value:$(n),illegal:!1,relevance:0,errorRaised:t,_emitter:D,_top:N}
;throw t}}function h(e,n){n=n||_.languages||Object.keys(t);const r=(e=>{
const t={value:$(e),illegal:!1,relevance:0,_top:c,_emitter:new _.__emitter(_)}
;return t._emitter.addText(e),t})(e),i=n.filter(M).filter(E).map((t=>g(t,e,!1)))
;i.unshift(r);const s=i.sort(((e,t)=>{
if(e.relevance!==t.relevance)return t.relevance-e.relevance
;if(e.language&&t.language){if(M(e.language).supersetOf===t.language)return 1
;if(M(t.language).supersetOf===e.language)return-1}return 0})),[o,a]=s,l=o
;return l.secondBest=a,l}function p(e){let t=null;const n=(e=>{
let t=e.className+" ";t+=e.parentNode?e.parentNode.className:""
;const n=_.languageDetectRe.exec(t);if(n){const t=M(n[1])
;return t||(j(a.replace("{}",n[1])),
j("Falling back to no-highlight mode for this block.",e)),t?n[1]:"no-highlight"}
return t.split(/\s+/).find((e=>d(e)||M(e)))})(e);if(d(n))return
;O("before:highlightElement",{el:e,language:n
}),!_.ignoreUnescapedHTML&&e.children.length>0&&(console.warn("One of your code blocks includes unescaped HTML. This is a potentially serious security risk."),
console.warn("https://github.com/highlightjs/highlight.js/issues/2886"),
console.warn(e)),t=e;const r=t.textContent,s=n?u(r,{language:n,ignoreIllegals:!0
}):h(r);e.innerHTML=s.value,((e,t,n)=>{const r=t&&i[t]||n
;e.classList.add("hljs"),e.classList.add("language-"+r)
})(e,n,s.language),e.result={language:s.language,re:s.relevance,
relevance:s.relevance},s.secondBest&&(e.secondBest={
language:s.secondBest.language,relevance:s.secondBest.relevance
}),O("after:highlightElement",{el:e,result:s,text:r})}let A=!1;function b(){
"loading"!==document.readyState?document.querySelectorAll(_.cssSelector).forEach(p):A=!0
}function M(e){return e=(e||"").toLowerCase(),t[e]||t[i[e]]}
function R(e,{languageName:t}){"string"==typeof e&&(e=[e]),e.forEach((e=>{
i[e.toLowerCase()]=t}))}function E(e){const t=M(e)
;return t&&!t.disableAutodetect}function O(e,t){const n=e;s.forEach((e=>{
e[n]&&e[n](t)}))}
"undefined"!=typeof window&&window.addEventListener&&window.addEventListener("DOMContentLoaded",(()=>{
A&&b()}),!1),Object.assign(e,{highlight:u,highlightAuto:h,highlightAll:b,
highlightElement:p,
highlightBlock:e=>(G("10.7.0","highlightBlock will be removed entirely in v12.0"),
G("10.7.0","Please use highlightElement now."),p(e)),configure:e=>{_=X(_,e)},
initHighlighting:()=>{
b(),G("10.6.0","initHighlighting() deprecated.  Use highlightAll() now.")},
initHighlightingOnLoad:()=>{
b(),G("10.6.0","initHighlightingOnLoad() deprecated.  Use highlightAll() now.")
},registerLanguage:(n,r)=>{let i=null;try{i=r(e)}catch(e){
if(H("Language definition for '{}' could not be registered.".replace("{}",n)),
!o)throw e;H(e),i=c}
i.name||(i.name=n),t[n]=i,i.rawDefinition=r.bind(null,e),i.aliases&&R(i.aliases,{
languageName:n})},unregisterLanguage:e=>{delete t[e]
;for(const t of Object.keys(i))i[t]===e&&delete i[t]},
listLanguages:()=>Object.keys(t),getLanguage:M,registerAliases:R,
autoDetection:E,inherit:X,addPlugin:e=>{(e=>{
e["before:highlightBlock"]&&!e["before:highlightElement"]&&(e["before:highlightElement"]=t=>{
e["before:highlightBlock"](Object.assign({block:t.el},t))
}),e["after:highlightBlock"]&&!e["after:highlightElement"]&&(e["after:highlightElement"]=t=>{
e["after:highlightBlock"](Object.assign({block:t.el},t))})})(e),s.push(e)}
}),e.debugMode=()=>{o=!1},e.safeMode=()=>{o=!0},e.versionString="11.1.0"
;for(const e in B)"object"==typeof B[e]&&n(B[e]);return Object.assign(e,B),e
})({})
;const q=/\b(?!env\b|use\b|include\b|def\b|alias\b|macro\b|pool\b|const\b|return\b|yield\b|typedef\b|struct\b|ptr\b|if\b|else\b|while\b|do\b|for\b|repeat\b|loop\b)[a-zA-Z_][a-zA-Z0-9_.]*\b/
;var Q=Object.freeze({__proto__:null,grmr_rgbasm:e=>{const t={className:"subst",
begin:/\{/,end:/\}/,contains:[{className:"variable",variants:[{
match:/[a-z_][a-z0-9_#@]*/},{match:/(?:[a-z_][a-z0-9_#@]*)?\.[a-z0-9_#@]*/}]}],
illegal:/[^a-z0-9_@#.\\}+ -]/};let n=e.COMMENT(/\/\*/,/\*\//)
;return n.relevance=0,{name:"RGBASM",case_insensitive:!0,
aliases:["rgbds","gbasm","gbz80"],keywords:{$pattern:/[a-z_][a-z0-9_#@]*/,
keyword:"adc add and bit call ccf cpl cp daa dec di ei halt inc jp jr ld ldi ldd ldio ldh nop or pop push res reti ret rlca rlc rla rl rrc rrca rra rr rst sbc scf set sla sra srl stop sub swap xor def bank align round ceil floor div mul sin cos tan asin acos atan atan2 pow log high low isconst sizeof startof strcmp strin strsub strlen strcat strupr strlwr strrin strrpl strfmt include print println printt printi printv printf export ds|0 db|0 dw|0 dl|0 section purge rsreset rsset incbin|10 charmap|10 newcharmap|10 setcharmap|10 fail warn fatal assert static_assert shift opt break macro endm rept for endr load endl pushc popc union nextu endu pushs pops pusho popo if|0 else|0 elif|0 endc|0 rb rw equ equs redef",
literal:["_NARG","_RS","__LINE__","__FILE__","__DATE__","__TIME__","__ISO_8601_LOCAL__","__ISO_8601_UTC__","__UTC_YEAR__","__UTC_MONTH__","__UTC_DAY__","__UTC_HOUR__","__UTC_MINUTE__","__UTC_SECOND__","__RGBDS_MAJOR__","__RGBDS_MINOR__","__RGBDS_PATCH__","__RGBDS_VERSION__"],
_hardware_inc:"_VRAM _VRAM8000 _VRAM8800 _VRAM9000 _SCRN0 _SCRN1 _SRAM _RAM _RAMBANK _OAMRAM _IO _AUD3WAVERAM _HRAM rRAMG rROMB0 rROMB1 rRAMB rP1 P1F_5 P1F_4 P1F_3 P1F_2 P1F_1 P1F_0 P1F_GET_DPAD P1F_GET_BTN P1F_GET_NONE rSB rSC rDIV rTIMA rTMA rTAC TACF_START TACF_STOP TACF_4KHZ TACF_16KHZ TACF_65KHZ TACF_262KHZ rIF rNR10 rAUD1SWEEP AUD1SWEEP_UP AUD1SWEEP_DOWN rNR11 rAUD1LEN rNR12 rAUD1ENV rNR13 rAUD1LOW rNR14 rAUD1HIGH rNR21 rAUD2LEN rNR22 rAUD2ENV rNR23 rAUD2LOW rNR24 rAUD2HIGH rNR30 rAUD3ENA rNR31 rAUD3LEN rNR32 rAUD3LEVEL rNR33 rAUD3LOW rNR34 rAUD3HIGH rNR41 rAUD4LEN rNR42 rAUD4ENV rNR43 rAUD4POLY rNR44 rAUD4GO rNR50 rAUDVOL AUDVOL_VIN_LEFT AUDVOL_VIN_RIGHT rNR51 rAUDTERM AUDTERM_4_LEFT AUDTERM_3_LEFT AUDTERM_2_LEFT AUDTERM_1_LEFT AUDTERM_4_RIGHT AUDTERM_3_RIGHT AUDTERM_2_RIGHT AUDTERM_1_RIGHT rNR52 rAUDENA AUDENA_ON AUDENA_OFF rLCDC LCDCF_OFF LCDCF_ON LCDCF_WIN9800 LCDCF_WIN9C00 LCDCF_WINOFF LCDCF_WINON LCDCF_BG8800 LCDCF_BG8000 LCDCF_BG9800 LCDCF_BG9C00 LCDCF_OBJ8 LCDCF_OBJ16 LCDCF_OBJOFF LCDCF_OBJON LCDCF_BGOFF LCDCF_BGON rSTAT STATF_LYC STATF_MODE10 STATF_MODE01 STATF_MODE00 STATF_LYCF STATF_HBL STATF_VBL STATF_OAM STATF_LCD STATF_BUSY rSCY rSCX rLY rLYC rDMA rBGP rOBP0 rOBP1 rWY rWX rKEY1 rSPD KEY1F_DBLSPEED KEY1F_PREPARE rVBK rHDMA1 rHDMA2 rHDMA3 rHDMA4 rHDMA5 HDMA5F_MODE_GP HDMA5F_MODE_HBL HDMA5F_BUSY rRP RPF_ENREAD RPF_DATAIN RPF_WRITE_HI RPF_WRITE_LO rBCPS BCPSF_AUTOINC rBCPD rOCPS OCPSF_AUTOINC rOCPD rSVBK rSMBK rPCM12 rPCM34 rIE IEF_HILO IEF_SERIAL IEF_TIMER IEF_STAT IEF_VBLANK AUDLEN_DUTY_12_5 AUDLEN_DUTY_25 AUDLEN_DUTY_50 AUDLEN_DUTY_75 AUDENV_UP AUDENV_DOWN AUDHIGH_RESTART AUDHIGH_LENGTH_ON AUDHIGH_LENGTH_OFF BOOTUP_A_DMG BOOTUP_A_CGB BOOTUP_A_MGB BOOTUP_B_CGB BOOTUP_B_AGB CART_COMPATIBLE_DMG CART_COMPATIBLE_DMG_GBC CART_COMPATIBLE_GBC CART_INDICATOR_GB CART_INDICATOR_SGB CART_ROM CART_ROM_MBC1 CART_ROM_MBC1_RAM CART_ROM_MBC1_RAM_BAT CART_ROM_MBC2 CART_ROM_MBC2_BAT CART_ROM_RAM CART_ROM_RAM_BAT CART_ROM_MMM01 CART_ROM_MMM01_RAM CART_ROM_MMM01_RAM_BAT CART_ROM_MBC3_BAT_RTC CART_ROM_MBC3_RAM_BAT_RTC CART_ROM_MBC3 CART_ROM_MBC3_RAM CART_ROM_MBC3_RAM_BAT CART_ROM_MBC5 CART_ROM_MBC5_BAT CART_ROM_MBC5_RAM_BAT CART_ROM_MBC5_RUMBLE CART_ROM_MBC5_RAM_RUMBLE CART_ROM_MBC5_RAM_BAT_RUMBLE CART_ROM_MBC7_RAM_BAT_GYRO CART_ROM_POCKET_CAMERA CART_ROM_BANDAI_TAMA5 CART_ROM_HUDSON_HUC3 CART_ROM_HUDSON_HUC1 CART_ROM_32KB CART_ROM_64KB CART_ROM_128KB CART_ROM_256KB CART_ROM_512KB CART_ROM_1024KB CART_ROM_2048KB CART_ROM_4096KB CART_ROM_8192KB CART_ROM_1152KB CART_ROM_1280KB CART_ROM_1536KB CART_SRAM_NONE CART_SRAM_2KB CART_SRAM_8KB CART_SRAM_32KB CART_SRAM_128KB CART_SRAM_ENABLE CART_SRAM_DISABLE CART_DEST_JAPANESE CART_DEST_NON_JAPANESE PADF_DOWN PADF_UP PADF_LEFT PADF_RIGHT PADF_START PADF_SELECT PADF_B PADF_A PADB_DOWN PADB_UP PADB_LEFT PADB_RIGHT PADB_START PADB_SELECT PADB_B PADB_A SCRN_X SCRN_Y SCRN_X_B SCRN_Y_B SCRN_VX SCRN_VY SCRN_VX_B SCRN_VY_B OAMA_Y OAMA_X OAMA_TILEID OAMA_FLAGS sizeof_OAM_ATTRS OAM_COUNT OAMF_PRI OAMF_YFLIP OAMF_XFLIP OAMF_PAL0 OAMF_PAL1 OAMF_BANK0 OAMF_BANK1 OAMF_PALMASK OAMB_PRI OAMB_YFLIP OAMB_XFLIP OAMB_PAL1 OAMB_BANK1 IEF_LCDC"
},contains:[e.COMMENT(/;/,/$/),n,t,{className:"number",variants:[{
match:/\$[0-9-af]+/},{match:/\b[0-9]+(\.[0-9]+)?/,relevance:0},{match:/&[0-7]+/
},{match:/%[01]+/},{match:/`[0-3]+/}]},{className:"operator",
match:/\*\*|~|\+|-|\*|\/|%|<<|>>|&|\||\^|!=|==|<=|>=|<|>|&&|\|\||!/,relevance:0
},{className:"punctuation",match:/[,[\]:]/,relevance:0},{className:"string",
begin:/"/,end:/"/,contains:[t],relevance:0},{className:"symbol",variants:[{
match:/^[ \t]*[a-z_][a-z0-9_#@]*(?=:)/},{
match:/^[ \t]*(?:[a-z_][a-z0-9_#@]*)?\.[a-z0-9_#@]+(?![a-z0-9_#@])/}],
relevance:0},{className:"type",
match:/\b(?:wram0|vram|romx|rom0|hram|wramx|sram|oam)(?![a-z0-9_#@])/},{
className:"variable",
match:/\b(?:af|bc|de|hl|hli|hld|a|b|c|d|e|h|l|nz|z|nc)(?![a-z0-9_#@])/,
relevance:0}],
illegal:[/\.[a-z0-9_#@]*\./,/\.[0-9]*\./,/^[ \t]*[^ \t\r\na-z_:;]/]}},
grmr_evscript:e=>{
const t=[e.C_BLOCK_COMMENT_MODE,e.C_LINE_COMMENT_MODE,e.NUMBER_MODE,e.QUOTE_STRING_MODE,{
match:/[\[\]()]/,scope:"punctuation"},{
match:/[!&^|<>*/%+-]|&&|\|\||[=!<>]=|<<|>>/,scope:"operator"}];function n(t,n){
return{begin:[/\bdef\b/,/\s+/,t,/\s*/,/\(/],beginScope:{1:"keyword",3:n,
5:"punctuation"},end:[/\)/,/;/],endScope:{1:"punctuation",2:"punctuation"},
endsWithParent:!0,keywords:{$pattern:q,literal:"u8 u16",keyword:"return const"},
contains:[e.C_BLOCK_COMMENT_MODE,e.C_LINE_COMMENT_MODE,{match:",",
scope:"punctuation"},{match:q,scope:"type"}]}}return{name:"evscript",
aliases:"evs",disableAutodetect:!0,
contains:[e.C_LINE_COMMENT_MODE,e.C_BLOCK_COMMENT_MODE,{begin:/#asm/,
beginScope:"meta",end:/#end/,endScope:"meta",relevance:10,subLanguage:"rgbasm"
},{beginKeywords:"include",end:/;/,endScope:"punctuation",
contains:[e.QUOTE_STRING_MODE,e.C_BLOCK_COMMENT_MODE,e.C_LINE_COMMENT_MODE]},{
begin:[/\btypedef\b/,/\s+/,q,/\s*/,/=/],beginScope:{1:"keyword",3:"title.class",
5:"operator"},end:/;/,endScope:"punctuation",scope:"type",keywords:{
built_in:"u8 u16"},contains:[e.C_BLOCK_COMMENT_MODE,e.C_LINE_COMMENT_MODE]},{
begin:[/\bstruct\b/,/\s+/,q,/\s*/,/\{/],beginScope:{1:"keyword",3:"title.class",
5:"punctuation"},end:/\}/,endScope:"punctuation",
contains:[e.C_LINE_COMMENT_MODE,e.C_BLOCK_COMMENT_MODE,{begin:[q,/\s*/,/:/],
beginScope:{1:"property",3:"punctuation"},end:/,/,endScope:"punctuation",
endsWithParent:!0,scope:"type",keywords:{built_in:"u8 u16"}}]},{
begin:[/\benv\b/,/\s+/,q,/\s*/,/\{/],beginScope:{1:"keyword",3:"title.class",
5:"punctuation"},end:/\}/,endScope:"punctuation",
contains:[e.C_LINE_COMMENT_MODE,e.C_BLOCK_COMMENT_MODE,{
begin:[/\buse\b/,/\s+/,q,/\s*/,/;/],beginScope:{1:"keyword",
3:"title.class.inherited",5:"punctuation"}
},n(/\byld\b/,"built_in"),n(/\bret\b/,"built_in"),n(q,"title.function"),{
begin:[/\bpool\b/,/\s*/,/=/],beginScope:{1:"keyword",3:"operator"},end:/;/,
endScope:"punctuation",endsWithParent:!0,contains:t},{
begin:[/\balias\b/,/\s+/,q,/\s*/,/\(/],beginScope:{1:"keyword",
3:"title.function",5:"punctuation"},end:/\)/,endScope:"punctuation",keywords:{
built_in:"u8 u16"},contains:[e.C_BLOCK_COMMENT_MODE,e.C_LINE_COMMENT_MODE,{
match:q,scope:"type"},{match:/,/,scope:"punctuation"}],starts:{
begin:[/=/,/\s*/,q,/\s*/,/\(/],beginScope:{1:"operator",5:"punctuation"},
end:/;/,endScope:"punctuation",contains:[{match:/\$\s*[0-9]*/,scope:"meta"},{
match:/,/,scope:"punctuation"},...t]}},{begin:[/\bmacro\b/,/\s+/,q,/\s*/,/\(/],
beginScope:{1:"keyword",3:"title.function",5:"punctuation"},
end:[/\)/,/\s*/,/=/,/\s*/,q,/\s*/,/;/],endScope:{1:"punctuation",3:"operator",
7:"punctuation"},keywords:{built_in:"u8 u16"},
contains:[e.C_BLOCK_COMMENT_MODE,e.C_LINE_COMMENT_MODE,{match:q,scope:"type"},{
match:/,/,scope:"punctuation"}]}]},{begin:[q,/\s+/,q,/\s*/,/\{/],beginScope:{
1:"type",3:"title.class",5:"punctuation"},starts:{begin:/\{/,
beginScope:"punctuation",end:/\}/,endScope:"punctuation",scope:"code",
keywords:"if else while do for repeat loop",contains:[{match:/;/,
scope:"punctuation"},{begin:[q,/\s*/,/[-+*/%&|^]?=(?!=)|<<=|>>=/],beginScope:{
3:"operator"},end:/;/,endScope:"punctuation",contains:t},{
beginKeywords:"return yield",end:/;/,endScope:"punctuation",
contains:[e.C_BLOCK_COMMENT_MODE]},{begin:[q,/\s+/,q,/\s*/,/=?(?!=)/],
beginScope:{1:"type",3:"variable",5:"operator"},end:/;/,endScope:"punctuation",
contains:t},{begin:[q,/\s+/,/\bptr\b/,/\s+/,q,/\s*/,/=?(?!=)/],beginScope:{
1:"type",3:"keyword",5:"variable",7:"operator"},end:/;/,endScope:"punctuation",
contains:t},...t,"self"]}}]}}});const ee=J;for(const e of Object.keys(Q)){
const t=e.replace("grmr_","").replace("_","-");ee.registerLanguage(t,Q[e])}
return ee}()
;"object"==typeof exports&&"undefined"!=typeof module&&(module.exports=hljs);