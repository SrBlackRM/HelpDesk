let login_form = document.getElementById("login");

login_form.addEventListener('submit', (e)=>{
    e.preventDefault();

    let login_obj = {
        email: document.getElementById("email").value,
        password: document.getElementById("password").value,
    }

   sendData(login_obj);
})

async function sendData(login_obj){
    let send_method = "POST";
    let send_header = {
        "Content-Type": "application/json"
    }
    let login_path = "/login";

    try{
        let response = await fetch(login_path, {
            method: send_method,
            headers: send_header,
            body: JSON.stringify(login_obj),
        })
        if (response.ok) {
            window.location.href("/new_ticket")
        } else {
            alert("Erro ao fazer login");
        }
    }
    catch{
        console.log("Erro na requisição: ", error);
        alert("Erro ao se conectar com o servidor! ");
    }

}