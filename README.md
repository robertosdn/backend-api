# REST API Rust

API backend completa em Rust com Axum e Tokio. Gestao de usuarios de acesso e uma das features planejadas do backend.

## Estado Do Projeto

Atualmente existem endpoints temporarios para validar o servidor:

- `GET /hello`
- `GET /hello/`
- `GET /hello/{name}`
- `POST /echo`

Eles serao removidos quando as primeiras features reais do backend estiverem implementadas e cobertas por testes.

## Arquitetura

- **CQRS**: commands alteram o write model; queries consultam o read model.
- **MySQL/InnoDB**: write model e fonte de verdade.
- **Transactional Outbox**: uma outbox propria para cada tabela de dominio que produzir eventos.
- **RabbitMQ**: transporte de eventos apos o commit.
- **Elasticsearch**: read model exclusivo das queries, inclusive consultas por id.
- **Redis**: somente sessoes, tokens revogados, rate limiting e dados temporarios; nao participa das queries de usuarios.
- **Docker**: ambiente padrao para build, testes e execucao.

O fluxo de desenvolvimento e:

```text
spec -> plan -> tasks -> implementation -> tests -> update docs
```

## Executar Com Docker

Iniciar o servico atual:

```bash
docker compose up --build rest-server
```

Executar os testes no estagio Docker de testes:

```bash
docker compose --profile test build rest-test
docker compose --profile test run --rm rest-test
```

A API atual fica disponivel em `http://localhost:8080`.

## Desenvolvimento Orientado Por Especificacao

Antes de implementar uma funcionalidade:

1. Atualize ou crie a especificacao em `docs/specs/`.
2. Registre decisoes tecnicas em `docs/decisions/`.
3. Atualize o plano em `docs/plans/`.
4. Derive tarefas em `docs/tasks/`.
5. Implemente em modulos separados por responsabilidade.
6. Adicione testes unitarios e de integracao.
7. Atualize a documentacao e execute as validacoes Docker.

## Documentacao

- [`AGENTS.md`](AGENTS.md): instrucoes compartilhadas para pessoas e agentes de qualquer ferramenta.
- [`docs/README.md`](docs/README.md): indice da documentacao SDD.
- [`docs/constitution.md`](docs/constitution.md): principios e Definition of Done.
- [`docs/architecture.md`](docs/architecture.md): arquitetura e limites entre write/read model.
- [`docs/specs/access-user-management.md`](docs/specs/access-user-management.md): especificacao da API de usuarios.
- [`docs/infrastructure/`](docs/infrastructure/): MySQL, RabbitMQ, Elasticsearch e Redis.
- [`docs/decisions/`](docs/decisions/): ADRs e consequencias das escolhas.
- [`docs/plans/`](docs/plans/): planos de implementacao.
- [`docs/tasks/`](docs/tasks/): checklists executaveis.

## Regras Para Desenvolvimento

- Manter commands, queries, dominio, persistencia, handlers e projetores em modulos separados.
- Nao consultar MySQL no caminho normal das queries; usar Elasticsearch.
- Nao publicar eventos antes do commit da transacao.
- Nunca armazenar ou expor senhas em texto puro.
- Criar testes para cada comportamento novo ou alterado.
- Revisar ownership, concorrencia, `unsafe`, ciclos de referencias e possiveis vazamentos de memoria.
