# Especificacao: Gestao de Usuario de Acesso

## Status

Planejada.

## Objetivo

Substituir os endpoints de demonstracao por uma API real para criar, alterar, consultar e autenticar usuarios de acesso.

## Modelo Conceitual

Um usuario de acesso possui, no minimo:

- `id` imutavel;
- `email` unico e normalizado;
- `name` ou nome de exibicao;
- `password_hash`, nunca a senha em texto puro;
- `status`, inicialmente `active` ou `disabled`;
- timestamps de criacao e alteracao;
- controle de versao ou outra estrategia de concorrencia otimista.

A resposta HTTP nunca deve expor `password_hash`.

## Endpoints

### Criar usuario

`POST /api/v1/access-users`

- Command: `CreateAccessUser`.
- Deve validar email, nome e politica de senha.
- Deve rejeitar email duplicado com `409 Conflict`.
- Deve armazenar somente um hash de senha com algoritmo apropriado.
- Deve retornar `201 Created` sem senha ou hash na resposta.
- Deve registrar um evento `AccessUserCreated` na outbox na mesma transacao da criacao.

### Alterar usuario

`PATCH /api/v1/access-users/{id}`

- Command: `UpdateAccessUser`.
- Deve permitir alterar nome, email e status conforme as regras de autorizacao.
- Alteracao de email deve preservar unicidade e normalizacao.
- Alteracao de senha deve gerar novo hash e nunca armazenar a senha original.
- Deve retornar `200 OK` sem senha ou hash.
- Deve registrar evento de integracao somente quando houver mudanca de estado relevante.

### Consultar usuario

`GET /api/v1/access-users/{id}`

- Query: `GetAccessUser`.
- Deve retornar `200 OK` com dados publicos do usuario.
- Deve retornar `404 Not Found` quando o usuario nao existir.
- Nao deve alterar estado nem criar evento.

### Listar usuarios

`GET /api/v1/access-users`

- Query: `ListAccessUsers`.
- Deve suportar paginacao deterministica.
- Deve permitir filtros documentados, como status e email.
- Nao deve retornar senha ou hash.
- Nao deve alterar estado nem criar evento.

### Login

`POST /api/v1/auth/login`

- Command: `LoginAccessUser` ou caso de uso de autenticacao com leitura de credenciais.
- Deve localizar o usuario por email normalizado e verificar o hash da senha.
- Deve rejeitar credenciais invalidas com resposta generica, sem revelar se o email existe.
- Deve rejeitar usuario `disabled`.
- Em sucesso, deve retornar uma credencial de sessao ou token conforme decisao arquitetural registrada.
- Nao deve registrar senha, token ou credencial em logs.
- Tentativas de login e eventos de seguranca devem seguir a politica de outbox quando houver consumidores de integracao.

## CQRS E Outbox

Commands alteram estado no MySQL e queries somente leem o read model no Elasticsearch. Queries nunca devem consultar MySQL como fallback. A criacao, alteracao, desativacao e eventos de seguranca devem usar uma unidade transacional. A escrita do usuario e o registro do evento na outbox devem ser confirmados atomicamente.

O processador da outbox deve publicar eventos com entrega at-least-once, retry com backoff e comportamento idempotente. O algoritmo deve seguir: selecionar eventos `pending` e `available_at <= now`, tentar publicar no RabbitMQ, registrar tentativa e, em caso de erro, aumentar `attempts`, calcular delay exponencial e marcar `last_error`. Eventos com sucesso devem ser marcados como `published` e manter idempotencia pelo `event_id` para evitar duplicacao em reprocessamento.

O projetor RabbitMQ -> Elasticsearch deve consumir os eventos publicadas por aggregate de usuario e aplicar as alteracoes no read model, preservando `id`, `email`, `name`, `status`, `version` e timestamps. A indexacao deve ser idempotente por `id` do usuario e nunca depender de consultas ao MySQL. Consumidores devem aceitar duplicatas usando o identificador do evento.

A reindexacao do Elasticsearch a partir do MySQL e uma operacao de infraestrutura e manutencao, nao parte do caminho de leitura normal da API. Quando o read model precisar ser reconstruido, um job de reindexacao consulta o MySQL em batch, reescreve os documentos no Elasticsearch e invalida ou substitui os indices relevantes. As queries da API continuam 100% no Elasticsearch e nunca consultam MySQL como fallback.

## Modulos E Contratos

A implementacao deve seguir a divisao em modulos para preservar baixo acoplamento e manter a regra do projeto:

- `domain`: entidades, value objects, enums e regras internas. Ex.: `AccessUser`, `AccessUserStatus`, `EmailAddress`, `UserId`, `PasswordHash`.
- `application/commands`: comandos de escrita. Ex.: `CreateAccessUser`, `UpdateAccessUser`, `LoginAccessUser`, cada um recebendo um DTO e retornando `Result<...>` com erros de validacao ou dominio.
- `application/queries`: consultas de leitura. Ex.: `GetAccessUser`, `ListAccessUsers`, sempre retornando views publicas sem efeitos colaterais.
- `repositories`: contratos para escrita e leitura. `AccessUserWriteRepository` define `insert`, `update`, `find_by_id`, `find_by_email` e `save_outbox_event`; `AccessUserReadRepository` define busca por id, filtros e paginacao no Elasticsearch.
- `outbox`: entidade de evento transacional e processador com retry, backoff, idempotencia e observabilidade.
- `auth`: validacao de senha, emissao e validacao de token, e middleware para autorização por papel.
- `http`: DTOs, handlers e rotas, sem logica de dominio embutida.

Os contratos devem manter um boundary claro: handlers HTTP transformam HTTP em comandos/queries, os handlers de aplicacao executam validacao e dominio, e os repositorios sao a unica troca de dados com MySQL/Elasticsearch/Redis.

