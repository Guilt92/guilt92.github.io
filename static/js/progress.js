// Reading progress: updates #progress width based on scroll in article
document.addEventListener('scroll', function(){
  const post = document.querySelector('.post-body');
  const bar = document.getElementById('progress');
  if(!post || !bar) return;
  const rect = post.getBoundingClientRect();
  const total = post.scrollHeight - window.innerHeight;
  const scrolled = Math.min(Math.max(window.scrollY - (post.offsetTop || 0), 0), total || 1);
  const pct = total>0 ? (scrolled/total)*100 : 0;
  bar.style.width = pct + '%';
});
