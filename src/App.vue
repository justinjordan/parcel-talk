<template>
  <div>
    <h1>{{ num }} is {{ isPrime(num) ? "PRIME" : "NOT PRIME" }}</h1>
    <input type="number" v-model="num" />
  </div>
</template>

<script>
import { isPrime } from "../lib/prime.rs";

export default {
  data() {
    return {
      num: 1
    };
  },
  methods: {
    isPrime
  }
};
</script>
