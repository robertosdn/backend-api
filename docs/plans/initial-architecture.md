# Plano: Evolucao Do Backend

## Objetivo

Substituir os endpoints de demonstracao `hello` e `echo` pelas primeiras features reais do backend, iniciando pela gestao de usuarios de acesso e mantendo CQRS, Transactional Outbox, seguranca de credenciais e testes automatizados.

## Contratos E Modulos Previstos

A estrutura inicial do backend deve seguir a separacao abaixo, mantendo CQRS e a outbox transacional:

```text
src/
  domain/
    access_user.rs
    access_user_status.rs
    events.rs
    value_objects.rs
  application/
    commands/
      create_access_user.rs
      update_access_user.rs
      login_access_user.rs
    queries/
      get_access_user.rs
      list_access_users.rs
  repositories/
    access_user_write_repository.rs
    access_user_read_repository.rs
  outbox/
    outbox_record.rs
    processor.rs
  auth/
    jwt.rs
    password_hash.rs
    authorization.rs
  http/
    routes.rs
    handlers.rs
    dto.rs
  infrastructure/
    mysql/
    elasticsearch/
    rabbitmq/
    redis/
```

### Contratos esperados

- `AccessUser` e `AccessUserStatus` ficam no modulo de dominio e encapsulam regras de email, nome, senha e transicoes de status.
- Commands aceitam apenas DTOs de entrada validos e retornam `Result<..., DomainError>` ou `Result<AccessUserId, ApplicationError>`.
- Queries retornam views publicas do read model, sem efeitos colaterais e sem acesso ao write model.
- Repositórios de escrita expõem `insert`, `update`, `find_by_id`, `find_by_email`, `save_event` e operam dentro de unidade transacional com outbox.
- A tabela `access_users_outbox` registra os eventos do aggregate com `id`, `aggregate_id`, `event_type`, `payload`, `status`, `attempts`, `available_at`, `created_at`, `published_at` e `last_error`, em uma transacao compartilhada com o usuario.
- Repositórios de leitura consultam o Elasticsearch e nunca fazem fallback para MySQL em consultas normais.
- Autenticacao e autorizacao sao camadas separadas: login valida senha e emissao de token; middleware/guard valida `sub`, `role` e escopos por request.

## Etapas

1. Fechar as decisoes em aberto da especificacao de gestao de usuarios, incluindo banco, credencial, hash, autorizacao e destino de eventos.
2. Escolher banco de dados e crate de acesso, documentando a decisao arquitetural.
3. Criar modulos separados para dominio, commands, queries, repositorios, autenticacao, outbox e handlers HTTP.
4. Criar o modelo da outbox e a unidade transacional que grava usuario e evento atomicamente.
5. Implementar criacao, alteracao e consultas de usuarios com testes unitarios e de integracao.
6. Implementar login com verificacao segura de senha, credencial de sessao/token e respostas que evitem enumeracao.
7. Implementar o processador da outbox com retry, backoff, idempotencia e observabilidade, com selecao de eventos pendentes, tentativa de publicacao no RabbitMQ, marcacao de erro e reprocessamento controlado.
8. Implementar o projetor RabbitMQ -> Elasticsearch para materializar o read model dos usuarios sem consultas ao MySQL em queries normais.
9. Implementar reindexacao de emergencia do Elasticsearch a partir do MySQL em batch, mantendo o caminho normal das queries somente no Elasticsearch.
10. Atualizar Docker Compose com banco e demais dependencias necessarias, aplicar migracoes SQL em bootstrap e validar pelo perfil de testes.
11. Remover `hello` e `echo` do router e retirar seus testes somente apos os endpoints reais estarem cobertos.

## Nao Escopo

- Nao manter `hello` e `echo` como parte do contrato final da API.
- Nao implementar autenticacao administrativa sem definir autorizacao minima.
- Nao escolher banco, formato de credencial ou broker sem uma decisao registrada.
- Nao publicar eventos diretamente a partir de handlers HTTP.

## Criterio De Saida

A primeira feature deve permitir criar, alterar, consultar e autenticar usuarios de acesso, demonstrar que a alteracao de estado e o evento da outbox sao confirmados juntos e remover os endpoints de demonstracao sem reduzir a cobertura de testes. O backend deve permanecer preparado para novas features de negocio.
