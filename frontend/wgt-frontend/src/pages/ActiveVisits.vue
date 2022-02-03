// Write a vue page that shows the active visits as a table
<template>
  <div class="top" v-if="authenticated">
    <div class="header">
      <h1>Active Visits</h1>
    </div>
    <div class="table_container">
      <div class="table_div">
        <div class="table_head">
          <div class="table_column">
            <h3>Student Name</h3>
          </div>
          <div class="table_column">
            <h3>Student ID</h3>
          </div>
          <div class="table_column">
            <h3>Checked In</h3>
          </div>
          <div class="table_column">
            <h3>Duration</h3>
          </div>
          <div class="table_column">
            <h3>Actions</h3>
          </div>
        </div>
        <div class="table_body">
          <div class="table_row" v-for="visit in visits" v-bind:key="visit.id">
            <div class="table_column" v-if="visit.student">
              <p>{{ visit.student.student_name }}</p>
            </div>
            <div class="table_column" v-else>
                <p>FAILED TO GET STUDENT NAME</p>
            </div>
            <div class="table_column" v-if="visit.student">
              <p>{{ visit.student.seven_id }}</p>
            </div>
            <div class="table_column" v-else>
                <p>FAILED TO GET STUDENT ID</p>
            </div>
            <div class="table_column">
              <p>{{ renderDate(visit.checked_in + "Z") }}</p>
            </div>
            <div class="table_column">
              <p>{{ calculateDuration(visit.checked_in + "Z") }}</p>
            </div>
            <div class="table_column">
                <input type="button" value="Force Checkout" @click="forceCheckOut(visit.student.seven_id)">
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <div class="error" v-else>
    <p>You're not authorized to view this page! Please provide a valid API token in your url!</p>
    <p>Example:</p>
    <a>{{ BASE_URL }}?token=TOKEN_HERE</a>
  </div>
</template>

<script>
import { visits } from "../assets/example_visits";

const debugging = false;
const EXAMPLE_VISITS = visits;
const BASE_URL = process.env.VUE_APP_ROOT_API;

export default {
  name: "ActiveVisits",
  props: ["endpoint"],
  data() {
    return {
      visits: [],
      loading: false,
      error: "",
      success: "",
      endpoint_path: this.endpoint,
      get_interval: null,
      authenticated: false,
      BASE_URL: BASE_URL
    };
  },
  methods: {
    get_students(e) {
      if (debugging) {
        this.visits = EXAMPLE_VISITS;
      } else {
        this.loading = true;
        this.error = "";
        this.success = "";
        this.axios
          .get(
            `${BASE_URL}${this.endpoint_path}?token=${this.$route.query.token}`
          )
          .then(
            (response) => {
              this.loading = false;
              try {
                if (response.data.status == 200) {
                  this.success = response.data.message;
                  this.visits = response.data.data;
                  this.visits.sort((a, b) => {
                    return a.checked_in < b.checked_in;
                  });
                } else {
                  console.error(`Status code ${response.data.status}`);
                  this.error = response.data.message;
                }
              } catch (e) {
                this.error = "Failed to parse JSON response from server";
                console.error(
                  `Failed to parse JSON response from server: ${JSON.stringify(
                    response.data
                  )}`
                );
                console.error(e);
              }
            },
            (response) => {
              this.loading = false;
              this.error = "Failed to get active visits";
              console.error(
                `Failed to get active visits: ${JSON.stringify(response.data)}`
              );
            }
          );
      }
      if (e) {
        e.preventDefault();
      }
    },
    calculateDuration(start) {
      let start_date = new Date(start);
      let end_date = new Date();
      let diff = end_date.getTime() - start_date.getTime();
      let duration = Math.floor(diff / 1000);
      let hours = Math.floor(duration / 3600);
      let minutes = Math.floor((duration % 3600) / 60);
      let seconds = duration % 60;
      return `${hours}h ${minutes}m ${seconds}s`;
    },
    renderDate(time) {
      let date = new Date(time);
      return date.toLocaleString();
    },
    checkMe(e) {
      if (this.$route.query.token) {
        this.axios.get(`${BASE_URL}me?token=${this.$route.query.token}`).then(
          (response) => {
            if (response.status == 200) {
              console.log("Successfully authenticated token");
              this.authenticated = true;
            } else {
              clearInterval(this.get_interval);
              console.error(
                `Failed to authorize current token ${this.$route.query.token} with error ${response}`
              );
              this.authenticated = false;
            }
          },
          (response) => {
            clearInterval(this.get_interval);
            console.error(
              `Failed to authorize current token ${this.$route.query.token} with error ${response}`
            );
            this.authenticated = false;
          }
        );
      } else {
        console.error("No token provided");
        clearInterval(this.get_interval);
        this.authenticated = false;
      }
      if (e) {
        e.preventDefault();
      }
      return this.authenticated;
    },
  },
  mounted() {
    this.checkMe();
    this.get_students();
    if(!debugging) {
        this.get_interval = setInterval(this.get_students, 1000);
    }
  },
  unmount() {
    clearInterval(this.get_interval);
  },
};
</script>

<style>
body {
    background-color: #111827;
}

.header {
  text-align: center;
}

.table_container {
  display: flex;
  justify-content: center;
}

.table_div {
  height: 100%;
  overflow: auto;
  text-align: center;
}

.table_flex {
  display: flex;
  flex-direction: row;
  width: 100%;
  height: 100%;
  justify-content: space-between;
}

.table_head {
  display: flex;
  flex-direction: row;
  width: 100%;
}

h3,
h1 {
    color: white;
}

.table_body {
    width: 100%;
    overflow-block: auto;
}

.table_column {
  width: 18vw;
}

.table_column > p {
    color: #374151;
    font-size: 1.25rem;
}

.table_column > input {
    color: #374151;
    font-size: 1.25rem;
    border: none;
    border-radius: 5px;
    margin: 0px;
    padding: 0px;
}

.table_row {
  display: flex;
  flex-direction: row;
  height: 100%;
  border-radius: 10px;
  margin: 1px;
  background-color: #e5e7eb;
}

.error {
    color: white;
}
</style>