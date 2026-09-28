import {readFileSync, writeFileSync} from 'node:fs';
import {createRequire} from 'node:module';
import http from 'node:http';
import {resolve} from 'node:path';
const root=process.cwd();
const require=createRequire(resolve(root,'rhwp-studio/package.json'));
const routes=new Map([['/rhwp.js',['application/javascript','pkg/rhwp.js']],['/rhwp_bg.wasm',['application/wasm','pkg/rhwp_bg.wasm']],['/blank',['application/octet-stream','saved/blank2010.hwp']]]);
const server=http.createServer((req,res)=>{const r=routes.get(req.url); if(!r){res.end('<!doctype html>');return;}res.setHeader('Content-Type',r[0]);res.end(readFileSync(r[1]));});
await new Promise(r=>server.listen(0,'127.0.0.1',r));
let browser;
try {
 browser=await require('puppeteer-core').launch({executablePath:'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true,protocolTimeout:180000});
 const page=await browser.newPage(); const errors=[]; page.on('pageerror',e=>errors.push(e.message));
 await page.goto(`http://127.0.0.1:${server.address().port}`);
 const result=await page.evaluate(async()=>{
  const m=await import('/rhwp.js'); await m.default({module_or_path:'/rhwp_bg.wasm'});
  const bytes=new Uint8Array(await(await fetch('/blank')).arrayBuffer());
  const rows=[];
  const snap=(d)=>{const tree=JSON.parse(d.getPageRenderTree(0));const lines=[]; const walk=n=>{if(n.type==='TextLine') lines.push({bbox:n.bbox,info:n});for(const c of n.children||[])walk(c);};walk(tree);const reopened=new m.HwpDocument(d.exportHwp());const s={pages:d.pageCount(),reopenedPages:reopened.pageCount(),lines,svg:d.renderPageSvg(0)};reopened.free();return s;};
  for(const byId of [false,true]) {
   const d=new m.HwpDocument(bytes);d.insertText(0,0,0,'Title');d.applyCharFormat(0,0,0,5,JSON.stringify({fontSize:2000}));
   for(const [i,t] of ['A','B','','C','D'].entries()){d.insertParagraph(0,i+1);if(t)d.insertText(0,i+1,0,t);}
   d.applyCharFormat(0,2,0,1,JSON.stringify({fontSize:1300,bold:true}));
   const runs=JSON.parse(d.getCharShapeRuns(0,2,0,1));
   if(byId){const r=Array.isArray(runs)?runs[0]:runs.runs[0];d.setCharShapeId(0,4,0,1,r.charShapeId??r.char_shape_id);}else d.applyCharFormat(0,4,0,1,JSON.stringify({fontSize:1300,bold:true}));
   rows.push({name:byId?'set-shape-id':'apply-char-format',runs,...snap(d)});
   d.deleteParagraph(0,3);rows.push({name:(byId?'set-shape-id':'apply-char-format')+'-delete-empty',...snap(d)});d.free();
  }
  return rows;
 });
 for(const row of result){writeFileSync(`output/pr-review/7468-validation/edited-head/wasm-${row.name}.svg`,row.svg);delete row.svg;}
 writeFileSync('output/pr-review/7468-validation/edited-head/wasm-live.json',JSON.stringify({browser:await browser.version(),errors,result},null,2));
 console.log(JSON.stringify({errors,result:result.map(({name,pages,reopenedPages,lines,runs})=>({name,pages,reopenedPages,lineCount:lines.length,runs}))}));
 if(errors.length||result.some(r=>r.pages!==1||r.reopenedPages!==1))throw Error('page or browser assertion failed');
} finally {if(browser)await browser.close();await new Promise(r=>server.close(r));}
