# Modificações do CAD em relação ao CADCraft

O CAD é uma versão modificada do [CADCraft](https://github.com/storytold/cadcraft) (versão
0.7.0, commit "Release 0.7.0"), feita para a Suíte FisiLab pelo FisiLab — LARFIC/UFRGS em parceria
com o CTA — Centro de Tecnologia Acadêmica. As modificações são © 2026 FisiLab — LARFIC/UFRGS e
seguem os mesmos termos do original (MIT OR Apache-2.0).

## O que mudou e por quê

### Nome e marca
- O nome exibido ao usuário passou de **CADCraft** para **CAD**: título da janela, nome do app
  no `eframe`, menus ("Sobre o CAD", "Ajuda do CAD", "Sair do CAD"), página inicial,
  diálogos, mensagens de status e de erro (motor, leitura de DXF/DWG, servidor MCP), `--help` e
  `--version` da linha de comando e os catálogos de tradução (`crates/ui-egui/src/i18n/*.tsv`,
  chaves e valores).
- Onde "CAD" sozinho seria lido como o termo genérico e não como o programa, o texto diz
  **FisiLab CAD**: mensagens de erro e de aviso (motor, leitura de DXF/DWG, estilos de plotagem e
  plotters), o servidor MCP (título, instruções, descrição das ferramentas e erros de conexão) e os
  campos `/Producer` e `/Creator` dos PDFs exportados.
- O nome e as logos da ArtCraft são marcas que não fazem parte da licença livre. Por isso a pasta
  `docs/brand/` foi apagada e o app não mostra mais nenhuma logo ou marca da ArtCraft. O crédito em
  texto ("baseado no CADCraft, da equipe ArtCraft e contribuidores") continua, como manda a licença.
- O empacotamento Linux (AppImage e .deb) usa os nomes, ícones e ids da suíte (`FisiLab-CAD-<versão>-x86_64.AppImage`, `fisilab-cad_<versão>_amd64.deb`, id `br.ufrgs.fisilab.cad`).

### Identidade da Suíte FisiLab
- Novo crate `crates/fisilab-brand` (cópia do crate compartilhado da suíte): paleta de cores,
  animação de abertura com a logo do CTA e a janela "Sobre a Suíte FisiLab".
- `crates/ui-egui/src/lib.rs`: `APP_INFO` (nome, descrição, equivalente proprietário e créditos
  ao projeto de origem); `CadApp::ui` chama `fisilab_brand::frame` em todo quadro.
- `apps/cadcraft/src/main.rs`: `fisilab_brand::splash::enable()` antes de abrir a janela.
- Menu **Ajuda ▸ Sobre a Suíte FisiLab…** (comando `ui.fisilab.about`), também no menu do app no
  macOS. A janela "Sobre o CAD" original continua (com os contribuidores do CADCraft) e ganhou
  os créditos da suíte e os links abaixo.

### Temas "Noite" e "Dia"
- Paleta original do CADCraft, sem cores novas. `crates/ui-egui/src/theme.rs` apenas ganhou os
  nomes: **Noite** (o tema escuro original, padrão) e **Dia** (o tema claro original), primeiros
  da lista; "Usar configuração do sistema" continua disponível. O interpretador de nomes aceita
  também `noite`/`night` e `dia`/`day`; os ids salvos continuam `dark` e `light`.
- Idioma: o CAD abre em português do Brasil na primeira execução (`FIRST_RUN_LANGUAGE`, aplicado
  pelo app desktop só quando não há preferências salvas); o padrão da biblioteca é o original.

