<template>
    <div class="fullscreen">
        <div class="forgot-one">
            <div class="color-box left-rotate"></div>
            <div class="color-box right-rotate"></div>
            <div class="input-box">
                <h1 class="title">Dashboard</h1>
                <div class="links">
                    <a class="link" :href="'/visits.html?token=' + token"
                        >Active Visits</a
                    >
                    <a class="link" :href="'/checkin.html?token=' + token"
                        >Checkin</a
                    >
                    <a class="link" :href="'/checkout.html?token=' + token"
                        >Checkout</a
                    >
                    <a class="link" :href="'/integrate.html?token=' + token"
                        >Integrate CSV</a
                    >
                </div>
            </div>
        </div>
    </div>
</template>

<script>
const urlParams = new URLSearchParams(window.location.search);

export default {
    name: "Home",
    data() {
        return {
            token: null,
        };
    },
    components: {},
    mounted() {
        this.token = urlParams.get("token");
    },
};
</script>

<style scope>
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

.link {
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
.link:hover {
    cursor: pointer;
    background-color: #1a6ef5;
}

.links {
    display: flex;
    justify-content: space-between;
    flex-direction: column;
    text-align: center;
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
