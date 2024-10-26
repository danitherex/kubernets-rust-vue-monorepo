<template>
    <div>
        <h1>API Response</h1>
        <p>{{ apiResponse }}</p>
        <button @click="fetchApiResponse">Fetch API Response</button>
    </div>
</template>

<script lang="ts">
const API_URL = "http://rust-backend-deployment:30100";

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