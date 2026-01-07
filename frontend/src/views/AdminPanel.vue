<template>
    <div class="admin-container">
        <div class="d-flex justify-content-between align-items-center mb-4">
            <h2 class="fw-bold">Administrative Control</h2>
            <div class="badge bg-dark p-2">
                Total Members: {{ members.length }}
            </div>
        </div>

        <div class="card shadow-sm border-0">
            <div class="table-responsive">
                <table class="table table-hover mb-0">
                    <thead class="table-dark">
                        <tr>
                            <th>ID</th>
                            <th>Full Name</th>
                            <th>Email</th>
                            <th>Address</th>
                            <th class="text-end">Actions</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr v-for="m in members" :key="m.id">
                            <td>#{{ m.id }}</td>
                            <td class="fw-bold">
                                {{ m.f_name }} {{ m.m_name }} {{ m.l_name }}
                            </td>
                            <td>{{ m.email }}</td>
                            <td>
                                <small>{{ m.address }}</small>
                            </td>
                            <td class="text-end">
                                <button
                                    @click="deleteMember(m.id)"
                                    class="btn btn-outline-danger btn-sm"
                                >
                                    Remove
                                </button>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import axios from "axios";

const members = ref([]);

const fetchData = async () => {
    try {
        const res = await axios.get("http://localhost:7070/members");
        members.value = res.data;
    } catch (e) {
        console.error("Access denied or server error");
    }
};

const deleteMember = async (id) => {
    if (confirm("Permanently delete this member?")) {
        await axios.delete(`http://localhost:7070/members/${id}`);
        fetchData();
    }
};

onMounted(fetchData);
</script>
