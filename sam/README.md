# SAM

For running the lambda locally (note: it *will* assume the role specified 
and write credentials to the fastly service+dict configured in the env 
parameters)

```
AWS_PROFILE=<profile that can assume the configured role> sam local invoke -e event_cloudwatch.json
```


*requires*: https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/install-sam-cli.html
