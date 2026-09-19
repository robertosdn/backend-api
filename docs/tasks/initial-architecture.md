# Tarefas: Gestao De Usuario De Acesso

## Decisoes E Fundacao

- [x] Registrar MySQL/InnoDB, `utf8mb4`, crate de persistencia pendente e estrategia de migracao.
- [x] Registrar formato da credencial, algoritmo de hash e politica de autorizacao.
- [x] Registrar RabbitMQ como broker e Redis para sessoes e dados temporarios.
- [x] Registrar Elasticsearch como read model exclusivo das queries.
- [x] Registrar uma tabela de outbox por tabela de dominio que produza eventos.
- [x] Definir modulos e contratos de commands, queries, dominio e repositorios.
- [x] Definir modelo de usuario, status, unicidade de email e concorrencia otimista.

## Persistencia E Outbox

- [x] Criar migracao/tabela de usuarios sem armazenar senha em texto puro.
- [x] Criar tabela transacional de outbox por tabela de dominio, com status, tentativas e timestamps.
- [x] Implementar unidade transacional para usuario e evento.
- [x] Documentar e configurar stack Docker Compose para subir infraestrutura e aplicar migrações SQL no bootstrap.
- [x] Documentar processador da outbox com retry, backoff e idempotencia.
- [x] Implementar processador da outbox com retry, backoff e idempotencia.
- [x] Documentar projetor RabbitMQ -> Elasticsearch.
- [x] Implementar projetor RabbitMQ -> Elasticsearch.
- [x] Documentar reindexacao do Elasticsearch a partir do MySQL sem usar MySQL no caminho normal das queries.
- [x] Implementar reindexacao do Elasticsearch a partir do MySQL sem usar MySQL no caminho normal das queries.

## Endpoints E Casos De Uso

- [x] Criar a estrutura modular fisica prevista em `docs/plans/initial-architecture.md` antes dos endpoints: `domain`, `application/commands`, `application/queries`, `repositories`, `outbox`, `auth`, `http` e `infrastructure`.
- [ ] Implementar `POST /api/v1/access-users` seguindo `http -> command -> dominio -> repositorio/outbox`, com validacao de email, nome e senha; email unico, hash Argon2id, evento transacional na outbox e resposta sem `password_hash`.
- [ ] Implementar `PATCH /api/v1/access-users/{id}`.
- [ ] Implementar `GET /api/v1/access-users/{id}`.
- [ ] Implementar `GET /api/v1/access-users` com paginacao.
- [ ] Implementar `POST /api/v1/auth/login`.
- [ ] Garantir que respostas e logs nunca exponham senha, hash ou token.

## Testes E Retirada

- [x] Verificar que `src/app.rs` contem somente composicao do router e registro de rotas, sem regras de dominio, hash, persistencia ou estado de usuarios.
- [ ] Criar o diretorio `tests/` e segmentar a suite em `domain/`, `application/`, `repositories/`, `outbox/`, `http/` e `fixtures/`.
- [ ] Remover a concentracao da suite em `src/tests.rs`; nenhum arquivo monolitico de testes deve permanecer.
- [ ] Adicionar testes de dominio em `tests/domain/` para validacao, normalizacao, senha e transicoes de status.
- [ ] Adicionar testes de commands e queries em `tests/application/`, cobrindo concorrencia e regras sem efeitos colaterais.
- [ ] Adicionar testes de repositorios em `tests/repositories/`, incluindo unicidade e atomicidade com a outbox.
- [ ] Adicionar testes do processador em `tests/outbox/`, cobrindo retry, backoff e reprocessamento apos falha de publicacao.
- [ ] Adicionar testes HTTP em `tests/http/`, cobrindo endpoints, codigos de erro, login valido, invalido, inexistente e usuario desabilitado.
- [ ] Adicionar testes de queries por id, filtros, paginacao e busca no Elasticsearch.
- [ ] Adicionar teste de falha do Elasticsearch sem fallback para MySQL.
- [ ] Adicionar MySQL, RabbitMQ, Elasticsearch e Redis ao Docker Compose com health checks e executar formatacao, testes e clippy dentro do Docker.
- [ ] Remover rotas e testes de `hello` e `echo` apos a nova API estar validada.
- [ ] Atualizar a especificacao, o plano e esta lista com os resultados.

## Gate De Conclusao

Nenhuma tarefa de endpoint pode ser marcada como concluida enquanto a estrutura modular prevista nao existir e enquanto `app.rs` concentrar responsabilidades de dominio, aplicacao ou infraestrutura. A revisao deve conferir os caminhos dos arquivos, os contratos entre camadas e os testes correspondentes.
