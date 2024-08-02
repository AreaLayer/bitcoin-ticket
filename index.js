import init, { create_ticket_event, purchase_ticket } from './pkg/bitcoin_ticket_platform.js';

let eventDetails = null;

async function run() {
    await init();
    document.getElementById('create-event').addEventListener('click', () => {
        const name = document.getElementById('event-name').value;
        const price = parseInt(document.getElementById('event-price').value);
        eventDetails = create_ticket_event(name, price);
        document.getElementById('event-details').innerText = `Event Created: ${JSON.stringify(eventDetails)}`;
    });

    document.getElementById('purchase-ticket').addEventListener('click', () => {
        if (eventDetails) {
            const purchaseDetails = purchase_ticket(eventDetails);
            document.getElementById('purchase-details').innerText = purchaseDetails;
        } else {
            document.getElementById('purchase-details').innerText = 'No event created yet.';
        }
    });
}

run();
