document.addEventListener('DOMContentLoaded', () => {
    
    const topContainer = document.querySelectorAll('.top-container');

    topContainer.forEach(container => {
        container.classList.add('top-container-show');
    })
});

document.addEventListener("DOMContentLoaded", function() {

    fetchWifiStatus();
    fetchWireguardStatus();

    setInterval(fetchWifiStatus, 5000);
    setInterval(fetchWireguardStatus, 5000);
});

async function sleep(time) {
    await new Promise(resolve => setTimeout(resolve, time))
}

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
    if (address && !/^(?:\d{1,3}\.){3}\d{1,3}\/(?:[0-9]|[1-2][0-9]|3[0-2])$/.test(address)) {
        setError('address-error', "Must be a valid CIDR address.");
    } else {
        clearError('address-error');
    }

    // Validate WireGuard Port
    const port = document.getElementById('port').value;
    if (port && (!/^\d{1,5}$/.test(port) || parseInt(port) > 65535)) {
        setError('port-error', "Must be a valid port between 0 and 65535.");
    } else {
        clearError('port-error');
    }

    // Validate WireGuard DNS
    const dns = document.getElementById('dns').value;
    if (dns && !/^(\d{1,3}\.){3}\d{1,3}$/.test(dns)) {
        setError('dns-error', "Must be a valid IP address.");
    } else {
        clearError('dns-error');
    }

    // Validate Client Private Key
    const privKey = document.getElementById('privkey').value;
    if (privKey.length > 32) {
        setError('privkey-error', "Must be 32 characters or less.");
    } else {
        clearError('privkey-error');
    }

    // Validate Remote Host Public Key
    const pubKey = document.getElementById('pubkey').value;
    if (pubKey.length > 32) {
        setError('pubkey-error', "Must be 32 characters or less.");
    } else {
        clearError('pubkey-error');
    }

    if (!isValid) {
        event.preventDefault();
    }

    document.getElementById('config').textContent = 'Saved.'

});

function connectWifi(event) {
    event.preventDefault();

    const form = event.target.closest('form');
    const passwordInput = form.querySelector('input[type="password"]');

    const wifiContainer = form.closest('.wifi');
    const errorDiv = wifiContainer.querySelector('.error');

    if (passwordInput.value.length > 64) {
        errorDiv.textContent = "Password must be 64 characters or less.";
        return;
    }

    errorDiv.textContent = "";

    form.submit();
}

async function fetchScannedWifis() {

    let scanned_wifis = document.getElementById('inner-scanned-wifis');
    scanned_wifis.innerHTML = "";

    try {
        document.getElementById('loading-svg').style.display = 'flex'; 
        
        const response = await fetch('/wifi');
        
        if (!response.ok) throw new Error('Error fetching scanned Wi-Fis.');

        const scannedWifis = await response.text();

        document.getElementById('loading-svg').style.display = 'none';

        scanned_wifis.innerHTML = scannedWifis;

        document.querySelectorAll('.wifi-connect button[type="submit"]').forEach(button => {
            button.addEventListener('click', connectWifi);
        });
    } 
    catch (error) {
        scanned_wifis.style.fontWeight = 'bold';
        scanned_wifis.innerHTML = 'Error fetching scanned Wi-Fis.';
        
        document.getElementById('loading-svg').style.display = 'none';
    }
}

function toggleDropdown(event, element) {
    if (event.target.closest('.wifi-connect')) return;

    const form = element.querySelector('.wifi-connect');
    const wifiContainer = element.closest('.wifi');

    form.classList.toggle('visible');
    wifiContainer.classList.toggle('expanded');
}

async function fetchWireguardStatus() {
    try {
        const response = await fetch("/wg-status");

        if (!response.ok) {
            console.error("Failed to fetch Wireguard status:", response.statusText);
            return;
        }

        const htmlContent = await response.text();
        const statusElement = document.getElementById("wireguard-status");

        if (htmlContent === statusElement.innerHTML){
            return;
        }

        statusElement.innerHTML = htmlContent;

    } catch (error) {
        console.error("Error fetching Wireguard status:", error);
    }
}

async function fetchWifiStatus() {
    try {
        const response = await fetch("/wifi-status");

        if (!response.ok) {
            console.error("Failed to fetch Wi-Fi status:", response.statusText);
            return;
        }

        const htmlContent = await response.text();
        const statusElement = document.getElementById("wifi-status");

        if (htmlContent === statusElement.innerHTML){
            return;
        }

        statusElement.innerHTML = htmlContent;

    } catch (error) {
        console.error("Error fetching Wi-Fi status:", error);
    }
}