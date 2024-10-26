<template>
    <div>
        <h1>API Response</h1>
        <p>{{ apiResponse }}</p>
        <button @click="fetchApiResponse">Fetch API Response</button>
    </div>
</template>

<script lang="ts">
const API_URL = window.env.VUE_APP_API_URL || "http://207.154.234.190:30100"; // Fallback for local dev
export default {
    data() {
        return {
            apiResponse: ''
        };
    },
    methods: {
        fetchApiResponse() {
            fetch(`${API_URL}/endpoint`)
                .then(response => response.json())
                .then(data => {
                    this.apiResponse = data.message;
                })
                .catch(error => {
                    console.error('Error fetching the API:', error);
                    this.apiResponse = 'Error fetching the API';
                });
        }
    }
};
</script>

<style scoped>
h1 {
    color: #ffffff;
}
div {
    background-color: #333333;
    padding: 20px;
    border-radius: 5px;
}
</style>