document.getElementById('config').addEventListener('submit', function(event) {
    let isValid = true;

    function setError(id, message) {
        const errorDiv = document.getElementById(id);
        errorDiv.textContent = message;
        isValid = false;
    }

    function clearError(id) {
        const errorDiv = document.getElementById(id);
        errorDiv.textContent = '';
    }

    // Validate Wi-Fi SSID
    const ssid = document.getElementById('ssid').value;

    const forbiddenChars = /[+\]\/"\t\s$]/; 
    const forbiddenFirstChars = /^[!#;]/;  
    
    if (ssid.length > 32) {
        setError('ssid-error', "WIFI SSID must be 32 characters or less.");
    } else if (forbiddenChars.test(ssid)) {
        setError('ssid-error', "WIFI SSID contains forbidden characters: +, ], /, \", TAB, or trailing spaces.");
    } else if (forbiddenFirstChars.test(ssid)) {
        setError('ssid-error', "WIFI SSID cannot start with any of these characters: !, #, ;");
    } else if (/\s$/.test(ssid)) {
        setError('ssid-error', "WIFI SSID cannot end with a space.");
    } else {
        clearError('ssid-error');
    }
    

    // Validate Wi-Fi Password
    const password = document.getElementById('passwd').value;
    if (password.length > 64) {
        setError('passwd-error', "WIFI Password must be 64 characters or less.");
    } else {
        clearError('passwd-error');
    }

    // Validate WireGuard Address
    const address = document.getElementById('address').value;
    if (address.length > 32) {
        setError('address-error', "WireGuard Address must be 32 characters or less.");
    } else if (address && !/^(?:\d{1,3}\.){3}\d{1,3}\/(?:[0-9]|[1-2][0-9]|3[0-2])$/.test(address)) {
        setError('address-error', "WireGuard Address must be a valid CIDR address (e.g., 0.0.0.0/24).");
    } else {
        clearError('address-error');
    }

    // Validate WireGuard Port
    const port = document.getElementById('port').value;
    if (port && (!/^\d{1,5}$/.test(port) || parseInt(port) > 65535)) {
        setError('port-error', "WireGuard Port must be a valid port between 0 and 65535.");
    } else {
        clearError('port-error');
    }

    // Validate WireGuard DNS
    const dns = document.getElementById('dns').value;
    if (dns && !/^(\d{1,3}\.){3}\d{1,3}$/.test(dns)) {
        setError('dns-error', "WireGuard DNS must be a valid IP address (e.g., 192.168.1.1).");
    } else {
        clearError('dns-error');
    }

    // Validate Client Private Key
    const privKey = document.getElementById('privkey').value;
    if (privKey.length > 32) {
        setError('privkey-error', "WireGuard Client Private Key must be 32 characters or less.");
    } else {
        clearError('privkey-error');
    }

    // Validate Remote Host Public Key
    const pubKey = document.getElementById('pubkey').value;
    if (pubKey.length > 32) {
        setError('pubkey-error', "WireGuard Server Public Key must be 32 characters or less.");
    } else {
        clearError('pubkey-error');
    }

    if (!isValid) {
        event.preventDefault();
    }
});

async function fetchScannedWifis() {

    let scanned_wifis = document.getElementById('inner-scanned-wifis');
    scanned_wifis.innerHTML = "";

    try {
        document.getElementById('loading-svg').style.display = 'block'; 
        
        const response = await fetch('/wifi');
        
        console.log('Response status:', response.status);
        if (!response.ok) throw new Error('Error fetching scanned Wi-Fi.');

        const scannedWifis = await response.text();

        document.getElementById('loading-svg').style.display = 'none';

        scanned_wifis.innerHTML = scannedWifis;
    } 
    catch (error) {
        scanned_wifis.innerHTML = 'Error fetching scanned Wi-Fi.';
        console.error('Fetch error:', error);
    }
}