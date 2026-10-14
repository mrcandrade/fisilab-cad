<p align="center">
  <img src="assets/fisilab/fisicad-256.png" alt="Ícone do FisiCAD" width="128">
</p>

<h1 align="center">FisiCAD</h1>

<p align="center">
  <b>Desenho técnico e CAD</b><br>
  Alternativa livre ao <b>Autodesk AutoCAD</b>
</p>

<p align="center">
  Disponibilizado pelo <b>FisiLab — LARFIC · UFRGS</b> em parceria com o <b>CTA — Centro de
  Tecnologia Acadêmica</b>, para estudantes, professores e técnicos da UFRGS.
</p>

---

O FisiCAD faz parte da [Suíte FisiLab](https://github.com/mrcandrade/larfic/tree/main/suite-fisilab):
programas livres para ensino, pesquisa e criação. Ele é uma versão modificada do
[CADCraft](https://github.com/storytold/cadcraft), uma reimplementação aberta e escrita em Rust do
fluxo de trabalho do AutoCAD.

## O que faz

- **O fluxo de trabalho que você já conhece:** linha de comando com prompts e `[palavras-chave]`
  clicáveis, autocompletar, apelidos (`L`, `C`, `@5<45`), referência a objetos (snaps), rastreamento
  polar, ortogonal, seleção por janela e por cruzamento e grips.
- **Desenho e modificação:** LINE, PLINE, CIRCLE, ARC, RECTANG, POLYGON, ELLIPSE, SPLINE, TEXT,
  MTEXT; MOVE, COPY, ROTATE, SCALE, MIRROR, OFFSET, TRIM, EXTEND, FILLET, CHAMFER, ARRAY e mais.
- **Camadas e propriedades:** paleta de camadas e gerenciador de propriedades de camadas, tipos e
  espessuras de linha, cores indexadas e true color, paleta de propriedades por objeto.
- **Anotação:** todas as cotas DIM* com estilos de cota, cotas associativas, MLEADER, TABLE,
  fontes TrueType e hachuras (por ponto interno, com ilhas, sólidas e em gradiente).
- **Blocos, layouts e plotagem:** BLOCK/INSERT com atributos, espaço de papel com viewports,
  configurações de página, PLOT e exportação para PDF, SVG e PNG.
- **Arquivos DXF e DWG:** DXF lido (R12–2018, ASCII e binário) e gravado nativamente; DWG aberto e
  salvo por meio da biblioteca livre acadrust.
- **Paramétrico:** restrições geométricas e dimensionais, AUTOCONSTRAIN e PARAMETERS com
  expressões.
- **Automação:** servidor MCP, canal de controle JSON e a ferramenta de linha de comando
  `fisicad-cli` (informações, conversão, scripts, exportação).

## Instalação

### Windows

Baixe o instalador `FisiCAD-Setup-<versão>.exe` no portal de downloads da Suíte FisiLab ou na
página de [Releases do mrcandrade/fisicad](https://github.com/mrcandrade/fisicad/releases) e
execute-o. O FisiCAD abre em português do Brasil e com o tema **UFRGS Noite**; o idioma e o tema
(**UFRGS Dia**, Claro, Escuro ou o do sistema) podem ser trocados no menu e nas Opções.

### Compilar a partir do código

É preciso ter o [Rust](https://rustup.rs) 1.95 ou mais recente.

```sh
git clone https://github.com/mrcandrade/fisicad
cd fisicad
cargo run --release -p cadcraft -- --sample   # abre o FisiCAD com um desenho de exemplo
cargo build --release -p cadcraft -p cadcraft-cli
```

Os executáveis ficam em `target/release/fisicad` (programa) e `target/release/fisicad-cli`
(linha de comando). Os pacotes Rust mantêm os nomes originais (`cadcraft`, `cadcraft-cli`, ...).

## Créditos

O FisiCAD é baseado no [CADCraft](https://github.com/storytold/cadcraft), criado pela equipe
ArtCraft e pelos contribuidores do CADCraft. O código é distribuído sob a licença MIT ou a
Apache-2.0, à sua escolha ([LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE)). Veja
[NOTICE](NOTICE), [ATTRIBUTION.md](ATTRIBUTION.md), [MODIFICACOES.md](MODIFICACOES.md) (o que mudou
nesta versão) e o [README original](README.upstream.md).

Os nomes e as marcas do projeto original pertencem aos seus donos. Esta versão não é afiliada ao
projeto original nem endossada por ele. AutoCAD e DWG são marcas da Autodesk, Inc.; o FisiCAD não
tem relação com a Autodesk.

## Manifesto

Conhecimento produzido em uma universidade pública deve circular livremente, e as ferramentas
usadas para ensinar, pesquisar e criar também: leia o
[Manifesto pelas tecnologias livres](https://github.com/mrcandrade/larfic/blob/main/suite-fisilab/MANIFESTO.md).

---

<p align="center">
  <img src="assets/fisilab/ufrgs-logo.gif" alt="UFRGS" height="48">
  &nbsp;&nbsp;&nbsp;
  <img src="assets/fisilab/cta-logo.webp" alt="CTA — Centro de Tecnologia Acadêmica" height="48">
  &nbsp;&nbsp;&nbsp;
  <img src="assets/fisilab/larfic-logo.png" alt="LARFIC" height="48">
</p>
