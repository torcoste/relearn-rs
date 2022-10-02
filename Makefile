generate-openapi:
	# generate rust client
	openapi-generator generate --input-spec openapi.yaml  \
		--generator-name rust \
		--output sdk \
		--additional-properties packageName=relearnsdk

	# generate rust server
	openapi-generator generate --input-spec openapi.yaml  \
		--generator-name rust-server \
		--output server \
		--additional-properties packageName=relearnserver
