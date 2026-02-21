// Build a small client-side Lunr index from /index.json (Hugo output)
async function initSearch(){
  try{
    const res = await fetch('/index.json');
    const json = await res.json();
    const documents = json.pages.map(p=>({ id:p.permalink, title:p.title, content:p.plain, tags:(p.tags||[]).join(' '), category:(p.categories||[]).join(' ') }));
    const idx = lunr(function(){ this.ref('id'); this.field('title'); this.field('content'); this.field('tags'); this.field('category'); documents.forEach(d=>this.add(d)) });
    window.__searchIndex = { idx, documents };
  }catch(e){ console.warn('Search init failed', e) }
}

function search(q){
  if(!window.__searchIndex) return [];
  const res = window.__searchIndex.idx.search(q+'*');
  return res.map(r=> window.__searchIndex.documents.find(d=>d.id===r.ref));
}

document.addEventListener('DOMContentLoaded', ()=>{ initSearch(); const el=document.getElementById('search'); if(el){ el.addEventListener('input', e=>{ const results = search(e.target.value); /* simple result handling: redirect to first result */ if(results && results[0]) location.href = results[0].id }) } });
