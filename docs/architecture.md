# Arquitetura

## Estado Atual

A aplicacao e um servico HTTP em Rust 2021 usando Axum e Tokio. O ponto de entrada esta em `src/main.rs` e a composicao das rotas esta em `src/app.rs`.

As rotas atuais `hello` e `echo` sao endpoints de demonstracao e estao marcadas para retirada em `docs/specs/hello-api.md`. A gestao de usuarios de acesso e a primeira feature de negocio documentada em `docs/specs/access-user-management.md`, dentro do backend completo.

O Dockerfile possui uma etapa `test`, que executa `cargo test`, e uma etapa `production`, que compila o binario release. O Docker Compose expoe o servico em `8080` e possui um perfil `test` para a imagem de testes.

A infraestrutura aprovada esta definida em `docs/decisions/adr-001-infrastructure-stack.md`: MySQL 9.7.2 com InnoDB e `utf8mb4` como write model, RabbitMQ 4.3.6 como broker, Elasticsearch 9.5.4 como read model e Redis 8.8 somente para sessoes e dados temporarios.

## Direcao Arquitetural

As novas funcionalidades devem seguir as camadas abaixo:

```text
HTTP handler
    |
    +--> Command handler --> Domain --> Write repository + Outbox repository
    |
    +--> Query handler   --> Elasticsearch read model

Outbox processor --> Event publisher --> Elasticsearch projector
```

### Commands

Commands representam intencoes que podem alterar estado. O command handler valida a entrada, executa a regra de dominio e grava a alteracao junto com o evento de integracao na mesma transacao.

### Queries

Queries somente leem o read model do Elasticsearch. Nao devem consultar MySQL como fallback, alterar estado, criar eventos ou depender de efeitos colaterais de commands.

### Transactional Outbox

A tabela ou armazenamento da outbox deve conter, no minimo, identificador do evento, tipo, payload, status, tentativas, timestamps e erro da ultima tentativa. Cada tabela de dominio que produzir eventos tera sua propria tabela de outbox. A alteracao de dominio e o registro da outbox devem ser confirmados atomicamente no MySQL/InnoDB.

Um processador separado busca eventos pendentes, publica cada evento no RabbitMQ e marca o registro como processado. O processamento deve ter retry, backoff, idempotencia e observabilidade. A publicacao deve ser at-least-once; consumidores precisam aceitar duplicatas.

### Redis Temporario

Redis nao participa do read model de CQRS. Deve ser usado somente para sessoes, tokens revogados, rate limiting e outros dados temporarios com TTL explicito. MySQL continua sendo o write model e Elasticsearch o read model.

## Restricoes

- Nao publicar eventos diretamente antes do commit da transacao.
- Nao misturar leitura e escrita no mesmo handler sem justificativa documentada.
- Nao introduzir banco, broker ou crate de persistencia sem atualizar esta arquitetura e a especificacao da funcionalidade.
- Manter o contrato HTTP existente salvo quando uma especificacao aprovada definir a mudanca.
