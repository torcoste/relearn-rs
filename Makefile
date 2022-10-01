generate-openapi:
	# generate rust client
	openapi-generator generate --input-spec openapi.yaml  \
		--generator-name rust \
		--output generated-rust-client

	# generate rust server
	openapi-generator generate --input-spec openapi.yaml  \
		--generator-name rust-server \
		--output generated-rust-server

	# generate schema
	rm -rf mysql-schema
	openapi-generator generate -i openapi.yaml  \
	--generator-name mysql-schema \
	--output generated-schema \
	--additional-properties=namedParametersEnabled=true \
	--additional-properties=identifierNamingConvention=snake_case
