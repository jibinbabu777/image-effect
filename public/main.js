import init, { grayscale } from './pkg/image_effect.js';

async function run() {
    // Initialize the WASM module
    await init();
    console.log("WASM module initialized");

    const input = document.getElementById('upload');
    const fileReader = new FileReader();

    fileReader.onloadend = async () => {
        let base64 = fileReader.result.replace(
            /^data:image\/(jpg|png|jpeg);base64,/, ''
        );

        try {
            // Call the grayscale function
            let imgDataUrl = await grayscale(base64);
            console.log("Generated grayscale image:", imgDataUrl);
            document.getElementById('new-img').setAttribute('src', imgDataUrl);
        } catch (e) {
            console.error("Error processing image:", e);
        }
    };

    input.addEventListener('change', () => {
        fileReader.readAsDataURL(input.files[0]);
    });
}

run();