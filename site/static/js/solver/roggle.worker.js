/**
 * This file is a Web Worker Responsible for running the Roggle
 * WASM module off of the main thread of the web application.
 * This ensures the user doesn't have their site freeze when it's solving!
 */
import init, { solve } from "./module/roggle.js";

onmessage = async function (e) {
	switch (e.data.msg) {
		case "solve": {
			let wasmSupported = !(typeof WebAssembly === "undefined");
			if (!wasmSupported) {
				postMessage({
					msg: e.data.msg,
					error: "Sorry! Your browser does not support the features needed to run Roggle"
				});
				break;
			}

			await init();
			try {
				const solutions = solve(e.data.board);
				postMessage({ msg: e.data.msg, solutions });
			} catch (err) {
				const error = `Problem while solving board: ${err.message}`;
				postMessage({ msg: e.data.msg, error });
			}
			break;
		}
		default:
			break;
	}
};
