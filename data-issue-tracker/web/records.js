// records.js
// Handles dynamic loading and rendering of entity records for records.hbs

// Robust entity extraction for /records/{entity}
const pathParts = window.location.pathname.split('/').filter(Boolean);
const entity = pathParts[pathParts.length - 1];
document.getElementById('entity-title').textContent = entity + 's';
document.getElementById('add-link').href = `/edit/${entity}`;

Promise.all([
    fetch(`/api/view/${entity}`).then(r => r.ok ? r.json() : {}),
    fetch(`/api/${entity}`).then(r => r.ok ? r.json() : []),
    fetch(`/api/entity-model/${entity}`).then(r => r.ok ? r.json() : {})
]).then(([viewDef, records, entityModel]) => {
    // Set page title from view definition if available
    if (viewDef && viewDef.title) {
        document.getElementById('entity-title').textContent = viewDef.title;
        document.title = viewDef.title;
    } else {
        document.getElementById('entity-title').textContent = entity + 's';
        document.title = entity + 's';
    }
    let fields = (viewDef && viewDef.fields) ? viewDef.fields : [];
    // Helper to resolve relation display value
    function renderRelationValue(field, value) {
        if (!value) return '';
        // If the field is a relation, try to resolve the title_attribute
        const rel = (entityModel.relations || []).find(r => r.name === field.name);
        if (rel) {
            // Try to fetch the related entity synchronously (not ideal, but works for small datasets)
            // If value is an array (many-to-many), map each
            if (Array.isArray(value)) {
                return value.map(v => window._relationCache?.[rel.type]?.[v] || v).join(', ');
            } else {
                return window._relationCache?.[rel.type]?.[value] || value;
            }
        }
        return value;
    }
    // Preload all related entities for this table (cache by id)
    window._relationCache = window._relationCache || {};
    const rels = (entityModel.relations || []).filter(r => fields.some(f => f.name === r.name));
    Promise.all(rels.map(r =>
        fetch(`/api/${r.type}`).then(res => res.ok ? res.json() : []).then(list => {
            window._relationCache[r.type] = {};
            // Find title_attribute for this related entity
            fetch(`/api/entity-model/${r.type}`).then(metaRes => metaRes.ok ? metaRes.json() : {}).then(meta => {
                const titleAttr = meta.title_attribute || 'id';
                list.forEach(item => {
                    window._relationCache[r.type][item.id] = item[titleAttr] || item.id;
                });
                // After all relations loaded, re-render table
                if (Object.keys(window._relationCache).length === rels.length) {
                    renderTable();
                }
            });
        })
    ));
    // Fallback render if no relations
    if (!rels.length) renderTable();
    function renderTable() {
        // Render header
        const headerRow = document.getElementById('table-header');
        headerRow.innerHTML = `<th>#</th>` + fields.map(f => `<th>${f.label || f.name}</th>`).join('') + '<th>Actions</th>';
        // Render body
        const body = document.getElementById('table-body');
        if (!Array.isArray(records) || records.length === 0) {
            body.innerHTML = `<tr><td colspan="${fields.length + 2}" style="color:gray">No records found.</td></tr>`;
            return;
        }
        body.innerHTML = records.map((rec, idx) => {
            return `<tr><td>${idx + 1}</td>` +
                fields.map(f => `<td>${renderRelationValue(f, rec[f.name])}</td>`).join('') +
                `<td>${rec.id ? `<a href="/edit/${entity}/${rec.id}">Edit</a>` : '<span style="color:gray">No ID</span>'}</td></tr>`;
        }).join('');
    }
}).catch(err => {
    document.getElementById('table-body').innerHTML = `<tr><td colspan="2" style="color:red">Error loading records: ${err}</td></tr>`;
});
