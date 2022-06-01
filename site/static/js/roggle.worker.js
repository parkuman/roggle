import init, { solve } from "../../node_modules/roggle/roggle";

onmessage = async function (e) {
	// let module;
	// let solve;
	switch (e.data.msg) {
		case "solve": {
			await init();

			try {
				const solutions = await solve(e.data.board);
				postMessage({ msg: e.data.msg, solutions });
			} catch (e) {
				const message = `Problem while solving board: ${e}`;
				throw new Error(message);
			}

			break;
		}
		default:
			break;
	}
};
