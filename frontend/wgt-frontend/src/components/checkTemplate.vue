<template>
    <div class="fullscreen" v-if="authenticated">
        <div class="forgot-one">
            <div class="color-box left-rotate"></div>
            <div class="color-box right-rotate"></div>
            <div class="input-box">
                <label class="title"> {{ title }} </label>
                <form @submit.prevent="submit" style="margin-top: 0.75rem">
                    <input
                        v-model="student_id"
                        type="text"
                        name="student_id"
                        placeholder="7-Digit Student ID Number"
                        autocomplete="off"
                    />
                    <button type="submit" class="submit">Submit</button>
                    <div class="loading">
                        <beat-loader v-if="loading"></beat-loader>
                        <p
                            class="status_text"
                            style="color: rgb(22 163 74)"
                            v-if="success"
                        >
                            {{ success }}
                        </p>
                        <p
                            class="status_text"
                            style="color: rgb(220 38 38)"
                            v-else-if="error"
                        >
                            {{ error }}
                        </p>
                    </div>
                </form>
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
import BeatLoader from "vue-spinner/src/BeatLoader.vue";
import Configuration from "@/assets/configuration";
import errorTemplate from "../components/errorTemplate.vue";

const BASE_URL = Configuration.value("rootAPI");
const urlParams = new URLSearchParams(location.search);

export default {
    name: "checkTemplate",
    props: ["title", "endpoint"],
    components: {
        BeatLoader,
        "error-template": errorTemplate,
    },
    data() {
        return {
            student_id: "",
            error: "",
            success: "",
            loading: false,
            endpoint_path: this.endpoint,
            authenticated: null,
        };
    },
    methods: {
        submit(e) {
            //   console.log(`Submitting ${this.student_id} with token "${urlParams.get("token")}"`);
            //   console.log(urlParams.get("token"));
            this.loading = true;
            this.error = "";
            this.success = "";
            this.axios
                .post(
                    `${BASE_URL}/${this.endpoint_path}?token=${urlParams.get(
                        "token"
                    )}`,
                    {
                        student_response: this.student_id,
                    }
                )
                .then(
                    (response) => {
                        this.loading = false;
                        try {
                            if (response.data.status == 204) {
                                this.success = response.data.message;
                                this.student_id = "";
                            } else {
                                console.error(
                                    `Status code ${response.data.status}`
                                );
                                this.error = response.data.message;
                            }
                            setTimeout(() => {
                                this.success = "";
                            }, 2000);
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
                        try {
                            this.error = response.data.message;
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
                    }
                );
            setTimeout(() => {
                this.success = "";
            }, 2000);
            setTimeout(() => {
                this.error = "";
            }, 5000);
            e.preventDefault();
        },
        checkMe(e) {
            if (urlParams.get("token")) {
                this.axios
                    .get(`${BASE_URL}/me?token=${urlParams.get("token")}`)
                    .then(
                        (response) => {
                            if (response.status == 200) {
                                console.log("Successfully authenticated token");
                                this.authenticated = true;
                            } else {
                                clearInterval(this.get_interval);
                                console.error(
                                    `Failed to authorize current token ${urlParams.get(
                                        "token"
                                    )} with error ${response}`
                                );
                                this.authenticated = false;
                            }
                        },
                        (response) => {
                            clearInterval(this.get_interval);
                            console.error(
                                `Failed to authorize current token ${urlParams.get(
                                    "token"
                                )} with error ${response}`
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
        console.log("Mounted");
        this.checkMe();
    },
    unmount() {},
};
</script>

<style scoped>
.fullscreen {
    display: flex;
    position: relative;
    background-color: #111827;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    min-height: 100vh;
    font-family: sans;
}

.forgot-one {
    position: relative;
    width: 100%;
    max-width: 24rem;
}

.color-box {
    position: absolute;
    width: 100%;
    height: 100%;
    border-radius: 1.5rem;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1),
        0 4px 6px -2px rgba(0, 0, 0, 0.05);
}

.left-rotate {
    background-color: #1d4ed8;
    transform: rotate(-6deg);
}

.right-rotate {
    background-color: #047857;
    transform: rotate(6deg);
}

.input-box {
    position: relative;
    padding-top: 1rem;
    padding-bottom: 1rem;
    padding-left: 1.5rem;
    padding-right: 1.5rem;
    background-color: #e5e7eb;
    border-radius: 1.5rem;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1),
        0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.title {
    display: block;
    margin-top: 0.75rem;
    color: #374151;
    font-size: 2.25rem;
    line-height: 2.5rem;
    font-weight: 700;
    text-align: center;
}

input {
    display: block;
    margin-top: 0.25rem;
    background-color: #f3f4f6;
    text-align: center;
    width: 100%;
    height: 2.75rem;
    border-radius: 0.75rem;
    border-style: none;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1),
        0 4px 6px -2px rgba(0, 0, 0, 0.05);
}

button {
    margin-top: 0.75rem;
    padding-top: 0.75rem;
    padding-bottom: 0.75rem;
    background-color: #3b82f6;
    transition-property: background-color, border-color, color, fill, stroke,
        opacity, box-shadow, transform;
    transition-duration: 500ms;
    transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    color: #ffffff;
    width: 100%;
    border-radius: 0.75rem;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1),
        0 10px 10px -5px rgba(0, 0, 0, 0.04);
}
button:hover {
    cursor: pointer;
    background-color: #1a6ef5;
}

.loading {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 20px;
    margin-top: 0.75rem;
    height: 1rem;
}

.status_text {
    font-weight: 700;
    text-align: center;
}
</style>
