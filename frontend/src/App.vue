<template>
    <nav class="navbar navbar-expand-lg navbar-dark bg-dark sticky-top shadow">
        <div class="container">
            <router-link class="navbar-brand fw-bold" to="/"
                >GK PORTAL</router-link
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

                    <li v-if="auth.isAdmin" class="nav-item dropdown">
                        <a
                            class="nav-link dropdown-toggle text-warning fw-bold"
                            href="#"
                            role="button"
                            data-bs-toggle="dropdown"
                            aria-expanded="false"
                        >
                            Admin Tools
                        </a>
                        <ul class="dropdown-menu dropdown-menu-dark shadow">
                            <li>
                                <router-link class="dropdown-item" to="/admin">
                                    <i class="bi bi-people me-2"></i>Member
                                    Directory
                                </router-link>
                            </li>
                            <li>
                                <router-link
                                    class="dropdown-item"
                                    to="/admin/contributions"
                                >
                                    <i class="bi bi-cash-stack me-2"></i>Record
                                    Payments
                                </router-link>
                            </li>
                            <li><hr class="dropdown-divider" /></li>
                            <li>
                                <router-link
                                    class="dropdown-item"
                                    to="/admin/settings"
                                >
                                    <i class="bi bi-gear me-2"></i>Financial
                                    Settings
                                </router-link>
                            </li>
                        </ul>
                    </li>
                </ul>

                <div class="d-flex align-items-center">
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
                    <template v-else>
                        <span class="text-light me-3 small d-none d-lg-inline">
                            Hi, {{ auth.user?.f_name }}
                        </span>
                        <button
                            @click="handleLogout"
                            class="btn btn-danger btn-sm"
                        >
                            Logout
                        </button>
                    </template>
                </div>
            </div>
        </div>
    </nav>

    <main class="container py-4 mt-2">
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

<style>
/* Smooth transition for dropdowns */
.dropdown-menu {
    margin-top: 0.5rem;
    border: none;
}

.nav-link.active {
    color: #0d6efd !important;
    border-bottom: 2px solid #0d6efd;
}

/* Fix for mobile spacing */
@media (max-width: 991.98px) {
    .navbar-nav {
        padding-top: 1rem;
        padding-bottom: 1rem;
    }
    .d-flex {
        padding-top: 1rem;
        border-top: 1px solid rgba(255, 255, 255, 0.1);
    }
}
</style>
