<template>
    <div>
        <input type="file" @change="uploadFile" ref="file" />
        <input type="button" value="Submit CSV" @click="submitFile" />
        <p>{{ output }}</p>
    </div>
</template>

<script>
const urlParams = new URLSearchParams(window.location.search);

export default {
    name: "IntegratePage",
    data() {
        return {
            token: null,
            csv: null,
            output: "",
        };
    },
    components: {},
    mounted() {
        this.token = urlParams.get("token");
    },
    methods: {
        uploadFile() {
            this.csv = this.$refs.file.files[0];
        },
        submitFile() {
            let comp = this;
            this.axios
                .post("/api/student/integrate?token=" + this.token, this.csv)
                .then((response) => {
                    console.log(response);
                    let status = response.data.status;
                    if (status != 200) {
                        comp.output = `There was an error while sending the response. Please check the console for more info. (Error code ${response.data.status} => ${response.data.message})`;
                    } else {
                        comp.output = "Successfully uploaded";
                    }
                })
                .catch((error) => {
                    console.error(error);
                    comp.output = `There was an error while sending the response. Please check the console for more info. (Error code ${error.response.status})`;
                });
        },
    },
};
</script>

<style></style>
