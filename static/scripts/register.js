const signup_form = document.getElementById("signup_form");
const submit_button = document.getElementById("signup_submit");

const user_name_textbox = signup_form.elements.namedItem("user_name");
const password_textbox = signup_form.elements.namedItem("password");
const email_textbox = signup_form.elements.namedItem("email");
const repeat_password_textbox = signup_form.elements.namedItem("repeat_password");

//Assume that the form is not valid
let user_name_flag = true;
let password_flag = true;
let email_flag = true;
let password_match_flag = true;

let account_created = false;

user_name_textbox.addEventListener("input", is_valid_user_name);
password_textbox.addEventListener("input", is_valid_password);
password_textbox.addEventListener("input", passwords_match);
email_textbox.addEventListener("input", is_valid_email);
repeat_password_textbox.addEventListener("input", passwords_match);


function toggle_submission(){
    if (user_name_flag || password_flag || email_flag || password_match_flag) {
        submit_button.disabled = true;
    }
    else{
        submit_button.disabled = false;
    }
}

function toggle_email_error() {
    let email_textbox = document.getElementById("email");
    let email_message = document.getElementById("email_error");

    if (!email_flag){
        email_message.style.display = "none"
        email_textbox.classList.remove("textbox_error");
    }
    else {
        email_message.style.display = "block"
        email_textbox.classList.add("textbox_error");
    }
}

function toggle_user_name_error() {
    let user_name_textbox = document.getElementById("user_name");
    let user_name_message = document.getElementById("user_name_error");

    if (!user_name_flag){
        user_name_message.style.display = "none"
        user_name_textbox.classList.remove("textbox_error");
    }
    else {
        user_name_message.style.display = "block"
        user_name_textbox.classList.add("textbox_error");
    }
}

function toggle_password_error(){
    let password_textbox = document.getElementById("password");
    let password_message = document.getElementById("password_error");

    if (!password_flag){
        password_message.style.display = "none"
        password_textbox.classList.remove("textbox_error");
    }
    else {
        password_message.style.display = "block"
        password_textbox.classList.add("textbox_error");
    }
}

function toggle_password_match_error() {
    let repeat_password_textbox = document.getElementById("repeat_password");
    let repeat_password_message = document.getElementById("repeat_password_error");
    let password_textbox = document.getElementById("password");
    let password_message = document.getElementById("password_error");

    if (!password_match_flag){
        repeat_password_message.style.display = "none"
        repeat_password_textbox.classList.remove("textbox_error");

        password_message.style.display = "none"
        password_textbox.classList.remove("textbox_error");
    }
    else {
        repeat_password_message.style.display = "block"
        repeat_password_textbox.classList.add("textbox_error");

        password_message.style.display = "block";
        password_textbox.classList.add("textbox_error");
    }
}

function is_valid_email(event){
    let regex = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
    email_flag = !(regex.test(event.target.value))
    toggle_submission();
    toggle_email_error();
}

function is_valid_user_name(event){
    let input = event.target.value;
    let flag = false;

    for (let i = 0; i < input.length; i++) {
        let code = input.charCodeAt(i);
        if (!(code > 47 && code < 58) && // numeric (0-9)
            !(code > 64 && code < 91) && // upper alpha (A-Z)
            !(code > 96 && code < 123)) { // lower alpha (a-z)
            flag = true;

            break;
        }
    }

    user_name_flag = flag;
    toggle_submission();
    toggle_user_name_error();
}

function is_valid_password(event){
    let password = event.target.value;
    if ((has_capital_letter(password) && has_number(password) && has_symbol(password)) && password.length >=8 ){
        password_flag = false;
    }
    else{
        let password_textbox = document.getElementById("password");
        let password_message = document.getElementById("password_error");
        password_message.style.display = "block"
        password_textbox.classList.add("textbox_error");
        password_flag = true;
    }

    toggle_submission();
    toggle_password_error();
}

function has_capital_letter(password){
    for(let i = 0; i < password.length; i++){
        let code = password.charCodeAt(i);
        if (code > 64 && code < 91) {
            return true;
        }
    }
    return false;
}

function has_number(password){
    let regex = /[0-9]/g
    for(let i = 0; i < password.length; i++){
        if (password.charAt(i).match(regex)){
            return true;
        }
    }
    return false
}

function has_symbol(password) {
    let format = /[ `!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?~]/;
    let flag = format.test(password)
    return flag
}

function passwords_match() {
    if (password_textbox.value === repeat_password_textbox.value) {
        password_match_flag = false;
    }
    else {
        password_match_flag = true;
    }
    toggle_submission();
    toggle_password_match_error();
}

function close_modal() {
    const modal = document.getElementById("signup_modal");

    modal.style.display = "none";
    document.body.style.overflow = "auto";

    if (account_created) {
        console.log("redirect");
        window.location.href = "../log_in.html"
    }
}

async function create_account() {
    submit_button.disabled = true;
    event.preventDefault();

    const data = new URLSearchParams(new FormData(event.target));
    const test = decodeURIComponent(data)
    console.log(test.toString());

    const modal = document.getElementById("signup_modal");
    const modal_heading = document.getElementById("signup_heading");
    const modal_text = document.getElementById("signup_content");

    await fetch("/api/users/create", {
        method: 'POST', // *GET, POST, PUT, DELETE, etc.
        mode: 'same-origin', // no-cors, *cors, same-origin
        credentials: 'same-origin', // include, *same-origin, omit
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: test
    }).then(response_data => {
        if (response_data.status == 201) {
            modal_heading.innerHTML = "Account Successfully Created!"
            modal_text.innerHTML = "Welcome to MyDemonList <i>USERNAME!</i> You can now login via the login page <a href=\"log_in.html\">here</a>. Be sure to check your e-mail account to verify your account, if you do not see an e-mail check your spam account or login and request a new verification e-mail."
            modal.style.display = "block";
            document.body.style.overflow = "hidden";

            account_created = true;
            console.log("True")
        }
        else {
            modal_heading.innerHTML = "Could not Create Account!"
            modal_text.innerHTML = "There was a problem creating your account, contact support for more information."
            modal.style.display = "block";
            document.body.style.overflow = "hidden";
        }
    }).catch(error => {
        modal_heading.innerHTML = "Could not Create Account!"
        modal_text.innerHTML = "There was a problem creating your account, contact support for more information." + error
        modal.style.display = "block";
        document.body.style.overflow = "hidden";
    });

    submit_button.disabled = false;
}