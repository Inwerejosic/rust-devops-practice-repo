<template>
    <div class="card shadow-sm border-0 rounded-4">
        <div class="card-body p-4">
            <h4 class="fw-bold mb-4 text-success">Record New Payment</h4>
            <form @submit.prevent="submitPayment">
                <div class="row g-3">
                    <div class="col-md-6">
                        <label class="form-label fw-bold">Select Member</label>
                        <select
                            v-model="payment.member_id"
                            class="form-select"
                            required
                        >
                            <option value="" disabled>
                                Choose a member...
                            </option>
                            <option
                                v-for="m in members"
                                :key="m.id"
                                :value="m.id"
                            >
                                {{ m.f_name }} {{ m.l_name }} (#{{ m.id }})
                            </option>
                        </select>
                    </div>

                    <div class="col-md-3">
                        <label class="form-label fw-bold">Month/Year</label>
                        <input
                            v-model="payment.month_period"
                            type="month"
                            class="form-control"
                            required
                        />
                    </div>

                    <div class="col-md-3">
                        <label class="form-label fw-bold"
                            >Amount Paid ($)</label
                        >
                        <input
                            v-model.number="payment.amount_paid"
                            type="number"
                            class="form-control"
                            step="0.01"
                            required
                        />
                    </div>
                </div>

                <div class="mt-4">
                    <button
                        class="btn btn-success px-5 fw-bold"
                        :disabled="submitting"
                    >
                        {{ submitting ? "Processing..." : "Confirm Payment" }}
                    </button>
                </div>
            </form>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted, reactive } from "vue";
import axios from "axios";

const members = ref([]);
const submitting = ref(false);
const payment = reactive({
    member_id: "",
    amount_paid: 0,
    month_period: "",
});

const fetchMembers = async () => {
    const res = await axios.get("http://localhost:7070/members");
    members.value = res.data;
};

const submitPayment = async () => {
    submitting.value = true;
    try {
        await axios.post("http://localhost:7070/contribute", payment);
        alert("Contribution recorded!");
        payment.member_id = "";
        payment.amount_paid = 0;
    } catch (e) {
        alert("Error recording payment");
    } finally {
        submitting.value = false;
    }
};

onMounted(fetchMembers);
</script>
