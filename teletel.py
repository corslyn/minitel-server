import serial
import time

global m
m = serial.Serial('/dev/ttyUSB0', 1200, parity=serial.PARITY_EVEN, bytesize=7, timeout=2)

def menu():
    print("[+] Affichage 3615 Teletel")
    code_service=""
    with open("teletel.vdt", "rb") as f:
        data = f.read()
        m.write(data)

    while True:
        if m.cd == False:
            # deconnexion
            break
        c = m.read(1).decode()
        if c == '':
            continue
        elif c == '\x13':
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
        elif c.isprintable():
            code_service += c
            m.write(c.encode())

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

if m.cd:
    time.sleep(2)
    m.reset_input_buffer()
    menu()
else:
    print("[!] Pas de connexion etablie")
