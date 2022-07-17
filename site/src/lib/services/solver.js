class Solver {
	_dispatchWorker(event) {
		const { msg } = event;
		this._status[msg] = ["loading"];

		this.worker.postMessage(event);

		return new Promise((res, rej) => {
			let interval = setInterval(() => {
				const status = this._status[msg];
				if (status[0] === "done") res(status[1]);
				if (status[0] === "error") rej(status[1]);
				if (status[0] !== "loading") {
					delete this._status[msg];
					clearInterval(interval);
				}
			}, 50);
		});
	}

	load() {
		this._status = {};
		this.worker = new Worker("/js/solver/roggle.worker.js", { type: "module" }); // initialize a module web worker to allow for es2015 imports

		this.worker.onmessage = (e) => {
			if (e.data.error) {
				return (this._status[e.data.msg] = ["error", e.data.error]);
			}

			return (this._status[e.data.msg] = ["done", e]);
		};
		this.worker.onerror = (e) => (this._status[e.message] = ["error", e.message]);
	}

	solve(board) {
		if (!this.worker) this.load();
		return this._dispatchWorker({ msg: "solve", board });
	}
}

export default new Solver();
