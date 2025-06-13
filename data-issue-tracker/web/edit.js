// edit.js
// Handles dynamic loading and form submission for edit.hbs

const pathParts = window.location.pathname.split('/').filter(Boolean);
const entity = pathParts[1];
const id = pathParts[2] || null;
document.getElementById('back-link').href = `/records/${entity}`;

function InputControl(name, value, label) {
    var self = this;
    self.name = name;
    self.value = value;
    self.label = label || name;
    self.render = () => {
        return `<tr><td><label for="attr_${self.name}">${self.label}</label></td><td><input type="text" id="attr_${self.name}" name="${self.name}" value="${self.value}" /></td></tr>`;
    }
}

function OptionControl(name, value, label, form) {
    var self = this;
    self.name = name;
    self.value = value;
    self.label = label || name;
    self.form = form; // Reference to FormModel for relation options
    self.render = () => {
        return `<tr><td><label for="attr_${self.name}">${self.label}</label></td><td><select id="attr_${self.name}" name="${self.name}">${self.form.create_select_options(self.name, self.value)}</select></td></tr>`;
    }
}

function MultiControl(name, value, label, form) {
    var self = this;
    self.name = name;
    self.value = value;
    self.label = label || name;
    self.form = form; // Reference to FormModel for relation options

    self.create_select_options = (relName) => {
        const rel_list = self.form.relation_values(relName);
        if (!rel_list) return '<option value="">(none)</option>';
        return rel_list.map(item => {
            return `<option value="${item.id}"${item.selected ? ' selected' : ''}>${item.title || item.id}</option>`;
        }).join('');
    };

    self.render = () => {
        return `<tr><td><label for="attr_${self.name}">${self.label}</label></td><td><select  multiple id="attr_${self.name}" name="${self.name}">${self.create_select_options(self.name)}</select></td></tr>`;
    }
}


function ListControl(name, value, label, form) {
    var self = this;
    self.name = name;
    self.value = value;
    self.label = label || name;
    self.form = form; // Reference to FormModel for relation options

    self.create_select_options = (relName) => {
        const rel_list = self.form.relation_values(relName);
        var html = [];
        rel_list.forEach(item => {
            if (item.selected) {
                html.push(`<tr><td>${item.title}</td></tr>`);
            }
        });
        return html.join('');
    };

    self.render = () => {
        var html = ['<tr><td><label for="attr_${self.name}">${self.label}</label></td><td><table>'];
        html.push(self.create_select_options(self.name));
        html.push('</table>');
        return html.join('');
        return `<select  multiple id="attr_${self.name}" name="${self.name}">${self.create_select_options(self.name)}</select></td></tr>`;
    }
}



function FormModel(entity, id) {
    var self = this;
    self.entity = entity;
    self.id = id;
    self.relation = {};

    self.init = () => {
        Promise.all([
            fetch(`/api/form/${entity}`).then(r => r.ok ? r.json() : {}),
            fetch(`/api/entity-model/${entity}`).then(r => r.ok ? r.json() : {}),
            id ? fetch(`/api/${entity}/${id}`).then(r => r.ok ? r.json() : {}) : Promise.resolve({})
        ]).then(([formDef, entityMeta, data]) => {
            self.formDef = formDef;
            self.entityMeta = entityMeta;
            self.data = data;
            console.log('FormModel initialized:', self);
            return Promise.all(entityMeta.relations.map(rel => self.preloadRelationOptions(rel)));
        }).then(self.update_html)
            .catch(err => console.error('Error initializing FormModel:', err));
    }
    self.update_html = () => {
        console.log('update html')
        if (self.formDef && self.formDef.title) {
            document.getElementById('edit-title').textContent = self.formDef.title;
            document.title = self.formDef.title;
        }
        document.getElementById('attributes-table').innerHTML = self.formDef.fields.map(self.create_input_field).join('');
        // Render relations (async)
        document.getElementById('relations-table').innerHTML = self.formDef.relations.map(self.create_relation_entry).join('');
        document.getElementById('purpose').textContent = self.entityMeta.purpose;
    }
    self.create_select_options = (relName, value) => {
        const rel = self.relation[relName];
        if (!rel || !rel.list) return '<option value="">(none)</option>';
        return rel.list.map(item => {
            return `<option value="${item.id}"${item.selected ? ' selected' : ''}>${item.title || item.id}</option>`;
        }).join('');
    };

    self.relation_values = (relName) => {
        const rel = self.relation[relName];
        if (!rel || !rel.list) return [];
        return rel.list;
    };

    self.create_input_field = fld => {
        const name = fld.name;
        const label = fld.label;
        var value = self.data[name] !== undefined ? self.data[name] : '';
        if (fld.name in self.relation) {
            fld.control = new OptionControl(name, value, label, self);
        }
        else {
            fld.control = new InputControl(name, value, label);
        }
        return fld.control.render();
    };

    self.create_relation_entry = (rel) => {
        const name = rel.name;
        const label = rel.label;
        var value = self.data[name] !== undefined ? self.data[name] : '';
        rel.control = new ListControl(name, value, label, self);
        return rel.control.render();
    }

    self.preloadRelationOptions = (rel) => {
        // Preload related entity list and use title_attribute for display
        var entity = rel.type;
        return Promise.all([
            fetch(`/api/${entity}`).then(res => res.ok ? res.json() : []),
            fetch(`/api/entity-model/${entity}`).then(res => res.ok ? res.json() : {})
        ])
            .then(([list, meta]) => {
                console.log(`data loaded for ${entity}`);
                var rec = { data: list, meta: meta };
                rec.list = list.map(item => ({
                    id: item.id,
                    title: item[meta.title_attribute],
                    selected: (self.data[rel.name] || []).indexOf(item.id) >= 0
                }));
                self.relation[rel.name] = rec;

            });
    };


}


