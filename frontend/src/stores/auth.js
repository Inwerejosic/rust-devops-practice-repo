import { defineStore } from "pinia";

export const useAuthStore = defineStore("auth", {
  state: () => ({
    token: localStorage.getItem("user_token") || null,
    user: JSON.parse(localStorage.getItem("user_info")) || null,
  }),
  getters: {
    isLoggedIn: (state) => !!state.token,
    isAdmin: (state) => state.user?.email === "inwe@gmail.com", // Securely checked on backend too
  },
  actions: {
    saveSession(token, userData) {
      this.token = token;
      this.user = userData;
      localStorage.setItem("user_token", token);
      localStorage.setItem("user_info", JSON.stringify(userData));
    },
    logout() {
      this.token = null;
      this.user = null;
      localStorage.clear();
    },
  },
});
