<template>
    <div class="row justify-content-center">
        <div class="col-md-8 col-lg-6">
            <div class="card shadow border-0 p-4">
                <h2 class="text-center mb-4 fw-bold text-primary">
                    Join the Community
                </h2>
                <form @submit.prevent="handleRegister">
                    <div class="row g-3">
                        <div class="col-sm-4">
                            <label class="form-label">First Name</label>
                            <input
                                v-model="form.f_name"
                                type="text"
                                class="form-control"
                                required
                            />
                        </div>
                        <div class="col-sm-4">
                            <label class="form-label">Middle Name</label>
                            <input
                                v-model="form.m_name"
                                type="text"
                                class="form-control"
                                placeholder="(Optional)"
                            />
                        </div>
                        <div class="col-sm-4">
                            <label class="form-label">Last Name</label>
                            <input
                                v-model="form.l_name"
                                type="text"
                                class="form-control"
                                required
                            />
                        </div>
                    </div>

                    <div class="mt-3">
                        <label class="form-label">Email Address</label>
                        <input
                            v-model="form.email"
                            type="email"
                            class="form-control"
                            required
                        />
                    </div>

                    <div class="mt-3">
                        <label class="form-label">Password</label>
                        <input
                            v-model="form.password"
                            type="password"
                            class="form-control"
                            required
                        />
                    </div>

                    <div class="row mt-3">
                        <div class="col-8">
                            <label class="form-label">Address</label>
                            <input
                                v-model="form.address"
                                type="text"
                                class="form-control"
                                required
                            />
                        </div>
                        <div class="col-4">
                            <label class="form-label">Age</label>
                            <input
                                v-model.number="form.age"
                                type="number"
                                class="form-control"
                                required
                            />
                        </div>
                    </div>

                    <button
                        class="btn btn-primary w-100 py-2 mt-4 fw-bold"
                        :disabled="loading"
                    >
                        {{ loading ? "Creating Account..." : "Register" }}
                    </button>
                </form>
            </div>
        </div>
    </div>
</template>

<script setup>
import { reactive, ref } from "vue";
import axios from "axios";
import { useRouter } from "vue-router";

const router = useRouter();
const loading = ref(false);
const form = reactive({
    f_name: "",
    m_name: "",
    l_name: "",
    email: "",
    password: "",
    address: "",
    age: null,
});

const handleRegister = async () => {
    loading.value = true;
    try {
        await axios.post("http://localhost:7070/register", form);
        alert("Success! You can now log in.");
        router.push("/login");
    } catch (e) {
        alert("Error: Registration failed. Ensure all fields are filled.");
    } finally {
        loading.value = false;
    }
};
</script>
