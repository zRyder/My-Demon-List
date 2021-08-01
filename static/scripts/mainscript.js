
function toggle_hamburger()
{
	const hamburgerMenu = document.getElementsByClassName("hamburger_menu")[0];
	const hamburgerButton = document.querySelector(".hamburger_container");

	if(hamburgerMenu.style.display == "flex")
	{
		hamburgerButton.classList.remove('open');
		hamburgerMenu.style.display = "none";
	}
	else
	{
		hamburgerButton.classList.add('open');
		hamburgerMenu.style.display = "flex";
	}
}

function reset_hamburger()
{
	if(window.innerWidth >= 1111)
	{
		const hamburgerButton = document.querySelector(".hamburger_container");
		hamburgerButton.classList.remove('open');

		const hamburgerMenu = document.getElementsByClassName("hamburger_menu")[0];
		hamburgerMenu.style.display = "none";
	}
}