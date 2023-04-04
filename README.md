# edge-creds-pusher

A basic Lambda that assumes a role on AWS - and pushes the aws credentials to
an external service config (ie: Fastly dictionary).

These credentials can then be used in these external services to (eg: Fastly
VCL) to sign AWS API requests (eg: to access DynamoDB tables or S3 buckets).

# usage

Simply build the function using `make build` - and deploy on AWS using your 
method of choice.

You will need to create an SSM parameter for the Fastly API key to be used
when pushing to the Fastly dictionary.

Configuration of the lambda is provided via environment variables:

`FASTLY_API_TOKEN_SSM_NAME`: Provide the full name of the SSM parameter
where the Fastly API token is stored (the lambda role will need access to
this + the KMS key if one was used to encrypt the value)
`FASTLY_SERVICE_ID`: Fastly Service ID where the dictionary is located.
`FASTLY_DICTIONARY_ID`: Fastly Dictionary ID where the values are to be
pushed.
`ASSUME_ROLE_ARN`: The AWS role that the lambda is to generate the keys
for.  Note: The Lambda role will require permissions to assume this role
+ the max session time will need to be at least the configured session time
set in the STS call (see sts.rs - `DEFAULT_SESSION_TIME`)
`LOG_LEVEL`: Self explanitory.  See rust tracing::Level for possible values.

You should then setup a scheduled event on Cloudwatch to trigger this function
regularly (eg: 1hr.  The default STS session duration is 4 hours).

An alarm should also be configured to alert as soon as there is a failure
pushing new credentials (assuming the default settings, this gives 3 hours to
rectify before the last credentials stored in the Fastly dict have expired).

