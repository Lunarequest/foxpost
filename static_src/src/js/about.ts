function bday() {
	const bday_element = document.getElementById("age");
	console.log(bday_element);
	const EPOCH = new Date(0);
	const EPOCH_YEAR = EPOCH.getFullYear();

	if (bday_element) {
		const bday = new Date("2003/04/19");
		const currentdate = new Date();
		const diff = new Date(currentdate.valueOf() - bday.valueOf());
		const age = Math.abs(diff.getFullYear() - EPOCH_YEAR);

		bday_element.innerHTML = age.toString();
	}
}

document.addEventListener("DOMContentLoaded", () => {
	console.log("bday script loaded");
	bday();
});
