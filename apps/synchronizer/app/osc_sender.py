from pythonosc import udp_client

def send_osc_message(path, message, address="127.0.0.1", port=1234):
    """
    Sends an OSC message to the specified address and port.

    :param address: The IP address of the OSC server.
    :param port: The port number of the OSC server.
    :param path: The OSC address path to send the message to.
    :param message: The message to be sent.
    """
    client = udp_client.SimpleUDPClient(address, port)
    client.send_message(path, message)