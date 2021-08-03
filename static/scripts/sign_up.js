const form = document.getElementById("signup_form");
const submit_button = document.getElementById("signup_submit");

submit_button.disabled=true;

const user_name_textbox = form.elements.namedItem("user_name");
const password_textbox = form.elements.namedItem("password");
const email_textbox = form.elements.namedItem("email");

//Assume that the form is not valid
let user_name_flag = true;
let password_flag = true;
let email_flag = true;

user_name_textbox.addEventListener("input", is_valid_user_name);
password_textbox.addEventListener("input", is_valid_password);
email_textbox.addEventListener("input", is_valid_email);

function enable_submission(){
    console.log(user_name_flag)
    console.log(password_flag)
    console.log(email_flag)
    if (user_name_flag || password_flag || email_flag) {
        submit_button.disabled = true;
    }
    else{
        submit_button.disabled = false;
    }
}

function is_valid_email(event){
    let regex = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
    email_flag = !(regex.test(event.target.value))
    enable_submission();
}

function is_valid_user_name(event){
    console.log("Valid")
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
    enable_submission();
}

function is_valid_password(event){
    let password = event.target.value;
    console.log("Test")
    if ((has_capital_letter(password) && has_number(password) && has_symbol(password)) && password.length >=8 ){
        password_flag = false;
    }
    else{
        password_flag = true;
    }

    enable_submission();
}

function has_capital_letter(password){
    for(let i = 0; i < password.length; i++){
        let code = password.charCodeAt(i);
        if (code > 64 && code < 91) {
            console.log("Caps true")
            return true;
        }
    }
    return false;
}

function has_number(password){
    let regex = /[0-9]/g
    for(let i = 0; i < password.length; i++){
        if (password.charAt(i).match(regex)){
            console.log("Numbers true")
            return true;
        }
    }
    console.log("Numbers false")
    return false
}

function has_symbol(password){
    let format = /[ `!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?~]/;
    let flag = format.test(password)


    console.log("Symbol "+ flag)
    return flag
}

function create_account() {
    event.preventDefault();
    console.log("Running")
}