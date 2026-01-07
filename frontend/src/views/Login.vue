<template>
    <div class="row justify-content-center pt-5">
        <div class="col-md-4">
            <div class="card shadow-lg border-0">
                <div class="card-body p-4">
                    <h3 class="text-center mb-4 fw-bold">Login</h3>
                    <form @submit.prevent="onLogin">
                        <div class="mb-3">
                            <label class="form-label">Email</label>
                            <input
                                v-model="email"
                                type="email"
                                class="form-control"
                                placeholder="name@email.com"
                                required
                            />
                        </div>
                        <div class="mb-3">
                            <label class="form-label">Password</label>
                            <input
                                v-model="password"
                                type="password"
                                class="form-control"
                                placeholder="••••••••"
                                required
                            />
                        </div>
                        <button class="btn btn-primary w-100 py-2">
                            Sign In
                        </button>
                    </form>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref } from "vue";
import axios from "axios";
import { useAuthStore } from "../stores/auth";
import { useRouter } from "vue-router";

const email = ref("");
const password = ref("");
const auth = useAuthStore();
const router = useRouter();

const onLogin = async () => {
    try {
        const res = await axios.post("http://localhost:7070/login", {
            email: email.value,
            password: password.value,
        });
        auth.saveSession(res.data.token, res.data);
        router.push(auth.isAdmin ? "/admin" : "/dashboard");
    } catch (err) {
        alert("Invalid Credentials");
    }
};
</script>
