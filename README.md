# rust-nb-server

Start the server by `cargo run`

To train a model `spamchecker`:

```sh
curl -X PUT \
  http://localhost:8000/model/spamchecker \
  -H 'Content-Type: application/json' \
  -d '{
	"updates": [
		[
			"spam",
			[{
				"feature_type": "Text",
				"name": "email.body",
				"value": "Good day dear beneficiary. This is Secretary to president of Benin republic is writing this email ... heritage, tax, dollars, money, credit card..."
				},
			{
				"feature_type": "Category",
				"name": "email.domain",
				"value": "evil.me"},
			{
				"feature_type": "Gaussian",
				"name": "email.n_words",
				"value": "400"
				}
			]
		],
		[
			"not spam",
			[{
				"feature_type": "Text",
				"name": "email.body",
				"value": "Hey bro, let'\''s go to have some hotpot soon..."
			},
			{
				"feature_type": "Category",
				"name": "email.domain",
				"value": "gmail.com"},
			{
				"feature_type": "Gaussian",
				"name": "email.n_words",
				"value": "40"
				}
			]
		]
	]
}'
```

To predict:
```
curl -X POST \
  http://localhost:8000/model/spamchecker \
  -H 'Content-Type: application/json' \
  -d '{
    "features": [
    [
    	{
			"feature_type": "Text",
			"name": "email.body",
			"value": "Give me your credit card number"
		},
		{
			"feature_type": "Category",
			"name": "email.domain",
			"value": "gmail.com"
		},
		{
			"feature_type": "Gaussian",
			"name": "email.n_words",
			"value": "500"
		}
    ],
    [
    	{
			"feature_type": "Text",
			"name": "email.body",
			"value": "Hotpot again?"
		},
		{
			"feature_type": "Category",
			"name": "email.domain",
			"value": "gmail.com"
		},
		{
			"feature_type": "Gaussian",
			"name": "email.n_words",
			"value": "48"
		}
    ]
    ]
}'
```

Example output will be:

```
{
    "predictions": [
        {
            "not spam": 0,
            "spam": 1
        },
        {
            "not spam": 1,
            "spam": 0
        }
    ]
}
```