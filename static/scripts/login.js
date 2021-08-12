const login_form = document.getElementById("login_form");
const submit_button = document.getElementById("login_submit");

const user_name_textbox = login_form.elements.namedItem("user_name");
const password_textbox = login_form.elements.namedItem("password");

let user_name_flag = true;
let password_flag = true;

let is_logged_in = false;

user_name_textbox.addEventListener("input", is_valid_user_name);
password_textbox.addEventListener("input", is_valid_password);

function toggle_submission(){
    if (user_name_flag || password_flag ) {
        submit_button.disabled = true;
    }
    else{
        submit_button.disabled = false;
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

async function login_account() {
    submit_button.disabled = true;
    event.preventDefault();

    const form_data =  decodeURIComponent(new URLSearchParams(new FormData(event.target)));
    console.log(form_data.toString());

    await fetch("/api/users/login", {
        method: 'POST', // *GET, POST, PUT, DELETE, etc.
        mode: 'same-origin', // no-cors, *cors, same-origin
        credentials: 'same-origin', // include, *same-origin, omit
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: form_data
    }).then(response => {
        if (response.status === 200) {
            is_logged_in = true;
        }
        return response.json();
    }).then(response_data => {
        if (!is_logged_in) {
            console.log(response_data)
            let json_response = JSON.parse(response_data);
            console.log(json_response);
            let error_message = document.getElementById("login_error");
            error_message.innerHTML = json_response["message"];
            error_message.style.display = "block";
        }
        else {
            window.location.href = "../index.html"
        }
    }
    ).catch(error => {
        let error_message = document.getElementById("login_error");
        error_message.innerHTML = "Could not authenticate, contact support for more information";
        error_message.style.display = "block";
    });

    submit_button.disabled = false;
}

