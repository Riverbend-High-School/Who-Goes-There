// Write a vue page that shows the active visits as a table
<template>
    <div class="fullscreen" v-if="authenticated">
        <div class="top">
            <div class="header">
                <h1>Active Students</h1>
                <input
                    @click="checkoutAll"
                    type="button"
                    value="Sign Out All Students"
                />
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
                        <div
                            class="table_row"
                            v-for="visit in visits"
                            v-bind:key="visit.id"
                        >
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
                                <p>
                                    {{
                                        calculateDuration(
                                            visit.checked_in + "Z"
                                        )
                                    }}
                                </p>
                            </div>
                            <div class="actions_column">
                                <input
                                    type="button"
                                    title="Force Checkout"
                                    value="X"
                                    @click="
                                        forceCheckout(visit.student.seven_id)
                                    "
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="header">
                <h1>Active Aides</h1>
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
                        <div
                            class="table_row"
                            v-for="visit in aides"
                            v-bind:key="visit.id"
                        >
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
                                <p>
                                    {{
                                        calculateDuration(
                                            visit.checked_in + "Z"
                                        )
                                    }}
                                </p>
                            </div>
                            <div class="actions_column">
                                <input
                                    type="button"
                                    title="Force Checkout"
                                    value="X"
                                    @click="
                                        forceCheckout(visit.student.seven_id)
                                    "
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
    <error-template
        v-else-if="authenticated != null"
        title="Access Denied!"
        message="It appears you have reached an endpoint you don't have access to. If you believe this to be an error please check to make sure you have included your access token in the url with the following format: <span>URL FORMAT</span>. If you would like to request access please click the button below."
    ></error-template>
</template>

<script>
import { visits } from "../assets/example_visits";

import errorTemplate from "../components/errorTemplate.vue";

const debugging = false;
const EXAMPLE_VISITS = visits;
const BASE_URL = process.env.VUE_APP_ROOT_API;

export default {
    name: "ActiveVisits",
    components: {
        "error-template": errorTemplate,
    },
    props: ["endpoint"],
    data() {
        return {
            visits: [],
            aides: [],
            loading: false,
            error: "",
            success: "",
            endpoint_path: this.endpoint,
            get_interval: null,
            authenticated: null,
            BASE_URL: BASE_URL,
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
                                    var all = response.data.data;
                                    all.sort((a, b) => {
                                        return a.checked_in < b.checked_in;
                                    });
                                    this.visits = all.filter(
                                        (visit) => !visit.student.is_aide
                                    );
                                    this.aides = all.filter(
                                        (visit) => visit.student.is_aide
                                    );
                                } else {
                                    console.error(
                                        `Status code ${response.data.status}`
                                    );
                                    this.error = response.data.message;
                                }
                            } catch (e) {
                                this.error =
                                    "Failed to parse JSON response from server";
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
                                `Failed to get active visits: ${JSON.stringify(
                                    response.data
                                )}`
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
                this.axios
                    .get(`${BASE_URL}me?token=${this.$route.query.token}`)
                    .then(
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
        forceCheckout(id) {
            this.axios.post(
                `${BASE_URL}checkout?token=${this.$route.query.token}`,
                {
                    student_response: id,
                }
            );
        },
        checkoutAll() {
            let working = this.visits;
            working.forEach((visit) => {
                this.axios.post(
                    `${BASE_URL}checkout?token=${this.$route.query.token}`,
                    {
                        student_response: visit.student.seven_id,
                    }
                );
            });
        },
    },
    mounted() {
        this.checkMe();
        this.get_students();
        if (!debugging) {
            this.get_interval = setInterval(this.get_students, 1000);
        }
    },
    unmount() {
        clearInterval(this.get_interval);
    },
};
</script>

<style scoped>
.header {
    text-align: center;
}

.header > input {
    position: absolute;
    top: 10px;
    right: 10px;
    width: 10rem;
    background-color: #b91c1c;
    color: #fee2e2;
    font-weight: bold;
    cursor: pointer;
    margin-top: 0.25rem;
    text-align: center;
    border-radius: 0.75rem;
    border-style: none;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1),
        0 4px 6px -2px rgba(0, 0, 0, 0.05);
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
    margin-top: 0px;
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

.actions_column > input {
    margin: 0.5rem;
    background-color: #b91c1c;
    transition-property: background-color, border-color, color, fill, stroke;
    transition-duration: 150ms;
    color: #fee2e2;
    height: 2.5rem;
    border-radius: 0.5rem;
    cursor: pointer;
    transform: translateY(9%);
    width: 2.5rem;
}

.actions_column {
    display: flex;
    justify-content: center;
    align-content: center;
    width: 18vw;
    text-align: center;
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

<style>
body {
    background-color: #111827;
}
</style>