## Modelo De Usuario E Status

O modelo de dominio do usuario de acesso deve seguir a estrutura abaixo:

- `id`: identificador UUIDv7 ou bigint gerado por banco, imutavel;
- `email`: string normalizada para lowercase, sem espacos e validada por formato canonical;
- `name`: nome de exibicao com limite de caracteres e validacao de tamanho;
- `password_hash`: hash Argon2id, nunca armazenado em texto puro;
- `status`: enum `active | disabled`;
- `created_at` e `updated_at`: timestamps de auditoria;
- `version`: numero de versao para concorrencia otimista.

A tabela MySQL `access_users` deve seguir o esquema:

```sql
CREATE TABLE access_users (
  id BINARY(16) PRIMARY KEY,
  email VARCHAR(254) NOT NULL,
  name VARCHAR(255) NOT NULL,
  password_hash VARCHAR(255) NOT NULL,
  status ENUM('active', 'disabled') NOT NULL DEFAULT 'active',
  version BIGINT NOT NULL DEFAULT 0,
  created_at DATETIME(6) NOT NULL,
  updated_at DATETIME(6) NOT NULL,
  UNIQUE KEY uk_access_users_email (email)
) ENGINE=InnoDB CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

A tabela MySQL `access_users_outbox` deve registrar cada evento gerado pela escrita do usuario na mesma transacao:

```sql
CREATE TABLE access_users_outbox (
  id BINARY(16) PRIMARY KEY,
  aggregate_id BINARY(16) NOT NULL,
  event_type VARCHAR(100) NOT NULL,
  payload JSON NOT NULL,
  status ENUM('pending', 'published', 'failed') NOT NULL DEFAULT 'pending',
  attempts INT NOT NULL DEFAULT 0,
  available_at DATETIME(6) NOT NULL,
  created_at DATETIME(6) NOT NULL,
  published_at DATETIME(6) NULL,
  last_error VARCHAR(1000) NULL,
  KEY ix_access_users_outbox_status_available (status, available_at)
) ENGINE=InnoDB CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

Regras de dominio:

- O email e unico no conjunto de usuarios ativos e inativos e deve ser consultado em forma normalizada.
- O status `disabled` bloqueia login e desabilita operacoes de autenticacao, mas nao remove o registro.
- `PATCH` de usuario deve atualizar `updated_at` e incrementar `version` somente quando houver mudanca relevante.
- Operacoes de escrita devem rejeitar condicoes de concorrencia quando `version` informado pelo cliente divergir do registro em banco.

## Unidade Transacional De Usuario E Evento

A alteracao do aggregate `AccessUser` e o registro do evento na outbox devem acontecer dentro da mesma transacao InnoDB. O contrato de integracao deve ser:

```text
begin transaction
  insert/update access_users
  insert access_users_outbox
commit
```

Se qualquer etapa falhar, a transacao inteira deve ser revertida. Em termos de desenho da aplicacao:

- o command handler valida a entrada e cria o evento de dominio;
- o repository de escrita persiste a entidade e o registro da outbox na mesma unidade transacional;
- o processador da outbox somente publica eventos apos o commit bem-sucedido;
- a API nao publica eventos fora da transacao, nem grava eventos de dominio em handlers HTTP.

Isso garante consistencia entre estado e integraçao, sem permitir que o usuario seja alterado sem que um evento validado tenha sido registrado.

## Seguranca

- Nunca armazenar ou retornar senha em texto puro.
- Usar biblioteca de hash de senha revisada, com parametros configuraveis.
- Aplicar validacao de entrada e limites de tamanho.
- Evitar enumeracao de usuarios no login.
- Definir autenticacao e autorizacao para operacoes administrativas antes de liberar a API em producao.
- Definir politica de rate limiting e bloqueio de tentativas antes de expor o login publicamente.

## Decisoes Registradas

- Banco de dados e crate de persistencia: a decisao arquitetural permanece em `MySQL/InnoDB` com `utf8mb4` e repositorio dedicado em Rust, conforme ADR da infraestrutura.
- Formato da credencial: usar JWT de acesso stateless com `sub`, `role` e `exp`, assinado com `RS256`. O token de acesso expira em 15 minutos e um refresh token, quando existir, e armazenado no Redis com TTL para revogacao e invalidacao rapida.
- Regras de autorizacao: `admin` pode criar, alterar, listar e desabilitar qualquer usuario; `user` pode consultar o proprio perfil e atualizar apenas dados nao sensiveis e de propria conta; anonimos nao podem acessar endpoints de gestao. Toda operacao valida o papel do token em cada request.
- Algoritmo de hash: usar `Argon2id` com parametros configuraveis (memoria 64 MiB, time cost 3, parallelism 2), armazenando somente `password_hash` e nunca a senha em texto puro. A politica de rotacao de senha exige rehash ao detectar parametros antigos ou quando a senha for alterada.
- Estrategia de rate limiting e bloqueio de tentativas: aplicar limite por IP e por email para login, com backoff exponencial e bloqueio temporario em Redis; respostas do login devem continuar genericas para evitar enumeracao.

## Testes Obrigatorios

- Testes unitarios de validacao, normalizacao, senha, transicoes de status e regras de dominio.
- Testes de command handlers para duplicidade, concorrencia e atomicidade com a outbox.
- Testes de queries sem efeitos colaterais e sem exposicao de hash.
- Testes de login bem-sucedido, senha invalida, usuario inexistente e usuario desabilitado.
- Testes HTTP dos contratos, codigos de status e formato das respostas.
- Testes do processador da outbox para retry, idempotencia e falha de publicacao.
