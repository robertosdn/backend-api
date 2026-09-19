# Documentacao SDD

Esta pasta usa Specification-Driven Development (SDD).

- `constitution.md`: principios e criterios obrigatorios.
- `architecture.md`: estrutura tecnica e regras de CQRS e Transactional Outbox.
- `infrastructure/`: especificacoes operacionais de banco, fila, cache e observabilidade.
- `infrastructure/search.md`: especificacao do Elasticsearch como read model.
- `decisions/`: ADRs com escolhas tecnicas e suas consequencias.
- `specs/`: comportamento esperado de cada funcionalidade.
- `plans/`: estrategia para implementar uma funcionalidade ou evolucao.
- `tasks/`: checklist executavel derivado do plano.

Fluxo recomendado:

```text
spec -> plan -> tasks -> implementation -> tests -> update docs
```
