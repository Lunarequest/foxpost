function bday() {
	const bday_element = document.getElementById("age");
	console.log(bday_element);
	const EPOCH = new Date(0);
	const EPOCH_YEAR = EPOCH.getFullYear();

	if (bday_element) {
		const diff = new Date(
			new Date().valueOf() - new Date("2003/04/19").valueOf(),
		);
		const age = Math.abs(diff.getFullYear() - EPOCH_YEAR);

		bday_element.innerHTML = age.toString();
	}
}

document.addEventListener("DOMContentLoaded", () => {
	console.log("bday script loaded");
	bday();
});
