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

## Organizacao Dos Testes Rust

Este projeto centraliza os testes no diretorio `tests/`, segmentados por responsabilidade. Nao deve existir um arquivo unico como `src/tests.rs` concentrando toda a suite.

Estrutura minima esperada:

```text
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

Em Rust, testes unitarios podem ficar proximos do modulo com `#[cfg(test)]`, enquanto testes de integracao ficam em `tests/` e exercitam a API publica do crate. Para este repositorio, o padrao operacional e manter tambem os testes unitarios em arquivos do diretorio `tests/`, usando testes de integracao por camada e evitando estado/testes monoliticos em `src/`. A implementacao pode usar modulos internos de teste quando precisar testar detalhes privados, mas o checklist e a cobertura principal devem permanecer segmentados em `tests/`.

Fluxo recomendado:

```text
spec -> plan -> tasks -> implementation -> tests -> update docs
```
