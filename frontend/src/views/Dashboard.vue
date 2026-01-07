<template>
    <div class="dashboard-container">
        <div class="welcome-banner card">
            <h1>Community Portal</h1>
            <p>Manage memberships and track contributions effectively.</p>
        </div>

        <div class="stats-grid">
            <div class="stat-card">
                <h3>Members</h3>
                <p class="stat-number">{{ stats.members }}</p>
            </div>
            <div class="stat-card">
                <h3>Collections</h3>
                <p class="stat-number text-success">
                    ${{ stats.total.toFixed(2) }}
                </p>
            </div>
        </div>

        <div v-if="!isLoggedIn" class="cta-section">
            <router-link to="/register" class="btn btn-primary"
                >Become a Member</router-link
            >
            <router-link to="/login" class="btn btn-alt"
                >Member Login</router-link
            >
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import axios from "axios";

const stats = ref({ members: 0, total: 0 });
const isLoggedIn = !!localStorage.getItem("user_token");

onMounted(async () => {
    try {
        const mRes = await axios.get("http://localhost:7070/members");
        stats.value.members = mRes.data.length;
        // Calculation logic for total could go here
    } catch (e) {
        console.error("Could not fetch public stats");
    }
});
</script>

<style scoped>
.stats-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
    margin: 20px 0;
}
.stat-card {
    background: white;
    padding: 30px;
    border-radius: 12px;
    text-align: center;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
}
.stat-number {
    font-size: 3rem;
    font-weight: bold;
    margin: 10px 0;
}
.welcome-banner {
    background: #1e293b;
    color: white;
    padding: 40px;
    text-align: center;
}
.cta-section {
    display: flex;
    gap: 15px;
    justify-content: center;
    margin-top: 30px;
}
</style>