### Links da comunidade
- Removidos os links para o Discord da ArtCraft e para getartcraft.com (janela Sobre, menu do macOS,
  instalador MSI, metadados de pacotes Linux). No lugar: "Site da Suíte FisiLab", "Manifesto pelas
  tecnologias livres", "Projeto original (CADCraft)" e "Reportar um problema"
  (https://github.com/mrcandrade/fisilab-cad/issues), além do botão "Sobre a Suíte FisiLab…".
- O CADCraft não faz verificação de atualizações, telemetria, envio de falhas nem downloads de
  servidores da ArtCraft; não houve nada a desativar além dos links acima.

### Idioma
- Numa instalação nova (sem preferências salvas) o CAD abre em **português do Brasil**
  (`FIRST_RUN_LANGUAGE`). A escolha do usuário, inclusive "Automático", é salva e respeitada.

### Executáveis, identificador e pastas
- Executáveis: `fisilab-cad.exe` (programa) e `fisilab-cad-cli.exe` (linha de comando), por `[[bin]]`
  explícitos; os pacotes Rust mantêm os nomes `cadcraft` e `cadcraft-cli`.
- Identificador do app: `br.ufrgs.fisilab.cad` (era `ai.storyteller.cadcraft`).
- Pastas de configuração, logs, autosave e plotters: `%APPDATA%\FisiLab CAD`,
  `~/Library/Application Support/FisiLab CAD`, `~/.config/fisilab-cad`, para não misturar com uma instalação
  do CADCraft.
- Recursos do Windows (`apps/cadcraft/build.rs`): ícone `assets/fisilab/fisilab-cad.ico`, nome,
  descrição, empresa e copyright do CAD. Ícone da janela: `assets/fisilab/fisilab-cad-256.png`.
- `packaging/windows/*`: nomes, GUIDs novos (não substituem uma instalação do CADCraft), ProgId
  `FisiLab.CAD.Drawing`, links e ícone atualizados. O MSI instala em
  `Program Files\Suite FisiLab\CAD` e guarda a escolha do atalho na área de trabalho em
  `HKCU\Software\FisiLab\CAD`.
- Logs: o executável se chama `fisilab-cad`, então os módulos dele registram com o alvo
  `fisilab_cad`; o filtro padrão (`apps/cadcraft/src/logging.rs`) é
  `warn,cadcraft*=info,fisilab_cad=info`.
- `fisilab-cad-cli mcp --connect … --launch` no macOS abre o app pelo identificador do pacote
  (`open -b br.ufrgs.fisilab.cad`).

### Outros
- `.github/` apagado (os fluxos de CI e de release do original rodariam contra este fork).
- `README.md` novo em português; o original está em `README.upstream.md`.
- `NOTICE` ganhou um cabeçalho do CAD; o texto original segue abaixo dele, sem alterações.

## O que não mudou
- Identificadores internos: nomes de crates e módulos Rust, ids de comandos, chaves de preferências,
  variáveis de ambiente `CADCRAFT_*` e nomes de arquivos de log.
- Formatos e nomes gravados em arquivos, por compatibilidade: a fonte "CADCraft Stroke", os
  dispositivos de plotagem "CADCraft PDF/PNG/SVG" e os marcadores dos arquivos de estilos de
  plotagem e de plotters.
- Comentários, documentação técnica em `docs/`, histórico, `CLAUDE.md`/`AGENTS.md`, `ROADMAP.md`.
- `LICENSE-MIT`, `LICENSE-APACHE` e `ATTRIBUTION.md` (este ainda lista os arquivos de
  `docs/brand/`, que foram apagados; nenhum material de terceiros foi adicionado).
- O ícone original do CADCraft em `assets/app-icon/` (usado só pelos scripts de pacote de macOS e
  Linux, que não foram adaptados) e o app web (`apps/cadcraft-web`), que só teve o nome trocado.
- Todo o resto do comportamento do CADCraft.

## Segurança e privacidade
- Xrefs com caminho de rede (UNC `\\host\share`, `//host/share`, `\\?\`, `\\.\`, `\??\`) ou URL (`file://host/...`, `http:`) vindos do desenho nunca são abertos: o xref aparece como não encontrado, com mensagem explicando. Isso evita que o Windows abra uma conexão SMB ao carregar o desenho e vaze o hash NTLM e o IP do usuário. Caminhos locais (absolutos e relativos) seguem funcionando. Fontes e imagens não seguem caminhos do desenho (fontes são buscadas só nas pastas do sistema).
- `fisilab-cad-cli mcp --launch` no macOS agora abre o app por `br.ufrgs.fisilab.cad` (antes `ai.storyteller.cadcraft`).
