<template>
    <div class="fullscreen">
        <div class="center-items">
            <div class="color-box left-rotate"></div>
            <div class="color-box right-rotate"></div>
            <div class="input-box">
                <h1 v-html="title"></h1>
                <p v-html="message"></p>
                <input
                    @click="returnDashboard"
                    type="button"
                    value="Return to Dashboard"
                />
            </div>
        </div>
    </div>
</template>

<script>
export default {
    name: "errorTemplate",
    props: ["title", "message"],
    methods: {
        returnDashboard() {
            this.$router.push("/");
        },
    },
};
</script>

<style scoped>
span {
    text-decoration-color: #da0000 !important;
    text-decoration: underline;
    text-decoration-thickness: auto;
}

.fullscreen {
    display: flex;
    position: relative;
    background-color: rgb(24 24 27);
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    min-height: 100vh;
    font-family: "Ubuntu", sans-serif;
}

.center-items {
    position: relative;
    width: 100%;
    max-width: 24rem;
}

.color-box {
    background-color: rgb(153 27 27);
    position: absolute;
    width: 100%;
    height: 100%;
    border-radius: 1.5rem;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1),
        0 4px 6px -2px rgba(0, 0, 0, 0.05);
}

.left-rotate {
    transform: rotate(-6deg);
}

.right-rotate {
    transform: rotate(6deg);
}

.input-box {
    position: relative;
    padding-left: 1.5rem;
    padding-right: 1.5rem;
    background-color: #e5e7eb;
    border-radius: 1.5rem;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1),
        0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

h1 {
    margin: 0px 0px 10px 0px;
    color: #374151;
    font-size: 2.75rem;
    line-height: 2.5rem;
    font-weight: 700;
    text-align: center;
}

p {
    color: #374151;
    font-size: 1.25rem;
    line-height: 1.5rem;
    text-align: center;
    margin: 0px 0px 20px 0px;
}

input {
    background-color: rgb(71 85 105);
    transition-property: background-color, border-color, color, fill, stroke,
        opacity, box-shadow, transform;
    transition-duration: 500ms;
    transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    color: #ffffff;
    width: 100%;
    border-radius: 0.75rem;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1),
        0 10px 10px -5px rgba(0, 0, 0, 0.04);
    font-size: 1.25rem;
}

input:hover {
    cursor: pointer;
    background-color: rgb(100 116 139);
}
</style>
