import * as wasm from "rust_enigma";
import Vue from "vue";

let enigma = wasm.Enigma.new();

let app = new Vue({
	el: "#app",
	data: {
		key: "",
		msg: "",
		encrypted_msg: "",
	},
	methods: {
		encrypt: function(event) {
			this.encrypted_msg = enigma.run(this.key, this.msg);
		},
	},
});

