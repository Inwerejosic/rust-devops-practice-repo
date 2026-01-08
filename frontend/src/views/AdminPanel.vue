<template>
    <div class="admin-panel">
        <div class="d-flex justify-content-between align-items-center mb-4">
            <div>
                <h2 class="fw-bold mb-0">Member Management</h2>
                <p class="text-muted">
                    View, edit, or remove community members
                </p>
            </div>
            <div class="text-end">
                <div class="badge bg-primary rounded-pill px-3 py-2">
                    Total Members: {{ members.length }}
                </div>
            </div>
        </div>

        <div class="card border-0 shadow-sm rounded-4 overflow-hidden">
            <div class="table-responsive">
                <table class="table table-hover align-middle mb-0">
                    <thead class="table-light">
                        <tr>
                            <th class="ps-4">Member</th>
                            <th>Email</th>
                            <th>Status</th>
                            <th>Location</th>
                            <th class="text-end pe-4">Actions</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr v-for="m in members" :key="m.id">
                            <td class="ps-4">
                                <div class="d-flex align-items-center">
                                    <div class="avatar-circle me-3">
                                        {{ m.f_name[0] }}{{ m.l_name[0] }}
                                    </div>
                                    <div>
                                        <div class="fw-bold">
                                            {{ m.f_name }} {{ m.m_name }}
                                            {{ m.l_name }}
                                        </div>
                                        <small class="text-muted"
                                            >ID: #{{ m.id }}</small
                                        >
                                    </div>
                                </div>
                            </td>
                            <td>{{ m.email }}</td>
                            <td>
                                <span
                                    v-if="m.is_admin"
                                    class="badge rounded-pill bg-warning text-dark"
                                >
                                    <i class="bi bi-shield-lock-fill me-1"></i>
                                    Admin
                                </span>
                                <span
                                    v-else
                                    class="badge rounded-pill bg-light text-secondary border"
                                >
                                    Member
                                </span>
                            </td>
                            <td>
                                <small
                                    class="text-truncate d-inline-block"
                                    style="max-width: 150px"
                                >
                                    {{ m.address }}
                                </small>
                            </td>
                            <td class="text-end pe-4">
                                <button
                                    @click="deleteMember(m.id)"
                                    class="btn btn-sm btn-outline-danger rounded-pill px-3"
                                >
                                    Delete
                                </button>
                            </td>
                        </tr>
                        <tr v-if="members.length === 0">
                            <td colspan="5" class="text-center py-5 text-muted">
                                No members found in the database.
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

// Fetch all members from the Rust backend
const fetchMembers = async () => {
    try {
        const res = await axios.get("http://localhost:7070/members");
        members.value = res.data;
    } catch (err) {
        console.error("Failed to fetch members:", err);
        alert("Could not load members. Check if the server is running.");
    }
};

// Handle member deletion
const deleteMember = async (id) => {
    if (
        confirm(
            "Are you sure you want to remove this member? This action cannot be undone.",
        )
    ) {
        try {
            await axios.delete(`http://localhost:7070/members/${id}`);
            // Refresh the list after deletion
            fetchMembers();
        } catch (err) {
            alert("Error deleting member.");
        }
    }
};

onMounted(fetchMembers);
</script>

<style scoped>
.avatar-circle {
    width: 40px;
    height: 40px;
    background-color: #e9ecef;
    color: #495057;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    font-size: 0.85rem;
    border: 2px solid #fff;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.table th {
    font-weight: 600;
    text-transform: uppercase;
    font-size: 0.75rem;
    letter-spacing: 0.5px;
    padding-top: 1rem;
    padding-bottom: 1rem;
}

.table-hover tbody tr:hover {
    background-color: rgba(13, 110, 253, 0.02);
}

.card {
    transition: transform 0.2s;
}
</style>
