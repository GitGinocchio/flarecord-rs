

- [x] Aggiungere l'endpoint api per `/sync` per sincronizzare i comandi
- [ ] Aggiungere l'endpoint `/health` per visualizzare statistiche, stato e informazioni del bot
- [ ] Sostituire `reqwest` con `worker::HttpRequest`
- [x] Creare dei comandi "default" da abilitare per il debug per esempio /update_commands
- [x] Utilizzare id gerarchici tipo: c1:s2:b3 (container1:section2:button3)
- [x] Il RootComponent deve essere creato dal framework che genera anche l'id automaticamente

- [ ] Aggiungere un wrapper Message(TwilightMessage)
- [ ] Aggiungere `CommandResponse::from_message(message: Message)`