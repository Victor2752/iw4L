# Black Ops 1 Zombies no IW4L — documentação do fork

Notas do trabalho para jogar **Kino der Toten** (`t5:zombie_theater`) no IW4L
com os arquivos originais do Black Ops 1. Escritas para quem continuar o
trabalho (pessoa ou agente) sem o histórico das conversas.

O resto de `docs/` é do projeto original (em inglês, um assunto por arquivo;
o mapa é [`../INDEX.md`](../INDEX.md)). Estes arquivos só cobrem o que este
fork mudou e como ele foi pensado.

| arquivo | assunto |
|---|---|
| [`arquitetura.md`](arquitetura.md) | camadas do repo, a fronteira de jogo, onde cada regra do BO1 mora e o que ainda é emprestado do MW2 |
| [`decompilacao.md`](decompilacao.md) | a decompilação do BO1 usada como fonte das regras: onde está, como ler, o que nunca trazer para o código |
| [`implementado.md`](implementado.md) | cada coisa que o fork já fez: a regra do BO1, os arquivos e o commit |
| [`menus.md`](menus.md) | como o motor de menus do BO1 funciona e o estado do menu de pausa |
| [`testes.md`](testes.md) | build, rodar a Kino por script, logs, screenshots, dumps e as checagens antes de commitar |
| [`pendencias.md`](pendencias.md) | o que falta, em ordem de prioridade |

## Estado em uma linha

Dá para jogar a Kino solo: andar, atirar, comprar armas, portas e perks
(Quick Revive, Speed Cola, Double Tap, Juggernog), zumbis entram pelas
janelas, morrem com headshot, HUD de pontos e rounds, last stand, e o ESC abre
o menu de pausa e as opções do próprio BO1 (com FPS e sensibilidade numérica
adicionados pelo IW4L). O menu pausa o jogo no solo. Faltam o visual (vision set,
olhos, portas girando), o movimento e as armas do BO1, a música e parte dos sons.

## Onde mais olhar

- [`../fidelity/t5.md`](../fidelity/t5.md): o **ledger** do BO1, com cada regra
  que falta (por id `t5.*`) e o que já foi fechado. Toda lacuna nova entra lá.
- [`../../BLACKOPS_TODO.md`](../../BLACKOPS_TODO.md): o roadmap original do zombies BO1.
- [`../RUN.md`](../RUN.md): verbos do console (`spawn`, `move`, `look`, `press`, `damage`…).
- [`../ARCHITECTURE.md`](../ARCHITECTURE.md): a fronteira de jogo, na versão do projeto original.

## Git

- Branch de trabalho: **`kino`**. Ela veio de `seba/main` (o fork
  `seba2390/iw4L`, que trouxe o zombies T5) e rastreia esse remoto.
- Para subir: `git push -u origin kino` (`origin` = `Victor2752/iw4L`).
  Nunca dê push no `seba` e não commite no `master`.
