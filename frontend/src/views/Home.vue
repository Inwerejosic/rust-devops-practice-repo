<template>
    <div class="container text-center py-5">
        <div class="row py-lg-5">
            <div class="col-lg-6 col-md-8 mx-auto">
                <h1 class="fw-light display-4">Community Portal</h1>
                <p class="lead text-muted">
                    A secure and transparent platform for managing community
                    contributions and memberships.
                </p>
                <div class="mt-4">
                    <router-link
                        v-if="!auth.isLoggedIn"
                        to="/register"
                        class="btn btn-primary btn-lg px-4 me-md-2"
                        >Get Started</router-link
                    >
                    <router-link
                        v-else
                        to="/dashboard"
                        class="btn btn-primary btn-lg px-4 me-md-2"
                        >Go to Dashboard</router-link
                    >
                    <router-link
                        to="/login"
                        v-if="!auth.isLoggedIn"
                        class="btn btn-outline-secondary btn-lg px-4"
                        >Member Login</router-link
                    >
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { useAuthStore } from "../stores/auth";
const auth = useAuthStore();
</script>
