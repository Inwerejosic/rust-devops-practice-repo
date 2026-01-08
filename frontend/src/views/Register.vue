<template>
    <div class="container py-5">
        <div class="row justify-content-center">
            <div class="col-md-8 col-lg-6">
                <div class="card shadow-lg border-0 rounded-4 p-4 p-md-5">
                    <div class="text-center mb-4">
                        <h2 class="fw-bold text-primary">Join the Community</h2>
                        <p class="text-muted">
                            Fill in your details to create a secure account
                        </p>
                    </div>

                    <form @submit.prevent="handleRegister">
                        <div
                            class="mb-4 p-3 bg-light rounded-3 d-flex justify-content-between align-items-center"
                        >
                            <div>
                                <label
                                    class="fw-bold mb-0 d-block"
                                    for="adminSwitch"
                                    >Administrative Access</label
                                >
                                <small class="text-muted"
                                    >Enable this to manage other members</small
                                >
                            </div>
                            <div class="form-check form-switch fs-4">
                                <input
                                    v-model="form.is_admin"
                                    class="form-check-input"
                                    type="checkbox"
                                    role="switch"
                                    id="adminSwitch"
                                />
                            </div>
                        </div>

                        <div class="row g-3">
                            <div class="col-sm-4">
                                <label class="form-label small fw-bold"
                                    >First Name</label
                                >
                                <input
                                    v-model="form.f_name"
                                    type="text"
                                    class="form-control shadow-sm"
                                    required
                                />
                            </div>
                            <div class="col-sm-4">
                                <label class="form-label small fw-bold"
                                    >Middle Name</label
                                >
                                <input
                                    v-model="form.m_name"
                                    type="text"
                                    class="form-control shadow-sm"
                                    placeholder="Optional"
                                />
                            </div>
                            <div class="col-sm-4">
                                <label class="form-label small fw-bold"
                                    >Last Name</label
                                >
                                <input
                                    v-model="form.l_name"
                                    type="text"
                                    class="form-control shadow-sm"
                                    required
                                />
                            </div>
                        </div>

                        <div class="mt-3">
                            <label class="form-label small fw-bold"
                                >Email Address</label
                            >
                            <input
                                v-model="form.email"
                                type="email"
                                class="form-control shadow-sm"
                                placeholder="email@example.com"
                                required
                            />
                        </div>

                        <div class="mt-3">
                            <label class="form-label small fw-bold"
                                >Password</label
                            >
                            <input
                                v-model="form.password"
                                type="password"
                                class="form-control shadow-sm"
                                placeholder="Minimum 8 characters"
                                required
                            />
                        </div>

                        <div class="row mt-3">
                            <div class="col-8">
                                <label class="form-label small fw-bold"
                                    >Home Address</label
                                >
                                <input
                                    v-model="form.address"
                                    type="text"
                                    class="form-control shadow-sm"
                                    required
                                />
                            </div>
                            <div class="col-4">
                                <label class="form-label small fw-bold"
                                    >Age</label
                                >
                                <input
                                    v-model.number="form.age"
                                    type="number"
                                    class="form-control shadow-sm"
                                    required
                                />
                            </div>
                        </div>

                        <button
                            class="btn btn-primary w-100 py-3 mt-5 fw-bold shadow-sm rounded-pill transition-all"
                            :disabled="loading"
                        >
                            <span
                                v-if="loading"
                                class="spinner-border spinner-border-sm me-2"
                            ></span>
                            {{
                                loading
                                    ? "Creating Account..."
                                    : "Create Account"
                            }}
                        </button>

                        <p class="text-center mt-3 small text-muted">
                            Already have an account?
                            <router-link
                                to="/login"
                                class="text-primary text-decoration-none fw-bold"
                                >Login here</router-link
                            >
                        </p>
                    </form>
                </div>
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
    is_admin: false,
});

const handleRegister = async () => {
    loading.value = true;
    try {
        // Note: URL should be updated if your backend is deployed elsewhere
        await axios.post("http://localhost:7070/register", form);
        alert("Success! Your account has been created. Please log in.");
        router.push("/login");
    } catch (e) {
        console.error(e);
        alert("Error: Registration failed. The email might already be in use.");
    } finally {
        loading.value = false;
    }
};
</script>

<style scoped>
.transition-all {
    transition: all 0.3s ease;
}
.transition-all:hover {
    transform: translateY(-2px);
    filter: brightness(1.1);
}
.form-control:focus {
    border-color: #0d6efd;
    box-shadow: 0 0 0 0.25rem rgba(13, 110, 253, 0.15);
}
</style>
