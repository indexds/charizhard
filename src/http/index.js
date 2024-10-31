document.addEventListener('DOMContentLoaded', () => {
    const wifiContainer = document.querySelector('.wifi-container');
    const formContainer = document.querySelector('.form-container');
    
    wifiContainer.classList.add('container-show');
    formContainer.classList.add('container-show');
});

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
        document.getElementById('loading-svg').style.display = 'flex'; 
        
        const response = await fetch('/wifi');
        
        console.log('Response status:', response.status);
        if (!response.ok) throw new Error('Error fetching scanned Wi-Fis.');

        const scannedWifis = await response.text();

        document.getElementById('loading-svg').style.display = 'none';

        scanned_wifis.innerHTML = scannedWifis;
    } 
    catch (error) {
        scanned_wifis.style.fontWeight = 'bold';
        scanned_wifis.innerHTML = 'Error fetching scanned Wi-Fis.';
        
        document.getElementById('loading-svg').style.display = 'none'; 
        console.error('Fetch error:', error);
    }
}

function toggleDropdown(event, element) {
    if (event.target.closest('.wifi-connect')) return;

    const form = element.querySelector('.wifi-connect');
    const wifiContainer = element.closest('.wifi');

    form.classList.toggle('visible');
    wifiContainer.classList.toggle('expanded');
}

