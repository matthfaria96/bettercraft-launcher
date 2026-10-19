<p align="center">
  <img src="../docs/bettercraft/logo.png" alt="BetterCraft Launcher" width="128">
</p>

<h1 align="center">BetterCraft Launcher</h1>

<p align="center"><a href="README.md">English</a> · <b>Português</b></p>

<p align="center">
  <b>O launcher do Craft App Suite repensado: sem ruído visual, direto ao ponto e com ícones melhores.</b><br>
  Uma versão melhorada da experiência do
  <a href="https://github.com/storytold/craft-launcher">ArtCraft Launcher</a>, sempre em dia com ele.
</p>

<p align="center">
  <img src="../docs/bettercraft/screenshot.png" alt="BetterCraft Launcher no macOS: a lista dos Craft Apps instalados, cada um com o ícone original ao lado do redesenhado, a versão e o botão Abrir" width="760">
</p>

## O que é

O BetterCraft Launcher instala, atualiza e abre os apps do Craft App Suite (PhotoCraft,
VectorCraft, FilmCraft, LightCraft, PdfCraft, EffectCraft, DesignCraft, WordCraft, DeckCraft,
GridCraft, CADCraft, SoundCraft e FigCraft), igual ao original. O que muda é a experiência:
fica só o que importa.

## O que melhorou

- **Sem ruído visual.** Uma lista compacta, uma linha por app: ícone, nome, versão e um botão.
  Saíram páginas de detalhes, notas de versão, avisos, links sociais, busca, ordenação e a troca
  entre grade e tabela.
- **Direto ao ponto.** A barra tem só duas coisas: *Quais ícones usar?* e *Atualizar*. O menu de
  cada app tem só Abrir, Mostrar no Finder, Reinstalar e Desinstalar.
- **Ícones redesenhados.** Com *Redesign*, cada Craft App instalado ganha um ícone novo, mais
  claro e consistente, no Finder, no Launchpad e no Dock, inclusive com o app aberto. *Originais
  Craft* devolve os ícones de fábrica. Cada linha mostra qual dos dois o app está usando.
- **Atualizações sem surpresa.** A versão é lida do próprio app, então as atualizações funcionam
  com qualquer ícone. O launcher nunca mexe num app enquanto ele atualiza e aplica o ícone na
  versão nova logo em seguida.
- **Configurações enxutas.** Atualizações, aparência e o estilo dos ícones. Nada além disso.
- **Sempre em dia com o original.** Um workflow traz as mudanças do
  [storytold/craft-launcher](https://github.com/storytold/craft-launcher) a cada 6 horas. Todo o
  código novo fica em `crates/ui-egui/src/bettercraft/`, com ganchos de uma linha no resto.

## Modo Estudo (não oficial)

Nas Configurações, o cartão *Estudo* troca os ícones e os nomes dos apps pelos dos programas que
eles imitam (PhotoCraft → Adobe Photoshop, WordCraft → Microsoft Word…), com uma fita amarela
CRAFT no canto do ícone. É um modo de teste, não oficial. Os ícones e nomes desses programas são
marcas dos seus donos.

## Instalar no macOS

Precisa do [Rust](https://rustup.rs) e das ferramentas de linha de comando do Xcode.

```sh
git clone https://github.com/matthfaria96/bettercraft-launcher.git
cd bettercraft-launcher
scripts/install-mac.sh
```

O script compila o app, instala em `/Applications/BetterCraft Launcher.app` e abre. Para pegar as
novidades do original e reinstalar:

```sh
scripts/sync-upstream.sh && scripts/install-mac.sh
```

Detalhes técnicos em [BETTERCRAFT.md](../BETTERCRAFT.md).

## Licença

Igual ao original: MIT ou Apache-2.0. O nome e a marca ArtCraft pertencem aos seus donos
(`docs/brand/LICENSE-brand.txt`).
