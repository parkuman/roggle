class ImageProcessing {
	_dispatch(event) {
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
		this.worker = new Worker("/js/image.worker.js"); // load worker

		// Capture events and save [status, event] inside the _status object
		this.worker.onmessage = (e) => {
			console.log(e);
			return (this._status[e.data.msg] = ["done", e]);
		};
		this.worker.onerror = (e) => {
			console.error(e);
			return (this._status.load = ["error", e]);
		};
		return this._dispatch({ msg: "load" });
	}

	imageProcessing(data, debug=false) {
		if (!this.worker) this.load();
		return this._dispatch({ msg: "imageProcessing", data, debug });
	}
}

export default new ImageProcessing();
