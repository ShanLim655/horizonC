# Horizon C — Offline-First Plan

## Goal

Make Horizon C usable without any Microsoft/Xbox account integration. Offline profiles must launch using game files the user already has from a legally obtained Minecraft: Java Edition installation. Do not add authentication bypasses for online-mode servers or unauthorized game-file downloads.

## Status

- [x] Planning
- [x] Audit the repository for Microsoft/Xbox sign-in, OAuth/Azure configuration, account/session/token handling, Microsoft-specific UI, endpoints, tests, and documentation
- [x] Replace account management with local offline profiles only
- [x] Remove Microsoft/Xbox authentication and account-related dependencies and configuration
- [x] Ensure game preparation uses locally supplied, legally obtained files only; remove Microsoft-account-gated downloads
- [x] Add a first-launch disclaimer with an unchecked acknowledgement and Exit / Accept & Continue actions
- [x] Update documentation, settings, CLI commands, and tests to match offline-only behavior
- [ ] Run formatting, tests, and available CI checks; fix regressions

## Requirements

1. No Microsoft or Xbox sign-in, OAuth, Azure app registration, account linking, token storage, or Microsoft-account UI remains in Horizon C.
2. Profiles are local offline usernames with appropriate validation and persistent selection.
3. The launcher must not download proprietary Minecraft game files from Microsoft/Mojang services. Users must import files from their own legally obtained installation.
4. Offline play and supported offline-mode/LAN multiplayer must continue to work. Do not claim offline profiles can join authenticated online-mode servers.
5. Before the first launch, show this disclaimer and require an unchecked acknowledgement:

   **Before You Play**

   Horizon C is an unofficial, open-source project and is not affiliated with or endorsed by Mojang or Microsoft. We encourage you to purchase an official copy of Minecraft: Java Edition to support its developers. Horizon C does not provide a Minecraft license; use game files from a legally obtained copy.

   Servers that require authenticated Minecraft accounts generally will not accept offline profiles. Offline-mode servers and LAN play may work depending on their configuration.

   By selecting Accept & Continue, you confirm that you understand these limitations and wish to continue.

   Buttons: **Exit** and **Accept & Continue**

6. Persist acknowledgement so it is not repeatedly shown, and make the disclaimer accessible from the launcher’s About or Settings page.
7. Keep unrelated game and launcher functionality intact. Do not leave dead UI, placeholder implementations, unused authentication dependencies, or misleading docs.
8. Keep existing license and attribution notices intact.

## Implementation Phases

### Phase 1 — Audit
Map every Microsoft/Xbox auth flow and every dependency on authenticated account/session data across the launcher, client, CLI, build configuration, tests, and docs. Identify the minimum changes needed before editing.

### Phase 2 — Offline-only account model
Implement local profile selection and launch arguments/session data without Microsoft tokens. Ensure no account-related network calls are made.

### Phase 3 — Local game-file preparation
Keep import/preparation from a user's existing legal installation. Remove Microsoft-account-gated downloads and any code that fetches proprietary game files from Microsoft/Mojang endpoints.

### Phase 4 — Disclaimer and docs
Add first-launch consent UI, persistence, an accessible disclaimer entry, and update README/help/CLI documentation.

### Phase 5 — Verification
Add or update tests for offline profile selection, launch without network access, disclaimer acceptance persistence, rejection/exit behavior, and absence of Microsoft authentication calls. Run formatting, all relevant tests, and CI checks.

## Exit Criteria

- No Microsoft/Xbox authentication or account-management flow remains in source, UI, configuration, or documentation.
- A local offline profile can launch with imported local game files and without network access.
- No proprietary game-file download is performed by Horizon C.
- The disclaimer blocks first launch until acknowledged and can be viewed later.
- Offline/LAN behavior and authenticated-server limitations are documented accurately.
- Formatting, tests, and CI pass; report any checks that could not be run.
