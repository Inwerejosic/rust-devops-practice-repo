<template>
    <div class="container">
        <div class="row">
            <div class="col-md-6">
                <div class="card shadow-sm border-0 rounded-4">
                    <div class="card-body p-4">
                        <h4 class="fw-bold mb-3">Global Settings</h4>
                        <p class="text-muted small">
                            Update the standard monthly contribution fee for all
                            members.
                        </p>

                        <form @submit.prevent="updateFee">
                            <div class="mb-3">
                                <label class="form-label fw-bold"
                                    >Current Monthly Fee ($)</label
                                >
                                <div class="input-group input-group-lg">
                                    <span class="input-group-text bg-white"
                                        >$</span
                                    >
                                    <input
                                        v-model="fee"
                                        type="number"
                                        class="form-control"
                                        placeholder="0.00"
                                        step="0.01"
                                    />
                                </div>
                            </div>
                            <button
                                class="btn btn-primary w-100 fw-bold py-2"
                                :disabled="loading"
                            >
                                {{
                                    loading
                                        ? "Updating..."
                                        : "Save Configuration"
                                }}
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import axios from "axios";

const fee = ref(0);
const loading = ref(false);

const fetchFee = async () => {
    try {
        const res = await axios.get("http://localhost:7070/admin/fee");
        // Assuming backend returns { "key": "monthly_fee", "value": "50" }
        fee.value = parseFloat(res.data.value);
    } catch (e) {
        console.error("Could not fetch fee");
    }
};

const updateFee = async () => {
    loading.value = true;
    try {
        await axios.put("http://localhost:7070/admin/fee", {
            key: "monthly_fee",
            value: fee.value.toString(),
        });
        alert("Standard fee updated successfully!");
    } catch (e) {
        alert("Failed to update fee");
    } finally {
        loading.value = false;
    }
};

onMounted(fetchFee);
</script>
