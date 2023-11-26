scripts := absolute_path('scripts')
root := absolute_path('')

format:
	cd "{{root}}" && cargo fmt
	prettier --write {{root}}

lint:
	cd "{{root}}" && cargo clippy
	prettier --check {{root}}
