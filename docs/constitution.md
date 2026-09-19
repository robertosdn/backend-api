# Constituicao do Projeto

## Objetivo

Manter uma API backend completa em Rust, testavel e preparada para evoluir com multiplas features sem perder clareza arquitetural.

## Principios Obrigatorios

1. **Rust idiomatico**: preferir tipos fortes, erros explicitos e responsabilidades pequenas.
2. **CQRS**: separar operacoes que alteram estado (commands) de operacoes que apenas consultam estado (queries).
3. **Transactional Outbox**: quando uma operacao alterar estado e gerar evento de integracao, persistir ambos na mesma transacao atomica antes de publicar o evento.
4. **Idempotencia**: comandos e o processamento da outbox devem tolerar retries sem duplicar efeitos.
5. **Testes**: toda funcionalidade nova ou alterada deve ter testes unitarios; fluxos HTTP relevantes devem ter testes de integracao.
6. **Docker**: compilar, testar e executar a aplicacao preferencialmente por Docker Compose ou pelas etapas do Dockerfile.
7. **Mudancas pequenas**: evitar refatoracoes fora do escopo e preservar contratos existentes.

## Definition Of Done

Uma mudanca so esta concluida quando:

- a especificacao correspondente foi atualizada;
- commands, queries e eventos estao separados conforme aplicavel;
- os testes unitarios e de integracao relevantes foram adicionados ou atualizados;
- `cargo fmt --check`, `cargo test` e `cargo clippy` foram executados no Docker quando suportados pelo ambiente;
- riscos, limitacoes e comandos executados foram registrados no resumo da mudanca.