document.model = new FormModel(entity, id);
document.model.init();


// Fetch entity model and record in parallel
// Promise.all([
//     fetch(`/api/form/${entity}`).then(r => r.ok ? r.json() : {}),
//     fetch(`/api/entity-model/${entity}`).then(r => r.ok ? r.json() : {}),
//     id ? fetch(`/api/${entity}/${id}`).then(r => r.ok ? r.json() : {}) : Promise.resolve({})
// ]).then(([formDef, entityMeta, data]) => {
//     // Set page title from form definition if available
//     if (formDef && formDef.title) {
//         document.getElementById('edit-title').textContent = formDef.title;
//         document.title = formDef.title;
//     } else {
//         document.getElementById('edit-title').textContent = id ? `Edit ${entity} (${id})` : `Add new ${entity}`;
//         document.title = id ? `Edit ${entity}` : `Add new ${entity}`;
//     }
//     let fields = (formDef && formDef.fields) ? formDef.fields : (entityMeta.attributes || []);
//     let relations = (formDef && formDef.relations) ? formDef.relations : (entityMeta.relations || []);

//     // Helper to resolve relation display value
//     function renderRelationOptions(rel, value) {
//         // Preload related entity list and use title_attribute for display
//         // return fetch(`/api/${rel.name || rel}`)
//         //     .then(res => res.ok ? res.json() : [])
//         //     .then(list => fetch(`/api/entity-model/${rel.type || rel}`)
//         //         .then(metaRes => metaRes.ok ? metaRes.json() : {})
//         //         .then(meta => {
//         //             const titleAttr = meta.title_attribute || 'id';
//         //             return list.map(item => {
//         //                 const selected = Array.isArray(value) ? value.includes(item.id) : value === item.id;
//         //                 return `<option value="${item.id}"${selected ? ' selected' : ''}>${item[titleAttr] || item.id}</option>`;
//         //             }).join('');
//         //         })
//         //     );
//     }

//     // Render fields
//     document.getElementById('attributes-table').innerHTML = fields.map(attr => {
//         const name = attr.name || attr;
//         const label = attr.label || name;
//         return `<tr><td><label for="attr_${name}">${label}</label></td><td><input type="text" id="attr_${name}" name="${name}" value="${data[name] !== undefined ? data[name] : ''}" /></td></tr>`;
//     }).join('');

//     // Render relations (async)
//     // const relTable = document.getElementById('relations-table');
//     // relTable.innerHTML = '';
//     // Promise.all(relations.map(rel => {
//     //     const name = rel.name || rel;
//     //     const label = rel.label || name;
//     //     return renderRelationOptions(rel, data[name]).then(options =>
//     //         `<tr><td><label for="rel_${name}">${label}</label></td><td><select id="rel_${name}" name="${name}">${options}</select></td></tr>`
//     //     );
//     // })).then(rows => {
//     //     relTable.innerHTML = rows.join('');
//     // });

//     // Set purpose
//     document.getElementById('purpose').textContent = entityMeta.purpose || 'No purpose at all';
// });

// Handle form submit
// const form = document.getElementById('edit-form');
// form.addEventListener('submit', function (e) {
//     e.preventDefault();
//     const formData = new FormData(form);
//     const obj = {};
//     for (const [k, v] of formData.entries()) obj[k] = v;
//     if (id) obj.id = id;
//     fetch(`/api/${entity}`, {
//         method: 'POST',
//         headers: { 'Content-Type': 'application/json' },
//         body: JSON.stringify(obj)
//     }).then(r => r.json()).then(resp => {
//         if (resp.id) window.location.href = `/records/${entity}`;
//         else alert('Error saving record: ' + (resp.error || 'Unknown error'));
//     });
// });
