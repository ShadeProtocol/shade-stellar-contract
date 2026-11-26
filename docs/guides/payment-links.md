# Payment links and QR codes for invoices

Share a hosted invoice URL when you want a payer to open a page that fetches a Shade invoice and prepares a `pay_invoice` transaction. This guide defines the URL fields, explains the current SEP-7 options, and gives practical QR-code recommendations.

## Recommended hosted payment link

Use an HTTPS page on a domain you control. The page should read the invoice from the selected Shade contract, show the merchant, amount, token, and network, then build and simulate `pay_invoice(payer, invoice_id)` before asking the connected wallet to sign. See [Wallet integration](wallet-integration.md) for the Soroban wallet flow.

Use these query parameters so links can be consumed consistently by merchant tools and hosted checkout pages:

| Parameter | Required | Meaning |
|---|---|---|
| `network` | Yes | Network name, such as `testnet` or `public` |
| `contract` | Yes | Shade contract ID on that network |
| `invoice` | Yes | Decimal `u64` invoice ID |

The canonical form is:

```text
https://<merchant-domain>/pay/shade?network=<network>&contract=<shade-contract-id>&invoice=<invoice-id>
```

For example, this is a **template**, not a live invoice. Replace the contract ID and invoice ID with values from your own testnet deployment, and replace the example domain with the payment page you control:

```text
https://merchant.example/pay/shade?network=testnet&contract=<TESTNET_SHADE_ID>&invoice=1
```

Treat the link as a locator, not as payment authorization. A payment page must fetch the invoice from the specified network and contract, display the current on-chain details, and have the payer review the actual transaction in their wallet. Do not trust an amount, token, or merchant name supplied only by the URL.

## SEP-7 support

[SEP-7](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0007.md) defines `web+stellar:pay` for account payments and `web+stellar:tx` for signing a supplied Stellar `TransactionEnvelope` in XDR form.

- `web+stellar:pay` is not an invoice request: it has no standard Shade contract or invoice identifier and does not invoke `pay_invoice`.
- `web+stellar:tx` can carry a prebuilt transaction XDR, including a Soroban transaction with an `InvokeHostFunction` operation. However, SEP-7 does not define Shade invoice discovery, does not build the transaction, and does not guarantee that a given wallet supports Soroban invocation or the required Soroban authorization flow. Wallet support varies; verify it with the target wallet before relying on this route.

For a wallet that supports the particular Soroban XDR, an application could prepare and simulate `pay_invoice` and pass the resulting transaction to a SEP-7 `tx` handler. Because transaction XDR is bulky and wallet support is not universal, prefer the hosted URL above for general-purpose links and QR codes. Never put a secret key or an unsigned, unreviewed payment request in a link.

Illustrative SEP-7 shape (the XDR placeholder is not scannable or signable):

```text
web+stellar:tx?xdr=<URL-ENCODED-BASE64-TRANSACTION-ENVELOPE>&network_passphrase=Test%20SDF%20Network%20%3B%20September%202015
```

## QR code recommendations

Encode the hosted HTTPS URL, not a long SEP-7 URI containing transaction XDR. Keep the URL short (aim for at most about 200 characters); QR capacity depends on encoding and error-correction level, and shorter payloads scan more reliably.

For the example template above, generate a fresh QR code using your real testnet contract ID and invoice ID. For example, with the [`qrcode` npm package](https://www.npmjs.com/package/qrcode):

```javascript
import QRCode from "qrcode";

const contractId = process.env.TESTNET_SHADE_ID;
if (!contractId) throw new Error("Set TESTNET_SHADE_ID to your deployed contract ID.");

const paymentUrl = new URL("https://merchant.example/pay/shade");
paymentUrl.searchParams.set("network", "testnet");
paymentUrl.searchParams.set("contract", contractId);
paymentUrl.searchParams.set("invoice", "1");

const svg = await QRCode.toString(paymentUrl.toString(), {
  type: "svg",
  errorCorrectionLevel: "M",
  margin: 4,
});
console.log(svg); // Embed this SVG or save it for printing.
```

Use these starting points for in-person use:

- Use error correction level **M** (about 15% recovery) for a clean print or screen; use **Q** (about 25%) if the surface may be damaged. Higher correction can make a larger, denser code.
- Preserve the four-module quiet zone around the QR code. Do not cover it with a logo or decorative elements.
- Print at least **3 cm (about 1.2 in) square** for close scanning; increase the size for longer scan distances. Keep strong contrast, avoid glare, and test the printed code with common phone cameras before deployment.
- Show the merchant and invoice details next to the code, and let the payer verify those details after opening the link.

The code snippet produces a QR code for an example testnet URL when `TESTNET_SHADE_ID` is set and invoice `1` exists. It does not create or verify that invoice.

## Security checklist

- Check the contract ID and network against a trusted source before signing. Contract IDs are network-specific; a testnet label in a URL does not prove that the linked page or wallet is actually using testnet.
- Fetch and display the invoice directly from that contract. Verify its merchant, status, remaining amount, and token before constructing the transaction.
- Review the wallet's Soroban invocation and token transfers. Reject unexpected contract IDs, methods, arguments, or network passphrases.
- Be alert for look-alike domains, substituted QR stickers, shortened URLs, and links that differ by a character. Navigate to a known merchant domain when unsure.
- Only use HTTPS and a domain the merchant controls. Do not trust a QR code's appearance as proof of authenticity.

## Related pages

- [Wallet integration](wallet-integration.md) — build, simulate, sign, and submit `pay_invoice`.
- [Accepting payments](accepting-payments.md) — invoice payment behavior and authorization.
- [SEP-7 specification](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0007.md) — URI scheme for delegated signing.

← [Back to guides](README.md)
