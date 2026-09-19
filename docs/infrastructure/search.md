# Infraestrutura: Elasticsearch

## Status

Aprovada como read model das queries.

## Responsabilidade

Elasticsearch sera o mecanismo exclusivo de consulta da API de usuarios de acesso, inclusive consultas por identificador. MySQL continua sendo o write model e a fonte de verdade dos commands; Elasticsearch e uma projecao derivada e reconstruivel.

## Fluxo

```text
Command -> MySQL + Outbox -> RabbitMQ -> Elasticsearch index
Query   -> Elasticsearch
```

O consumidor deve projetar os eventos da outbox no indice correspondente depois que o commit do MySQL for concluido. A entrega e a projecao devem ser idempotentes usando o identificador do evento e a versao do agregado.

## Regras

- Queries nao podem consultar MySQL como fallback.
- Consultas por `id`, filtros, paginacao e busca textual devem usar Elasticsearch.
- Nao indexar `password_hash`, senhas, tokens ou segredos.
- Definir mappings, aliases, politica de versionamento e estrategia de reindexacao.
- Configurar replicas, refresh interval, timeouts e limites de paginação.
- Aceitar consistencia eventual entre uma escrita no MySQL e sua disponibilidade no indice.
- Reconstruir o indice a partir do MySQL por processo operacional, sem transformar essa reconstrução em fallback de query.

## Comportamento De Falhas

Se Elasticsearch estiver indisponivel, as queries devem retornar erro observavel de read model indisponivel. O sistema nao deve consultar MySQL silenciosamente para mascarar a falha.

## Testes

- Projecao de eventos de criacao, alteracao e desativacao.
- Projecao idempotente e ordenacao por versao.
- Consulta por id, filtros, paginacao e busca textual.
- Falha do Elasticsearch sem fallback para MySQL.
- Reindexacao e recuperação após perda do indice.
