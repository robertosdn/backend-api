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

- [ ] Implementar `POST /api/v1/access-users` com validacao de email, nome e senha; email unico, hash Argon2id, evento transacional na outbox e resposta sem `password_hash`.
- [ ] Implementar `PATCH /api/v1/access-users/{id}`.
- [ ] Implementar `GET /api/v1/access-users/{id}`.
- [ ] Implementar `GET /api/v1/access-users` com paginacao.
- [ ] Implementar `POST /api/v1/auth/login`.
- [ ] Garantir que respostas e logs nunca exponham senha, hash ou token.

## Testes E Retirada

- [ ] Adicionar testes unitarios de dominio, validacao, senha e transicoes de status.
- [ ] Adicionar testes de commands, queries, concorrencia e atomicidade com a outbox.
- [ ] Adicionar testes de login valido, invalido, inexistente e usuario desabilitado.
- [ ] Adicionar testes HTTP dos endpoints e codigos de erro.
- [ ] Adicionar testes de retry e reprocessamento apos falha de publicacao.
- [ ] Adicionar testes de queries por id, filtros, paginacao e busca no Elasticsearch.
- [ ] Adicionar teste de falha do Elasticsearch sem fallback para MySQL.
- [ ] Adicionar MySQL, RabbitMQ, Elasticsearch e Redis ao Docker Compose com health checks e executar formatacao, testes e clippy dentro do Docker.
- [ ] Remover rotas e testes de `hello` e `echo` apos a nova API estar validada.
- [ ] Atualizar a especificacao, o plano e esta lista com os resultados.
