// edit.js
// Handles dynamic loading and form submission for edit.hbs

const pathParts = window.location.pathname.split('/').filter(Boolean);
const entity = pathParts[1];
const id = pathParts[2] || null;
document.getElementById('back-link').href = `/records/${entity}`;

// Fetch entity model and record in parallel
Promise.all([
  fetch(`/api/form/${entity}`).then(r => r.ok ? r.json() : {}),
  fetch(`/api/entity-model/${entity}`).then(r => r.ok ? r.json() : {}),
  id ? fetch(`/api/${entity}/${id}`).then(r => r.ok ? r.json() : {}) : Promise.resolve({})
]).then(([formDef, entityMeta, data]) => {
  // Set page title from form definition if available
  if (formDef && formDef.title) {
    document.getElementById('edit-title').textContent = formDef.title;
    document.title = formDef.title;
  } else {
    document.getElementById('edit-title').textContent = id ? `Edit ${entity} (${id})` : `Add new ${entity}`;
    document.title = id ? `Edit ${entity}` : `Add new ${entity}`;
  }
  let fields = (formDef && formDef.fields) ? formDef.fields : (entityMeta.attributes || []);
  let relations = (formDef && formDef.relations) ? formDef.relations : (entityMeta.relations || []);
  // Render fields
  document.getElementById('attributes-table').innerHTML = fields.map(attr => {
    const name = attr.name || attr;
    const label = attr.label || name;
    return `<tr><td><label for="attr_${name}">${label}</label></td><td><input type="text" id="attr_${name}" name="${name}" value="${data[name] !== undefined ? data[name] : ''}" /></td></tr>`;
  }).join('');
  // Render relations
  document.getElementById('relations-table').innerHTML = relations.map(rel => {
    const name = rel.name || rel;
    const label = rel.label || name;
    return `<tr><td><label for="rel_${name}">${label}</label></td><td><input type="text" id="rel_${name}" name="${name}" value="${data[name] !== undefined ? data[name] : ''}" /></td></tr>`;
  }).join('');
  // Set purpose
  document.getElementById('purpose').textContent = entityMeta.purpose || 'No purpose at all';
});

// Handle form submit
const form = document.getElementById('edit-form');
form.addEventListener('submit', function(e) {
  e.preventDefault();
  const formData = new FormData(form);
  const obj = {};
  for (const [k, v] of formData.entries()) obj[k] = v;
  if (id) obj.id = id;
  fetch(`/api/${entity}`, {
    method: 'POST',
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify(obj)
  }).then(r => r.json()).then(resp => {
    if (resp.id) window.location.href = `/records/${entity}`;
    else alert('Error saving record: ' + (resp.error || 'Unknown error'));
  });
});
