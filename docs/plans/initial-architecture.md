# Plano: Evolucao Do Backend

## Objetivo

Substituir os endpoints de demonstracao `hello` e `echo` pelas primeiras features reais do backend, iniciando pela gestao de usuarios de acesso e mantendo CQRS, Transactional Outbox, seguranca de credenciais e testes automatizados.

## Contratos E Modulos Previstos

Esta estrutura e obrigatoria para a implementacao, nao apenas uma sugestao de organizacao. Cada responsabilidade deve existir no modulo indicado antes de a etapa correspondente ser marcada como concluida. `src/app.rs` deve permanecer limitado a composicao do router e registro de rotas; ele nao pode conter entidades, value objects, regras de validacao, hash de senha, acesso a repositorios, armazenamento de estado ou orquestracao de commands.

A estrutura inicial do backend deve seguir a separacao abaixo, mantendo CQRS e a outbox transacional:

```text
src/
  domain/
    access_user.rs
    access_user/
      events.rs
      status.rs
      value_objects.rs
  application/
    commands/
      access_user.rs
      access_user/
        create_access_user.rs
        update_access_user.rs
        login_access_user.rs
    queries/
      access_user.rs
      access_user/
        get_access_user.rs
        list_access_users.rs
  repositories/
    access_user.rs
    access_user/
      access_user_write_repository.rs
      access_user_read_repository.rs
  outbox/
    outbox_record.rs
    processor.rs
    access_user.rs
    access_user/
      for_access_user_created.rs
      for_access_user_updated.rs
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
tests/
  domain/
    access_user.rs
    value_objects.rs
  application/
    commands.rs
    queries.rs
  repositories/
    access_user_write.rs
  outbox/
    processor.rs
  http/
    access_users.rs
  fixtures/
    mod.rs
```

### Contratos esperados

- `AccessUser` e `AccessUserStatus` ficam no modulo de dominio e encapsulam regras de email, nome, senha e transicoes de status.
- Commands aceitam apenas DTOs de entrada validos e retornam `Result<..., DomainError>` ou `Result<AccessUserId, ApplicationError>`.
- Queries retornam views publicas do read model, sem efeitos colaterais e sem acesso ao write model.
- Repositórios de escrita expõem `insert`, `update`, `find_by_id`, `find_by_email`, `save_event` e operam dentro de unidade transacional com outbox.
- A tabela `access_users_outbox` registra os eventos do aggregate com `id`, `aggregate_id`, `event_type`, `payload`, `status`, `attempts`, `available_at`, `created_at`, `published_at` e `last_error`, em uma transacao compartilhada com o usuario.
- Repositórios de leitura consultam o Elasticsearch e nunca fazem fallback para MySQL em consultas normais.
- Autenticacao e autorizacao sao camadas separadas: login valida senha e emissao de token; middleware/guard valida `sub`, `role` e escopos por request.

### Regra de implementacao

Para cada endpoint novo, a implementacao deve seguir o fluxo `http -> command -> dominio -> repositorio/outbox`, com contratos definidos em arquivos proprios. O handler HTTP somente desserializa a entrada, chama o command handler e converte o resultado em resposta HTTP. Uma implementacao nao pode ser aceita se regras de dominio ou persistencia estiverem concentradas em `app.rs`, `http/handlers.rs` ou em um unico arquivo monolitico.

Os testes tambem devem seguir a separacao modular. O diretorio `tests/` e obrigatorio para a suite principal, com arquivos separados para dominio, commands/queries, repositorios, outbox e HTTP. Um arquivo unico `src/tests.rs` nao atende ao plano. Testes unitarios internos com `#[cfg(test)]` sao permitidos quando necessarios para acessar detalhes privados, mas nao substituem os testes segmentados em `tests/`.

## Etapas

1. Fechar as decisoes em aberto da especificacao de gestao de usuarios, incluindo banco, credencial, hash, autorizacao e destino de eventos.
2. Escolher banco de dados e crate de acesso, documentando a decisao arquitetural.
3. Criar modulos separados para dominio, commands, queries, repositorios, autenticacao, outbox e handlers HTTP.
4. Criar o modelo da outbox e a unidade transacional que grava usuario e evento atomicamente.
5. Implementar criacao, alteracao e consultas de usuarios com MySQL/Elasticsearch reais e testes unitarios e de integracao segmentados em `tests/` por camada.
6. Implementar login com verificacao segura de senha, credencial de sessao/token e respostas que evitem enumeracao.
7. Implementar o processador da outbox com retry, backoff, idempotencia e observabilidade, com selecao de eventos pendentes, tentativa de publicacao no RabbitMQ, marcacao de erro e reprocessamento controlado.
8. Implementar o projetor RabbitMQ -> Elasticsearch para materializar o read model dos usuarios sem consultas ao MySQL em queries normais.
9. Implementar reindexacao de emergencia do Elasticsearch a partir do MySQL em batch, mantendo o caminho normal das queries somente no Elasticsearch.
10. Atualizar Docker Compose com banco e demais dependencias necessarias, aplicar migracoes SQL em bootstrap e validar os endpoints contra a infraestrutura real pelo perfil de testes.
11. Remover `hello` e `echo` do router e retirar seus testes somente apos os endpoints reais estarem cobertos.

Em cada etapa de implementacao, a revisao deve verificar a arvore de arquivos, os limites de dependencia entre modulos, a infraestrutura efetivamente usada pelo runtime e a existencia de testes da camada alterada no diretorio `tests/`. A tarefa so pode ser marcada como concluida quando essa verificacao passar; mocks em memoria nao substituem MySQL, RabbitMQ ou Elasticsearch nos fluxos de integracao.

## Nao Escopo

- Nao manter `hello` e `echo` como parte do contrato final da API.
- Nao implementar autenticacao administrativa sem definir autorizacao minima.
- Nao escolher banco, formato de credencial ou broker sem uma decisao registrada.
- Nao publicar eventos diretamente a partir de handlers HTTP.

## Criterio De Saida

A primeira feature deve permitir criar, alterar, consultar e autenticar usuarios de acesso, demonstrar que a alteracao de estado e o evento da outbox sao confirmados juntos e remover os endpoints de demonstracao sem reduzir a cobertura de testes. O backend deve permanecer preparado para novas features de negocio.

O criterio de saida inclui a estrutura modular prevista: dominio, commands, queries, repositorios, outbox, autenticacao e HTTP devem estar separados em modulos proprios, com `app.rs` contendo somente a montagem do router.

Tambem inclui uma suite de testes segmentada em `tests/`, sem concentrar os testes em `src/tests.rs` ou em um arquivo monolitico.
