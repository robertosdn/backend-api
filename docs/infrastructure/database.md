# Infraestrutura: MySQL

## Status

Aprovada para a primeira implementacao persistente.

## Escolha

- Banco: MySQL.
- Storage engine: InnoDB em todas as tabelas de dominio e outbox.
- Charset: `utf8mb4`.
- Collation: definir explicitamente por ambiente, preferindo uma collation `utf8mb4` deterministica.
- Acesso: crate Rust de persistencia a ser registrada na ADR da implementacao.

> O MySQL possui o charset historico `utf8` limitado a 3 bytes. Para suportar Unicode completo, inclusive emoji, o projeto deve usar `utf8mb4` nas tabelas, conexoes e migracoes.

## Regras De Modelagem

- Toda tabela deve declarar `ENGINE=InnoDB` e `CHARACTER SET=utf8mb4`.
- Chaves primarias e estrangeiras devem ser indexadas.
- Emails devem ter normalizacao e unicidade definidas no schema.
- Senhas devem ser armazenadas somente como hash.
- Datas devem usar uma convencao UTC documentada.
- Alteracoes de schema devem ser versionadas por migracoes reproduziveis.

## Outbox Por Tabela

Cada tabela de dominio que produzir eventos tera sua propria tabela de outbox. Para o usuario de acesso, por exemplo, a tabela `access_users` sera acompanhada de `access_users_outbox`.

A escrita do registro de dominio e do registro correspondente na outbox deve ocorrer na mesma transacao InnoDB. A outbox deve conter, no minimo, `event_id`, `aggregate_id`, `event_type`, `payload`, `status`, `attempts`, `available_at`, `created_at`, `published_at` e `last_error`.

Cada outbox pode ser processada independentemente, mas todos os eventos devem carregar um identificador globalmente unico para idempotencia no RabbitMQ e nos consumidores.

## Testes

- Testar migracoes em banco MySQL real via Docker.
- Testar rollback quando a escrita da outbox falhar.
- Testar unicidade, concorrencia otimista e charset `utf8mb4`.
- Testar que nenhuma senha ou segredo e persistido em texto puro.
