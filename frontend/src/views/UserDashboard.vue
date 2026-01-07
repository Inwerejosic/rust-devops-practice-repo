<template>
    <div class="row">
        <div class="col-md-4">
            <div class="card border-0 shadow-sm bg-primary text-white mb-4">
                <div class="card-body">
                    <h5>Welcome back,</h5>
                    <h2 class="fw-bold">{{ auth.user?.f_name }}</h2>
                    <hr />
                    <p class="mb-0 small">
                        Account Status:
                        <span class="badge bg-success">Active</span>
                    </p>
                </div>
            </div>
        </div>
        <div class="col-md-8">
            <div class="card border-0 shadow-sm">
                <div class="card-body">
                    <h5 class="fw-bold mb-3">My Contribution History</h5>
                    <div class="table-responsive">
                        <table class="table align-middle">
                            <thead class="table-light">
                                <tr>
                                    <th>Period</th>
                                    <th>Amount</th>
                                    <th>Status</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr v-for="c in contributions" :key="c.id">
                                    <td>{{ c.month_period }}</td>
                                    <td class="fw-bold text-success">
                                        ${{ c.amount_paid }}
                                    </td>
                                    <td>
                                        <span class="badge bg-info"
                                            >Confirmed</span
                                        >
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import axios from "axios";
import { useAuthStore } from "../stores/auth";

const auth = useAuthStore();
const contributions = ref([]);

onMounted(async () => {
    try {
        const res = await axios.get(
            `http://localhost:7070/contributions/${auth.user.member_id}`,
        );
        contributions.value = res.data;
    } catch (e) {
        console.error("Error fetching data");
    }
});
</script>
