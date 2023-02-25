import init, * as wasm from "./pkg/arith_ops.js"
await init();

let builder = new wasm.ExpressionBuilder();
let display = document.querySelector(".display");
wasm.enable_logging();

document.querySelectorAll("button").forEach(btn => {
    if (btn.classList.contains("special") && btn.value == "=" || btn.classList.contains("wide")) {
        return;
    }

    btn.addEventListener("click", () => {
        builder.extend(btn.value.charAt(0))
        display.textContent = builder.to_string();
    });
});

document.querySelectorAll("button.wide").forEach(btn => {
    btn.addEventListener("click", () => {
        if (btn.value === "c") {
            builder.clear();
        } else if (btn.value === "d") {
            builder.delete();
        }
        display.textContent = builder.to_string();
    });
});

document.querySelectorAll("button.special").forEach(btn => {
    if (btn.value == "=") {
        btn.addEventListener("click", () => {
            builder.extend(btn.value.charAt(0))
            let request = new wasm.ArithmeticRequest(builder.collect());
            let response = wasm.calculate(request);
            display.textContent = response.result();
        });
    }
});
