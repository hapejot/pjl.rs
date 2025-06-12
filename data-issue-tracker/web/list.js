// list.js
// Handles dynamic loading and rendering of the entity list for list.hbs

document.addEventListener('DOMContentLoaded', () => {
  fetch('/api/entities')
    .then(r => r.ok ? r.json() : [])
    .then(entities => {
      const ul = document.getElementById('entity-list');
      if (!Array.isArray(entities) || entities.length === 0) {
        ul.innerHTML = '<li>No entities found.</li>';
        return;
      }
      ul.innerHTML = entities.map(entity =>
        `<li><a href="/edit/${entity}">Add new ${entity}</a> | <a href="/records/${entity}">Show records</a></li>`
      ).join('');
    })
    .catch(() => {
      const ul = document.getElementById('entity-list');
      ul.innerHTML = '<li style="color:red">Error loading entities.</li>';
    });
});
