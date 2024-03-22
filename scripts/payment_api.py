import mercadopago
import json
from mercadopago import config

sdk = mercadopago.SDK("TEST-4566808951513212-022113-66e2c9a3b1f3f52a47290e91158f053e-1677856707")
request_options = config.RequestOptions("TEST-4566808951513212-022113-66e2c9a3b1f3f52a47290e91158f053e-1677856707")

def generate_payment_url() -> str:
    prefer = {
        "items": [
            {
                "id": 0,
                "description": "Assinatura do Plano Bronze",
                "title": "Plano Bronze",
                "quantity": 1,
                "currency_id": "BRL",
                "unit_price": 4.50
            }
        ]
    }
    preference = sdk.preference().create(prefer, request_options)
    return preference["response"]["init_point"]