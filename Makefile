# 開発環境のコンテナを起動
.PHONY: rust-dev-up
rust-dev-up:
	docker compose up -d rust-dev
	docker compose up -d db-dev

.PHONY: rust-dev-in
rust-dev-in:
	docker compose exec rust-dev bash

# 本番環境のコンテナを起動
.PHONY: prod
prod:
	docker compose up -d rust-prod