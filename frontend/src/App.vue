<template>
  <div class="app-container">
    <h1>Member Management Portal</h1>

    <div class="card form-card">
      <h2>{{ isEditing ? 'Edit Member' : 'Add New Member' }}</h2>
      <form @submit.prevent="saveMember">
        <div class="form-grid">
          <div class="input-group">
            <label>First Name</label>
            <input v-model="form.f_name" type="text" required />
          </div>
          <div class="input-group">
            <label>Middle Name</label>
            <input v-model="form.m_name" type="text" />
          </div>
          <div class="input-group">
            <label>Last Name</label>
            <input v-model="form.l_name" type="text" required />
          </div>
          <div class="input-group">
            <label>Email</label>
            <input v-model="form.email" type="email" required />
          </div>
          <div class="input-group">
            <label>Age</label>
            <input v-model.number="form.age" type="number" required />
          </div>
          <div class="input-group full-width">
            <label>Address</label>
            <input v-model="form.address" type="text" required />
          </div>
        </div>

        <div class="form-actions">
          <button type="submit" :class="isEditing ? 'btn-put' : 'btn-post'">
            {{ isEditing ? 'Update Member (PUT)' : 'Register Member' }}
          </button>
          <button v-if="isEditing" type="button" @click="resetForm" class="btn-cancel">Cancel</button>
        </div>
      </form>
    </div>

    <div class="card table-card">
      <div class="header-row">
        <h2>Member Directory</h2>
        <button @click="fetchMembers" class="btn-refresh">Refresh List</button>
      </div>

      <table v-if="members.length > 0">
        <thead>
          <tr>
            <th>ID</th>
            <th>Full Name</th>
            <th>Email</th>
            <th>Age</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="m in members" :key="m.id">
            <td>{{ m.id }}</td>
            <td>{{ m.f_name }} {{ m.m_name }} {{ m.l_name }}</td>
            <td>{{ m.email }}</td>
            <td>
              {{ m.age }} 
              <button @click="incrementAge(m)" class="btn-patch" title="Patch Age +1">+1</button>
            </td>
            <td>
              <button @click="editMember(m)" class="btn-edit">Edit</button>
              <button @click="deleteMember(m.id)" class="btn-delete">Delete</button>
            </td>
          </tr>
        </tbody>
      </table>
      <p v-else class="empty-msg">No members found. Add one above!</p>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue';

const API_URL = 'http://localhost:7070';
const members = ref([]);
const isEditing = ref(false);
const editingId = ref(null);

const initialForm = {
  f_name: '',
  m_name: '',
  l_name: '',
  email: '',
  address: '',
  age: 18
};

const form = reactive({ ...initialForm });

// --- API ACTIONS ---

// GET: Fetch all
const fetchMembers = async () => {
  try {
    const res = await fetch(`${API_URL}/members`);
    members.value = await res.json();
  } catch (err) {
    console.error("Connection failed", err);
  }
};

// POST & PUT: Save/Update
const saveMember = async () => {
  const method = isEditing.value ? 'PUT' : 'POST';
  const url = isEditing.value ? `${API_URL}/member/${editingId.value}` : `${API_URL}/member`;

  const res = await fetch(url, {
    method,
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(form)
  });

  if (res.ok) {
    resetForm();
    fetchMembers();
  }
};

// PATCH: Partial update (demonstrating the logic)
const incrementAge = async (member) => {
  const res = await fetch(`${API_URL}/member/${member.id}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ age: member.age + 1 })
  });
  if (res.ok) fetchMembers();
};

// DELETE
const deleteMember = async (id) => {
  if (!confirm("Are you sure?")) return;
  await fetch(`${API_URL}/member/delete/${id}`, { method: 'DELETE' });
  fetchMembers();
};

// --- UI LOGIC ---

const editMember = (member) => {
  isEditing.value = true;
  editingId.value = member.id;
  Object.assign(form, member);
};

const resetForm = () => {
  isEditing.value = false;
  editingId.value = null;
  Object.assign(form, initialForm);
};

onMounted(fetchMembers);
</script>

<style>
/* Clean Minimal Styling */
body { background: #f0f2f5; font-family: 'Inter', sans-serif; color: #333; }
.app-container { max-width: 900px; margin: 40px auto; padding: 0 20px; }
.card { background: white; padding: 25px; border-radius: 12px; box-shadow: 0 4px 6px -1px rgba(0,0,0,0.1); margin-bottom: 20px; }
.form-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 15px; }
.full-width { grid-column: span 3; }
.input-group { display: flex; flex-direction: column; }
label { font-size: 0.85rem; font-weight: 600; margin-bottom: 5px; color: #666; }
input { padding: 10px; border: 1px solid #ddd; border-radius: 6px; font-size: 1rem; }
.form-actions { margin-top: 20px; display: flex; gap: 10px; }

table { width: 100%; border-collapse: collapse; margin-top: 15px; }
th { text-align: left; padding: 12px; border-bottom: 2px solid #eee; color: #888; text-transform: uppercase; font-size: 0.75rem; }
td { padding: 12px; border-bottom: 1px solid #f0f0f0; }

button { cursor: pointer; border: none; border-radius: 6px; font-weight: 600; transition: opacity 0.2s; }
button:hover { opacity: 0.8; }
.btn-post { background: #2ecc71; color: white; padding: 12px 24px; }
.btn-put { background: #3498db; color: white; padding: 12px 24px; }
.btn-patch { background: #eee; font-size: 0.7rem; margin-left: 5px; padding: 2px 5px; }
.btn-edit { background: #f1c40f; color: #000; padding: 5px 10px; margin-right: 5px; }
.btn-delete { background: #e74c3c; color: white; padding: 5px 10px; }
.btn-cancel { background: #95a5a6; color: white; padding: 12px 24px; }
.header-row { display: flex; justify-content: space-between; align-items: center; }
</style>
