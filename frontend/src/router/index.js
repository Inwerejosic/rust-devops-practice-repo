import { createRouter, createWebHistory } from "vue-router";
import { useAuthStore } from "../stores/auth";
import Home from "../views/Home.vue";
import Login from "../views/Login.vue";
import Register from "../views/Register.vue";
import UserDashboard from "../views/UserDashboard.vue";
import AdminPanel from "../views/AdminPanel.vue";
import AdminSettings from "../views/AdminSettings.vue";
import ContributionManager from "../views/ContributionManager.vue";

const routes = [
  { path: "/", component: Home },
  { path: "/login", component: Login },
  { path: "/register", component: Register },
  {
    path: "/dashboard",
    component: UserDashboard,
    meta: { requiresAuth: true },
  },
  {
    path: "/admin",
    component: AdminPanel,
    meta: { requiresAuth: true, requiresAdmin: true },
  },
  {
    path: "/admin/settings",
    component: AdminSettings,
    meta: { requiresAdmin: true },
  },
  {
    path: "/admin/contributions",
    component: ContributionManager,
    meta: { requiresAdmin: true },
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach((to, from, next) => {
  const auth = useAuthStore();
  if (to.meta.requiresAuth && !auth.isLoggedIn) next("/login");
  // else if (to.meta.requiresAdmin && !auth.isAdmin) next("/dashboard");
  else next();
});

export default router;
