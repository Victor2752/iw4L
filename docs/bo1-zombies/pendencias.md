# Pendências, em ordem

1. **Restos dos menus** ([menus.md](menus.md)):
   - pausar o jogo solo com o menu aberto (`CL_PauseGame`), o que pede uma
     pausa no relógio da autoridade que não "alcance" os ticks perdidos;
   - animações de item (`ui_animate`, `UIAnimInfo`) e blur;
   - esconder a retícula sob o menu;
   - layout de texto do BO1 (`t5.hud.menu_layout`).
2. **Portas girando**: uma brush fora da pose autorada não é desenhada (a
   porta some, a tábua arrancada some). Plano: recalcular os vértices na pose
   atual enquanto a peça se mexe e regravar esse trecho no buffer do mundo na
   GPU, reaproveitando `bake_t5_script_brush_poses`. Conferir junto que a
   colisão acompanha (`RotateTo`, `NotSolid`).
3. **Vision set** (imagem "apagada"): `assets/src/lane/t5.rs` descarta as
   `.vision`. A cadeia do BO1 (`rb_postfx.cpp::RB_BloomLDR`):
   - bloom: `bloom_downsample_ldr` → `resampleFinal` → `bloom_downsample_convolution`
     → `bloom_remap` → blur X/Y → streak → `bloomSmoothA`;
   - `bloom_apply_ldr` (curva Hable, W=8);
   - `postfx_color` (`POSTFX_CONTROL0..A`: matrizes de temperatura, matiz e
     saturação, contraste, bleach e as bandas mid).

   O IW4L já traduz os shaders nativos de pós do T5; falta montar a cadeia e
   as constantes.
4. **Movimento do BO1**: portar `bg_pmove.cpp` (sprint, dive to prone,
   velocidades, mantle) e os passos (`fly_step_*`). É o maior item; fatiar.
5. **Máquina de armas do BO1** (`bg_weapons.cpp`): troca, ADS, melee,
   offhand. Os perks que faltam (troca rápida, ADS) entram aqui.
6. **Last stand do BO1** (`game_sp/g_sp_client.cpp`): hoje usa o pm type e a
   altura de visão do MW2.
7. **Client scripts (CSC)**: nenhum roda hoje. Isso tira o brilho dos olhos
   dos zumbis, os efeitos de tábuas e risers, a música
   (`clientsyssetstate`), as luzes da energia e os efeitos do teleporte.
8. **Sons que faltam**: `uin_transition_zombie_theater` (tela de loading) e os
   passos `fly_step_*`/`fly_gear_*`.

Cada item, ao fechar, sai daqui, entra em [`implementado.md`](implementado.md)
e fecha a entrada no `docs/fidelity/t5.md`.
