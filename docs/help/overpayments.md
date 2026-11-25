# What happens if I overpay?

Shade invoice checkout rejects a payment that is larger than the unpaid invoice balance. The extra amount is not taken, and checkout does not automatically send a separate refund.

<a id="overpayments"></a>

## If you see an unexpected amount

1. Check the invoice balance and the amount shown in your wallet before approving.
2. If the wallet request is higher than the remaining balance, reject it and reload checkout.
3. If you already approved a transaction and it is confirmed, send the transaction ID and invoice details to the merchant.

Do not submit another payment until the merchant has checked the transaction. A second, separate payment may not be treated as an overpayment on the original invoice.
