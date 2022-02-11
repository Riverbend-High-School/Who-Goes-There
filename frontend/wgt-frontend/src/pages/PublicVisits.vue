// Write a vue page that shows the active visits as a table
<template>
    <div class="fullscreen">
        <div class="top">
            <div class="header">
                <h1>Active Students</h1>
            </div>
            <div class="table_container">
                <div class="table_div">
                    <div class="table_head">
                        <div class="table_column">
                            <h3>Student Name</h3>
                        </div>
                        <div class="table_column">
                            <h3>Sign In Time</h3>
                        </div>
                        <div class="table_column">
                            <h3>Sign Out Time</h3>
                        </div>
                        <div class="table_column">
                            <h3>Visit Duration</h3>
                        </div>
                    </div>
                    <div class="table_body">
                        <div
                            class="table_row"
                            v-for="visit in visits"
                            v-bind:key="visit.id"
                        >
                            <div class="table_column" v-if="visit.student_name">
                                <p>{{ visit.student_name }}</p>
                            </div>
                            <div class="table_column" v-else>
                                <p>FAILED TO GET STUDENT NAME</p>
                            </div>

                            <div class="table_column">
                                <p>{{ renderDate(visit.checked_in + "Z") }}</p>
                            </div>

                            <div class="table_column" v-if="visit.left_at">
                                <p>{{ renderDate(visit.left_at + "Z") }}</p>
                            </div>
                            <div class="table_column" v-else>
                                <p>Active Visit</p>
                            </div>

                            <div class="table_column" v-if="visit.left_at">
                                <p>
                                    {{
                                        calculateDuration(
                                            visit.checked_in + "Z",
                                            visit.left_at + "Z"
                                        )
                                    }}
                                </p>
                            </div>
                            <div class="table_column" v-else>
                                <p>
                                    {{ visit.duration }}
                                </p>
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
                            <h3>Sign In Time</h3>
                        </div>
                        <div class="table_column">
                            <h3>Sign Out Time</h3>
                        </div>
                        <div class="table_column">
                            <h3>Visit Duration</h3>
                        </div>
                    </div>
                    <div class="table_body">
                        <div
                            class="table_row"
                            v-for="visit in aides"
                            v-bind:key="visit.id"
                        >
                            <div class="table_column" v-if="visit.student_name">
                                <p>{{ visit.student_name }}</p>
                            </div>
                            <div class="table_column" v-else>
                                <p>FAILED TO GET STUDENT NAME</p>
                            </div>

                            <div class="table_column">
                                <p>{{ renderDate(visit.checked_in + "Z") }}</p>
                            </div>

                            <div class="table_column" v-if="visit.left_at">
                                <p>{{ visit.left_at }}/p></p>
                            </div>
                            <div class="table_column" v-else>
                                <p>Active Visit</p>
                            </div>

                            <div class="table_column" v-if="visit.left_at">
                                <p>
                                    {{
                                        calculateDuration(
                                            visit.checked_in + "Z",
                                            visit.left_at + "Z"
                                        )
                                    }}
                                </p>
                            </div>
                            <div class="table_column" v-else>
                                <p>
                                    {{ visit.duration }}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import { visits } from "../assets/example_visits";

const debugging = false;
const EXAMPLE_VISITS = visits;
const BASE_URL = process.env.VUE_APP_ROOT_API;

export default {
    name: "ActiveVisits",
    components: {},
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
                this.axios.get(`${BASE_URL}/${this.endpoint_path}`).then(
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
                                    (visit) => !visit.is_aide
                                );
                                this.aides = all.filter(
                                    (visit) => visit.is_aide
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
        calculateDurationFromNow(start) {
            let now = new Date();
            return this.calculateDuration(start, now.toLocaleString());
        },
        calculateDuration(start, end) {
            let start_date = new Date(start);
            let end_date = new Date(end);
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
        updateDuration() {
            this.visits.forEach((visit) => {
                if (!visit.left_at) {
                    visit.duration = this.calculateDurationFromNow(
                        visit.checked_in + "Z"
                    );
                }
            });
            this.aides.forEach((visit) => {
                if (!visit.left_at) {
                    visit.duration = this.calculateDurationFromNow(
                        visit.checked_in + "Z"
                    );
                }
            });
        },
    },
    mounted() {
        this.get_students();
        if (!debugging) {
            this.get_interval = setInterval(this.get_students, 30000);
        }
        this.now_counter_interval = setInterval(this.updateDuration, 1000);
    },
    unmount() {
        clearInterval(this.get_interval);
        clearInterval(this.now_counter_interval);
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
