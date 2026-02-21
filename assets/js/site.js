const state = {posts:[], index: {}, tags: new Set(), categories: new Set()}

async function loadIndex(){
  const res = await fetch('content/posts.json');
  const json = await res.json();
  state.posts = json.posts.sort((a,b)=> new Date(b.date)-new Date(a.date));
  state.posts.forEach(p=>{ p.id = p.slug; state.tags = new Set([...state.tags, ...p.tags]); state.categories.add(p.category) })
}

function renderFilters(){
  const catEl = document.getElementById('categories');
  const tagsEl = document.getElementById('tags');
  catEl.innerHTML = '';
  Array.from(state.categories).forEach(c=>{ const li=document.createElement('li'); li.textContent=c; li.onclick=()=>filterByCategory(c); catEl.appendChild(li) })
  tagsEl.innerHTML='';
  Array.from(state.tags).forEach(t=>{ const li=document.createElement('li'); li.textContent=t; li.onclick=()=>filterByTag(t); tagsEl.appendChild(li) })
}

function renderListing(posts){
  const listing = document.getElementById('listing');
  document.getElementById('post').classList.add('hidden');
  listing.innerHTML = '';
  const grid = document.createElement('div'); grid.className='listing-grid';
  posts.forEach(p=>{
    const c=document.createElement('article'); c.className='card';
    c.innerHTML = `<div class="meta">${p.date} • ${p.category}</div><h3>${p.title}</h3><div class="excerpt">${p.excerpt||''}</div>`;
    c.onclick = ()=>location.hash = `#/post/${p.slug}`;
    grid.appendChild(c);
  });
  listing.appendChild(grid);
}

async function renderPost(slug){
  const post = state.posts.find(p=>p.slug===slug);
  if(!post) return renderListing(state.posts);
  const postEl = document.getElementById('post');
  const md = await (await fetch(post.file)).text();
  const html = marked.parse(md);
  postEl.innerHTML = `<h1>${post.title}</h1><div class="meta">${post.date} • ${post.category} • ${post.tags.join(', ')}</div><div class="body">${html}</div><p><a href="#/">← Back</a></p>`;
  postEl.classList.remove('hidden');
  document.getElementById('listing').innerHTML = '';
  Prism.highlightAll();
}

function filterByCategory(cat){
  const posts = state.posts.filter(p=>p.category===cat);
  renderListing(posts);
}

function filterByTag(tag){
  const posts = state.posts.filter(p=>p.tags.includes(tag));
  renderListing(posts);
}

function applySearch(q){
  q = q.trim().toLowerCase();
  if(!q) return renderListing(state.posts);
  const results = state.posts.filter(p=>{
    return p.title.toLowerCase().includes(q) || (p.excerpt||'').toLowerCase().includes(q) || p.tags.join(' ').toLowerCase().includes(q) || p.category.toLowerCase().includes(q)
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
  renderListing(state.posts);
  document.getElementById('search').addEventListener('input', e=>applySearch(e.target.value));
  router();
});
