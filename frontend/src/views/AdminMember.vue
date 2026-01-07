<template>
    <div class="card shadow-sm border-0">
        <div
            class="card-header bg-white py-3 d-flex justify-content-between align-items-center"
        >
            <h4 class="mb-0 fw-bold">Member Directory</h4>
            <span class="badge bg-primary">{{ members.length }} Members</span>
        </div>
        <div class="table-responsive">
            <table class="table table-hover align-middle mb-0">
                <thead class="table-light">
                    <tr>
                        <th>ID</th>
                        <th>Name</th>
                        <th>Email</th>
                        <th>Location</th>
                        <th class="text-end">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="m in members" :key="m.id">
                        <td>#{{ m.id }}</td>
                        <td class="fw-bold">{{ m.f_name }} {{ m.l_name }}</td>
                        <td>{{ m.email }}</td>
                        <td>
                            <small class="text-muted">{{ m.address }}</small>
                        </td>
                        <td class="text-end">
                            <button
                                @click="removeMember(m.id)"
                                class="btn btn-outline-danger btn-sm"
                            >
                                Delete
                            </button>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import axios from "axios";

const members = ref([]);

const fetchMembers = async () => {
    const res = await axios.get("http://localhost:7070/members");
    members.value = res.data;
};

const removeMember = async (id) => {
    if (confirm("Are you sure?")) {
        await axios.delete(`http://localhost:7070/members/${id}`);
        fetchMembers();
    }
};

onMounted(fetchMembers);
</script>
