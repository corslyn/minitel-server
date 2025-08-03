import serial
import time

def menu():
    print("[+] Affichage 3615 Teletel")
    code_service=""
    with open("ecrans/teletel.vdt", "rb") as f:
        data = f.read()
        m.write(data) # affichage de la page vers le client

    while True:
        if m.cd == False:
            # deconnexion
            break
        c = m.read(1).decode()
        if c == '': # rien
            continue
        elif c == '\x13': # code SUP -> touche spéciale minitel (cx/fin, sommaire, guide, annulation, correction, retour, suite, répétition, envoi)
            c = m.read(1).decode()
            if c == '\x44': # guide
                m.write(b"\x0ctouche guide")
                time.sleep(3)
                menu()
            elif c == '\x49': # cx/fin
                m.write(b"\x0cAu revoir !")
                time.sleep(1)
                m.close()
            elif c == '\x41':
                service(code_service)
        elif c.isprintable(): # caractère normal ASCII -> clavier classique
            code_service += c
            m.write(c.encode()) # echo vers le client

def service(code_service):
    match code_service:
        case "JEANBON":
            with open("3615_jeanbon.vdt", "rb") as f:
                data = f.read()
                m.write(data)

            while True:
                c = m.read(1).decode()
                if c == '':
                    continue
                elif c == '\x13':
                    c = m.read(1).decode()
                    if c == '\x49': # cx/fin
                        menu()
        case _:
            m.write(b"\x0cErreur: Le service '" + code_service.encode() + b"' est inconnu")
            time.sleep(5)
            menu()


global m
m = serial.Serial('/dev/ttyUSB0', 1200, parity=serial.PARITY_EVEN, bytesize=7, timeout=2) # 1200 bauds, 7-E-1 : 7 bits de donnees, parité paire, 1 bit d'arret

init = "ATE0L0M0X4&N2S27=16S10=100S0=1\r" # https://noelmrtn.fr/posts/v23_server/

m.write(b"ATZ0\r") # on reset le modem
m.write(init.encode()) # on initialise le modem

if m.cd: # verifie si on a le carrier detect, ce qui signifie qu'on est connecté à un minitel/modem en face
    time.sleep(2)
    m.reset_input_buffer()
    menu()
else:
    #implementer une vraie boucle pour checker en permanence si on a une co, au lieu d'avoir a relancer le script à la main
    print("[!] Pas de connexion etablie")
