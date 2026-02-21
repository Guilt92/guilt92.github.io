const state = {posts:[], tags: new Set(), categories: new Set()}

function debounce(fn, wait=200){ let t; return (...a)=>{ clearTimeout(t); t=setTimeout(()=>fn(...a), wait) }}

async function loadIndex(){
  const res = await fetch('content/posts.json');
  const json = await res.json();
  state.posts = json.posts.map(p=>({ ...p })).sort((a,b)=> new Date(b.date)-new Date(a.date));
  state.posts.forEach(p=>{ state.tags = new Set([...state.tags, ...(p.tags||[])]); state.categories.add(p.category) })
}

function renderFilters(){
  const catEl = document.getElementById('categories');
  const tagsEl = document.getElementById('tags');
  const recentEl = document.getElementById('recent');
  catEl.innerHTML = '';
  Array.from(state.categories).forEach(c=>{ const li=document.createElement('li'); li.textContent=c; li.className='muted'; li.onclick=()=>filterByCategory(c); catEl.appendChild(li) })
  tagsEl.innerHTML='';
  Array.from(state.tags).forEach(t=>{ const b=document.createElement('button'); b.className='tag'; b.textContent=t; b.onclick=()=>filterByTag(t); tagsEl.appendChild(b) })
  recentEl.innerHTML='';
  state.posts.slice(0,6).forEach(p=>{ const it=document.createElement('li'); it.textContent=p.title; it.onclick=()=>location.hash=`#/post/${p.slug}`; recentEl.appendChild(it) })
}

function renderHeroFeature(){
  const hero = document.getElementById('hero-feature');
  const featured = state.posts.find(p=>p.featured) || state.posts[0];
  if(!featured) return;
  hero.innerHTML = `<div class="post-card" onclick="location.hash='#/post/${featured.slug}'" style="height:100%"><div class="meta">${featured.date} • ${featured.category}</div><h3>${featured.title}</h3><p class='excerpt'>${featured.excerpt||''}</p></div>`
}

function renderListing(posts){
  const listing = document.getElementById('listing');
  listing.innerHTML = '';
  const grid = document.createElement('div'); grid.className='listing-grid';
  posts.forEach(p=>{
    const c=document.createElement('article'); c.className='post-card';
    c.innerHTML = `<div class="meta">${p.date} • ${p.category}</div><h3>${p.title}</h3><div class="excerpt">${p.excerpt||''}</div>`;
    c.onclick = ()=>location.hash = `#/post/${p.slug}`;
    grid.appendChild(c);
  });
  listing.appendChild(grid);
}

async function renderPost(slug){
  const post = state.posts.find(p=>p.slug===slug);
  if(!post) return renderListing(state.posts);
  const md = await (await fetch(post.file)).text();
  const html = marked.parse(md);
  const listing = document.getElementById('listing');
  listing.innerHTML = `<article class='card'><h1>${post.title}</h1><div class='meta'>${post.date} • ${post.category} • ${post.tags.join(', ')}</div><div class='body'>${html}</div><p><a href="#/">← Back</a></p></article>`;
  Prism.highlightAll();
}

function filterByCategory(cat){ renderListing(state.posts.filter(p=>p.category===cat)); }
function filterByTag(tag){ renderListing(state.posts.filter(p=> (p.tags||[]).includes(tag))); }

function applySearch(q){
  q = q.trim().toLowerCase();
  if(!q) return renderListing(state.posts);
  const results = state.posts.filter(p=>{
    return p.title.toLowerCase().includes(q) || (p.excerpt||'').toLowerCase().includes(q) || (p.tags||[]).join(' ').toLowerCase().includes(q) || (p.category||'').toLowerCase().includes(q)
  });
  renderListing(results);
}

function router(){
  const hash = location.hash || '#/';
  const m = hash.match(/^#\/post\/(.+)$/);
  if(m) renderPost(m[1]); else renderListing(state.posts);
}

window.addEventListener('hashchange', router);
document.addEventListener('DOMContentLoaded', async ()=>{
  document.getElementById('year').textContent = new Date().getFullYear();
  await loadIndex();
  renderFilters();
  renderHeroFeature();
  renderListing(state.posts);
  const searchEl = document.getElementById('search');
  if(searchEl) searchEl.addEventListener('input', debounce(e=>applySearch(e.target.value), 200));
  router();
});

