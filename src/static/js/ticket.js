// Listener form

let open_ticket_form = document.getElementById('open_ticket_form');


open_ticket_form.addEventListener('submit', (e) => {
    e.preventDefault();

    let ticket = {
        ticket_title: document.getElementById('titulo').value,
        ticket_description: document.getElementById('descricao').value,
        category_id: parseInt(document.getElementById('categoria').value, 10),
        ticket_client_id: 1
    }
    
    sendTicketToBackEnd(ticket)
})

async function sendTicketToBackEnd(ticket){
    let ticket_path = "/new_ticket";
    let send_method = "POST";
    let send_headers = {"Content-Type": "application/json"}


    try{
        let response = await fetch(ticket_path, {
            method: send_method,
            headers: send_headers,
            body: JSON.stringify(ticket)
        });

        if (response.ok) {
            alert("Chamado criado com sucesso!!");
            open_ticket_form.reset();
        } else {
            alert("Erro ao abrir chamado!");
        }
    }
    catch {
        console.log("Erro na requisição: ", error);
        alert("Erro ao se conectar com o servidor! ");
    }
}