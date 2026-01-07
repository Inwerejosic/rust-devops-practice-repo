<template>
  <div class="view-container">
    <div class="card sidebar-layout">
      <div class="payment-form">
        <h3>Record Payment</h3>
        <form @submit.prevent="submitPayment">
          <label>Member</label>
          <select v-model="form.member_id" @change="fetchHistory" required>
            <option value="" disabled>Select member...</option>
            <option v-for="m in members" :key="m.id" :value="m.id">{{ m.f_name }} {{ m.l_name }}</option>
          </select>

          <label>Amount ($)</label>
          <input type="number" v-model.number="form.amount_paid" step="0.01" required />

          <label>Month</label>
          <input type="month" v-model="form.month_period" required />

          <button type="submit" class="btn-save">Submit Payment</button>
        </form>
      </div>

      <div class="history-table">
        <h3>Payment History</h3>
        <table>
          <thead>
            <tr>
              <th>Amount</th>
              <th>Month</th>
              <th>Date</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="c in contributions" :key="c.id">
              <td>${{ c.amount_paid }}</td>
              <td>{{ c.month_period }}</td>
              <td>{{ new Date(c.created_at).toLocaleDateString() }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import axios from 'axios';

const API_URL = 'http://localhost:7070';
const members = ref([]);
const contributions = ref([]);
const form = ref({ member_id: '', amount_paid: 0, month_period: '' });

const fetchMembers = async () => {
  const res = await axios.get(`${API_URL}/members`);
  members.value = res.data;
};

const fetchHistory = async () => {
  if (!form.value.member_id) return;
  const res = await axios.get(`${API_URL}/contributions/${form.value.member_id}`);
  contributions.value = res.data;
};

const submitPayment = async () => {
  await axios.post(`${API_URL}/contribute`, form.value);
  alert("Payment Success");
  fetchHistory();
};

onMounted(fetchMembers);
</script>

<style scoped>
.sidebar-layout { display: grid; grid-template-columns: 300px 1fr; gap: 30px; }
select, input { width: 100%; padding: 10px; margin-bottom: 15px; border-radius: 6px; border: 1px solid #ddd; }
.btn-save { background: #10b981; color: white; width: 100%; border: none; padding: 12px; border-radius: 6px; cursor: pointer; }
</style>
