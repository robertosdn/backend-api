# Constituicao do Projeto

## Objetivo

Manter uma API backend completa em Rust, testavel e preparada para evoluir com multiplas features sem perder clareza arquitetural.

## Principios Obrigatorios

1. **Rust idiomatico**: preferir tipos fortes, erros explicitos e responsabilidades pequenas.
2. **CQRS**: separar operacoes que alteram estado (commands) de operacoes que apenas consultam estado (queries).
3. **Transactional Outbox**: quando uma operacao alterar estado e gerar evento de integracao, persistir ambos na mesma transacao atomica antes de publicar o evento.
4. **Idempotencia**: comandos e o processamento da outbox devem tolerar retries sem duplicar efeitos.
5. **Testes segmentados**: toda funcionalidade nova ou alterada deve ter testes unitarios e, quando aplicavel, testes de integracao em arquivos separados dentro de `tests/`; nao concentrar a suite em `src/tests.rs`.
6. **Docker**: compilar, testar e executar a aplicacao preferencialmente por Docker Compose ou pelas etapas do Dockerfile.
7. **Mudancas pequenas**: evitar refatoracoes fora do escopo e preservar contratos existentes.
8. **Modularidade verificavel**: responsabilidades devem estar em modulos e arquivos proprios conforme o plano; `app.rs` e ponto de composicao, nao local para regras de negocio ou infraestrutura.

## Definition Of Done

Uma mudanca so esta concluida quando:

- a especificacao correspondente foi atualizada;
- commands, queries e eventos estao separados conforme aplicavel;
- a estrutura de modulos prevista no plano existe e `app.rs` contem somente composicao do router;
- os testes unitarios e de integracao relevantes foram adicionados ou atualizados;
- os testes estao segmentados por camada no diretorio `tests/`, sem arquivo monolitico de testes;
- `cargo fmt --check`, `cargo test` e `cargo clippy` foram executados no Docker quando suportados pelo ambiente;
- riscos, limitacoes e comandos executados foram registrados no resumo da mudanca.
