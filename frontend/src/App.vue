<template>
    <nav class="navbar navbar-expand-lg navbar-dark bg-dark sticky-top shadow">
        <div class="container">
            <router-link class="navbar-brand fw-bold" to="/"
                >PORTAL</router-link
            >
            <button
                class="navbar-toggler"
                type="button"
                data-bs-toggle="collapse"
                data-bs-target="#navbarNav"
            >
                <span class="navbar-toggler-icon"></span>
            </button>
            <div class="collapse navbar-collapse" id="navbarNav">
                <ul class="navbar-nav me-auto">
                    <li class="nav-item">
                        <router-link class="nav-link" to="/">Home</router-link>
                    </li>
                    <li v-if="auth.isLoggedIn" class="nav-item">
                        <router-link class="nav-link" to="/dashboard"
                            >My Account</router-link
                        >
                    </li>
                    <li v-if="auth.isAdmin" class="nav-item">
                        <router-link
                            class="nav-link text-warning fw-bold"
                            to="/admin"
                            >Admin Area</router-link
                        >
                    </li>
                </ul>
                <div class="d-flex">
                    <template v-if="!auth.isLoggedIn">
                        <router-link
                            to="/login"
                            class="btn btn-outline-light me-2"
                            >Login</router-link
                        >
                        <router-link to="/register" class="btn btn-primary"
                            >Join</router-link
                        >
                    </template>
                    <button v-else @click="handleLogout" class="btn btn-danger">
                        Logout
                    </button>
                </div>
            </div>
        </div>
    </nav>

    <main class="container py-4">
        <router-view />
    </main>
</template>

<script setup>
import { useAuthStore } from "./stores/auth";
import { useRouter } from "vue-router";

const auth = useAuthStore();
const router = useRouter();

const handleLogout = () => {
    auth.logout();
    router.push("/login");
};
</script>
