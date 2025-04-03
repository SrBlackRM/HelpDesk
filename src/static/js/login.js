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
        .then(response => response.json())
        .then(data => {
            if (data.token) {
                localStorage.setItem("token", data.token); // Armazena o token no navegador
                window.location.href = "/new_ticket";  // Redireciona para a página restrita
            } else {
                alert("Login falhou!");
            }
        })
        .catch(error => console.error("Erro ao logar:", error));
    }
    catch{
        console.log("Erro na requisição");
        alert("Erro ao se conectar com o servidor! ");
    }

}