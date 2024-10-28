document.getElementById('config').addEventListener('submit', function(event){
    
    const ssid = document.getElementById('ssid').value;
    if (ssid.length > 32){
        alert("WIFI SSID must be 32 characters or less.");
        event.preventDefault();
        return;
    }

    const password = document.getElementById('passwd').value;
    if (password.length > 64){
        alert("WIFI Password must be 64 characters or less.");
        event.preventDefault();
        return;
    }

    const address = document.getElementById('address').value;
    if (address.length > 32){
        alert("WireGuard Address must be 32 characters or less.");
        event.preventDefault();
        return;
    }
    if (address && !/^(?:\d{1,3}\.){3}\d{1,3}\/(?:[0-9]|[1-2][0-9]|3[0-2])$/.test(address)){
        alert("WireGuard Address must be a valid CIDR address (e.g., 0.0.0.0/24).");
        event.preventDefault();
        return;
    }

    const port = document.getElementById('port').value;
    if (port && (!/^\d{1,5}$/.test(port) || parseInt(port) > 65535)){
        alert("WireGuard Port must be a valid port between 0 and 65535.");
        event.preventDefault();
        return;
    }

    const dns = document.getElementById('dns').value;
    if (dns && !/^(\d{1,3}\.){3}\d{1,3}$/.test(dns)){
        alert("WireGuard DNS must be a valid IP address (e.g., 192.168.1.1).");
        event.preventDefault();
        return;
    }

    const privKey = document.getElementById('privkey').value;
    if (privKey.length > 32){
        alert("WireGuard Client Private Key must be 32 characters or less.");
        event.preventDefault();
        return;
    }

    const pubKey = document.getElementById('pubkey').value;
    if (pubKey.length > 32){
        alert("WireGuard Server Public Key must be 32 characters or less.");
        event.preventDefault();
        return;
    }

});
