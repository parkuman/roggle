import init, { solve } from "../../node_modules/roggle/roggle";

onmessage = async function (e) {
	// let module;
	// let solve;
	switch (e.data.msg) {
		case "solve": {
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

self.addEventListener("unhandledrejection", function (event) {
	// the event object has two special properties:
	// event.promise - the promise that generated the error
	// event.reason  - the unhandled error object
	throw event.reason;
});
