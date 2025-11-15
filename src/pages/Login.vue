<template>
  <v-container
    class="d-flex align-center justify-center"
    style="min-height: 100vh;"
  >
    <v-card
      class="pa-6"
      elevation="10"
      style="border-radius: 1rem; max-width: 400px; width: 90%; background-color: #1e1e1e;"
    >
      <v-card-title class="text-h5 white--text justify-center mb-4"
      style="color: white">
        Вхід
      </v-card-title>
      
      <v-card-text>
        <v-form @submit.prevent="login" class="d-flex flex-column gap-4">
          <v-text-field
            v-model="username"
            label="Логін"
            outlined
            rounded
            color="white"
            style="color: white"
            dense
            placeholder="Введіть логін"
            class="white--text"
          ></v-text-field>

          <v-text-field
            v-model="password"
            label="Пароль"
            type="password"
            style="color: white"
            outlined
            rounded
            color="white"
            dense
            placeholder="Введіть пароль"
            class="white--text"
          ></v-text-field>

          <v-btn
            type="submit"
            color="white"
            class="black--text font-weight-bold"
            elevation="4"
            large
          >
            Увійти
          </v-btn>
        </v-form>

        <v-alert
          v-if="error"
          type="error"
          class="mt-4"

          colored-border
        >
          {{ error }}
        </v-alert>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const username = ref('')
const password = ref('')
const error = ref('')

const login = async () => {

  let answer = await invoke<boolean>("user_list",{
    user: username.value,
    password: password.value
  });

  if (answer == true) {
    error.value = ''
    router.push('/dashboard')
  } else {
    error.value = 'Невірний логін або пароль'
  }
}
</script>

<style scoped>
.v-text-field input {
  color: white !important;
}

.v-text-field .v-label {
  color: rgba(255, 255, 255, 0.7) !important;
}

.v-btn {
  transition: transform 0.2s, box-shadow 0.2s;
}

.v-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
}
</style>
