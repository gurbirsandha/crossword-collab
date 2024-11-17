<template>
  <main>
    <div class="card flex justify-center">
    <InputText type="text" v-model="values._00" v-on:update:model-value="sendChanges"/>
    <InputText type="text" v-model="values._01" v-on:update:model-value="sendChanges"/>
    <Button @click="click">fffsdf</Button>
    </div>  
  </main>
</template>


<script setup lang="ts">
import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import {onMounted,ref} from 'vue';

const values = ref({
    _00: '',
    _01: ''
});

let socket: WebSocket;

onMounted(() => {
    // socket = new WebSocket('ws://127.0.0.1:3030/echo');
    // console.log(socket);
    // // 
    // socket.onmessage = (event) => {
    //     values.value = JSON.parse(event.data)
    //     console.log("WebSocket message received:", event);
    // };
    // socket.onopen = (event) => {
    //     console.log("WOW OPEN", event)
    //     // socket.send("wow grape1");
    //     // socket.send("wow grape2");
    //     // socket.send("wow grape3");
    // }

})

function sendChanges(val: string | undefined) {
    if (val) {
        console.log(val)
        socket.send(JSON.stringify(values.value));
    }
}
function click() {
    socket = new WebSocket('ws://127.0.0.1:3030/play');
    socket.onopen = () => {
        socket.send('nice');
    }
}
</script>