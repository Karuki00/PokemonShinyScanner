import { useEffect, useRef, useState, type CSSProperties } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { relaunch } from "@tauri-apps/plugin-process";
import { check, type Update } from "@tauri-apps/plugin-updater";
import "./App.css";
import buttonSoundUrl from "./assets/sound_effects/button.mp3";
import pcOnSoundUrl from "./assets/sound_effects/pc-on.mp3";
import pcOffSoundUrl from "./assets/sound_effects/pc-off.mp3";

const normalPokemonSprites = import.meta.glob("./assets/pokemon/*.png", {
  eager: true,
  import: "default",
  query: "?url",
}) as Record<string, string>;
const shinyPokemonSprites = import.meta.glob("./assets/pokemon/shiny/*.png", {
  eager: true,
  import: "default",
  query: "?url",
}) as Record<string, string>;
const pcBoxWallpapers = import.meta.glob("./assets/pc_box/wallpaper_*.png", {
  eager: true,
  import: "default",
  query: "?url",
}) as Record<string, string>;

interface PokemonIvs {
  hp: number;
  attack: number;
  defense: number;
  speed: number;
  specialAttack: number;
  specialDefense: number;
}

interface PokemonEvs {
  hp: number;
  attack: number;
  defense: number;
  speed: number;
  specialAttack: number;
  specialDefense: number;
}

interface ScannedPokemon {
  partySlot: number;
  pid: number;
  otid: number;
  trainerId: number;
  secretId: number;
  species: number;
  speciesName: string;
  nationalDex: number;
  natureId: number;
  natureName: string;
  shinyValue: number;
  shiny: boolean;
  abilityNumber: number;
  abilityName: string;
  checksumValid: boolean;
  ivs: PokemonIvs;
  evs: PokemonEvs;
  primaryType?: string;
}

interface PartyUpdate {
  connected: boolean;
  partyCount: number;
  pokemon: ScannedPokemon[];
}

interface PcBoxSnapshot {
  index: number;
  occupiedCount: number;
  invalidSlots: number[];
  slots: (ScannedPokemon | null)[];
}

interface PcStorageUpdate {
  connected: boolean;
  supported: boolean;
  available: boolean;
  currentBox: number | null;
  totalPokemon: number;
  boxes: PcBoxSnapshot[];
  message: string;
}

interface GameStateUpdate {
  connected: boolean;
  state: string;
  detail: string;
  partyCount: number;
  battleTypeFlags: number;
  emeraldCallback2: number;
  starterSelectionReady: boolean;
  starterConfirmationReady: boolean;
  emeraldStarterSelection: number | null;
  battleActionMenuReady: boolean;
  battleActionMenuCursor: number | null;
  wildAppearedTextReady: boolean;
  wildEscapedTextReady: boolean;
  battleTextReady: boolean;
  emeraldInBattle: boolean | null;
  emeraldPokeballCount: number | null;
  emeraldBagMenuReady: boolean;
  emeraldBagPocket: number | null;
  emeraldBagCursor: number | null;
  emeraldNicknamePromptReady: boolean;
  emeraldNicknamePromptCursor: number | null;
  emeraldDexRegistrationReady: boolean;
  emeraldCatchConfirmed: boolean;
  emeraldOverworldReady: boolean;
  emeraldBattleOutcome: number | null;
  emeraldBattleCommunication: number[] | null;
  emeraldBattleTextStringId: number | null;
  emeraldCaughtSpecies: number | null;
  emeraldWildSpecies: number | null;
  emeraldDexTaskId: number | null;
  emeraldDexTaskFunction: number | null;
  emeraldDexTaskActive: boolean | null;
  emeraldDexPageInputReady: boolean;
  emeraldNamingScreenActive: boolean;
}

interface BattleUiDiagnosticSample {
  timestampUnixMs: number;
  gameProfile: string;
  addressBasis: string;
  ewramProcessBase: string | null;
  iwramProcessBase: string | null;
  battleTypeFlagsAddress: string | null;
  battleTypeFlags: number | null;
  activeBattlerAddress: string | null;
  activeBattler: number | null;
  controllerFunctionAddress: string | null;
  controllerFunction: number | null;
  battlerControllerFunctions: number[];
  controllerExecFlagsAddress: string | null;
  controllerExecFlags: number | null;
  actionCursorAddress: string | null;
  actionCursor: number | null;
  currentMenuCandidate: boolean;
  candidateState: string;
  emeraldInBattleAddress: string | null;
  emeraldInBattle: boolean | null;
  battleBufferCommandAddress: string | null;
  battleBufferCommand: number | null;
  battleBufferStringId: number | null;
  battlerBufferCommands: number[];
  battlerBufferStringIds: number[];
  wildAppearedTextCandidate: boolean;
  wildEscapedTextCandidate: boolean;
  emeraldCallback2: number | null;
  opponentSpecies: number | null;
  opponentPid: number | null;
  opponentShiny: boolean | null;
  romFingerprintVerified: boolean;
}

type Page = "dashboard" | "party" | "pc-storage" | "pokemon-details" | "ev-training" | "starter" | "wild" | "settings";
type HuntMode = "starter" | "wild";
type StarterTarget = "treecko" | "mudkip" | "torchic";
type EvStat = keyof PokemonIvs;
type EvValues = Record<EvStat, number>;
interface PokemonEvTrack {
  target: EvValues;
}
type EvTrackStorage = Record<string, PokemonEvTrack>;

const EV_STORAGE_KEY = "emerald-iv-scanner-ev-tracks";
const EV_STATS: { id: EvStat; label: string }[] = [
  { id: "hp", label: "HP" },
  { id: "attack", label: "Attack" },
  { id: "defense", label: "Defense" },
  { id: "specialAttack", label: "Sp. Attack" },
  { id: "specialDefense", label: "Sp. Defense" },
  { id: "speed", label: "Speed" },
];
const EV_TRAINING_GUIDES: Record<EvStat, { pokemon: string; yield: number }[]> = {
  hp: [
    { pokemon: "Marill", yield: 2 },
    { pokemon: "Azurill", yield: 1 },
    { pokemon: "Wailmer", yield: 2 },
  ],
  attack: [
    { pokemon: "Poochyena", yield: 1 },
    { pokemon: "Mightyena", yield: 2 },
    { pokemon: "Shuppet", yield: 2 },
  ],
  defense: [
    { pokemon: "Geodude", yield: 1 },
    { pokemon: "Aron", yield: 1 },
    { pokemon: "Graveler", yield: 2 },
  ],
  specialAttack: [
    { pokemon: "Ralts", yield: 1 },
    { pokemon: "Numel", yield: 1 },
    { pokemon: "Roselia", yield: 2 },
  ],
  specialDefense: [
    { pokemon: "Tentacool", yield: 1 },
    { pokemon: "Tentacruel", yield: 2 },
    { pokemon: "Dustox", yield: 3 },
  ],
  speed: [
    { pokemon: "Wingull", yield: 1 },
    { pokemon: "Zigzagoon", yield: 1 },
    { pokemon: "Zubat", yield: 1 },
  ],
};
const EMPTY_EV_VALUES: EvValues = {
  hp: 0,
  attack: 0,
  defense: 0,
  speed: 0,
  specialAttack: 0,
  specialDefense: 0,
};
// Emerald's chooser order is Treecko, Torchic, Mudkip; it starts on Torchic (index 1).
const STARTER_INDEX: Record<StarterTarget, number> = { treecko: 0, torchic: 1, mudkip: 2 };
const STARTER_NAME: Record<StarterTarget, string> = { treecko: "Treecko", mudkip: "Mudkip", torchic: "Torchic" };

interface HuntSession {
  attempts: number;
}

interface HuntStorage {
  starter: HuntSession | null;
  wild: HuntSession | null;
}

const STORAGE_KEY = "emerald-iv-scanner-hunts";
const HUNT_POKEMON_CACHE_KEY = "emerald-iv-scanner-latest-hunt-pokemon";
const BOX_THEME_STORAGE_KEY = "emerald-iv-scanner-box-theme";
const GAME_STORAGE_KEY = "pokemon-shiny-scanner-game";
const APP_VERSION = "0.3.1";
const UPDATE_CHECK_INTERVAL_MS = 15 * 60 * 1000;
const RELEASES_URL = "https://github.com/Karuki00/PokemonShinyScanner/releases/tag";

type GameId = "emerald" | "ruby" | "sapphire" | "firered" | "leafgreen";
type RegionId = "hoenn" | "kanto";

const GAMES: {
  id: GameId;
  name: string;
  generation: string;
  region: RegionId;
  available: boolean;
}[] = [
  { id: "emerald", name: "Pokémon Emerald", generation: "GEN III", region: "hoenn", available: true },
  { id: "ruby", name: "Pokémon Ruby", generation: "GEN III", region: "hoenn", available: true },
  { id: "sapphire", name: "Pokémon Sapphire", generation: "GEN III", region: "hoenn", available: true },
  { id: "firered", name: "Pokémon FireRed", generation: "GEN III", region: "kanto", available: true },
  { id: "leafgreen", name: "Pokémon LeafGreen", generation: "GEN III", region: "kanto", available: true },
];

const REGIONS: { id: RegionId; name: string; subtitle: string; games: GameId[] }[] = [
  { id: "hoenn", name: "Hoenn Scanner", subtitle: "Emerald · Ruby · Sapphire", games: ["emerald", "ruby", "sapphire"] },
  { id: "kanto", name: "Kanto Scanner", subtitle: "FireRed · LeafGreen", games: ["firered", "leafgreen"] },
];
const BOX_THEMES = [
  "Meadow",
  "Stone",
  "Sand",
  "Field",
  "Lavender",
  "Volcanic",
  "Ice",
  "Cave",
  "Beach",
  "Ocean",
  "Tidal",
  "Sky",
  "Battle",
  "Metal",
  "Crimson",
  "Clean",
  "Reshiram",
  "Zekrom",
  "Poké Ball",
  "League",
  "Floral",
  "Dark",
  "Map",
  "Contest",
] as const;

const BOX_THEME_PALETTES = [
  { accent: "#4eaa68", secondary: "#82cc78", highlight: "#c3f19e", border: "#397d51", glow: "rgba(92, 214, 112, .35)", tint: "rgba(8, 36, 20, .34)" },
  { accent: "#929ca5", secondary: "#c0c6ca", highlight: "#e5e8e9", border: "#626e78", glow: "rgba(184, 198, 207, .28)", tint: "rgba(27, 34, 39, .4)" },
  { accent: "#d9a64e", secondary: "#ebc875", highlight: "#fff0ad", border: "#9b7133", glow: "rgba(255, 200, 92, .3)", tint: "rgba(54, 35, 9, .34)" },
  { accent: "#78994b", secondary: "#a9c268", highlight: "#d9e89b", border: "#536d36", glow: "rgba(176, 215, 91, .29)", tint: "rgba(30, 41, 14, .36)" },
  { accent: "#a775c4", secondary: "#c69bdd", highlight: "#ead2f4", border: "#78518e", glow: "rgba(207, 143, 235, .32)", tint: "rgba(44, 22, 55, .37)" },
  { accent: "#d96e55", secondary: "#ef9472", highlight: "#ffd0a0", border: "#a74e40", glow: "rgba(255, 119, 74, .33)", tint: "rgba(58, 22, 17, .36)" },
  { accent: "#65bcd0", secondary: "#9ce2eb", highlight: "#e0ffff", border: "#43899a", glow: "rgba(107, 220, 245, .34)", tint: "rgba(14, 42, 52, .36)" },
  { accent: "#8878c7", secondary: "#b5a8e6", highlight: "#e3dbff", border: "#645393", glow: "rgba(151, 126, 239, .32)", tint: "rgba(31, 24, 57, .38)" },
  { accent: "#d6ad69", secondary: "#e9ce91", highlight: "#fff0c5", border: "#9f804e", glow: "rgba(246, 202, 121, .3)", tint: "rgba(51, 38, 18, .34)" },
  { accent: "#527fd1", secondary: "#73a9eb", highlight: "#b7e3ff", border: "#3a5ea1", glow: "rgba(78, 149, 255, .36)", tint: "rgba(17, 34, 83, .37)" },
  { accent: "#39aeb8", secondary: "#65d5d0", highlight: "#b2fff0", border: "#287f89", glow: "rgba(66, 226, 220, .34)", tint: "rgba(9, 48, 53, .37)" },
  { accent: "#6e9fd8", secondary: "#a4c9f1", highlight: "#e0f1ff", border: "#4b719f", glow: "rgba(131, 185, 255, .32)", tint: "rgba(25, 42, 67, .35)" },
  { accent: "#cc617d", secondary: "#e98ba0", highlight: "#ffd1d9", border: "#934458", glow: "rgba(255, 113, 149, .32)", tint: "rgba(55, 21, 32, .38)" },
  { accent: "#9ba7b2", secondary: "#c2ccd3", highlight: "#edf2f4", border: "#6e7b86", glow: "rgba(196, 216, 229, .27)", tint: "rgba(25, 34, 41, .4)" },
  { accent: "#ce4c50", secondary: "#ec7371", highlight: "#ffb1a8", border: "#963337", glow: "rgba(255, 75, 75, .34)", tint: "rgba(55, 15, 19, .4)" },
  { accent: "#b5c3bd", secondary: "#d3ddd8", highlight: "#f5faf7", border: "#7d9188", glow: "rgba(220, 241, 232, .27)", tint: "rgba(31, 41, 38, .4)" },
  { accent: "#c8b78d", secondary: "#e2d3b3", highlight: "#fff2d4", border: "#92815d", glow: "rgba(240, 215, 161, .28)", tint: "rgba(47, 39, 24, .39)" },
  { accent: "#2385db", secondary: "#3975cf", highlight: "#7bd9ff", border: "#374e71", glow: "rgba(39, 153, 255, .42)", tint: "rgba(14, 22, 31, .46)" },
  { accent: "#d74a58", secondary: "#f4f1e9", highlight: "#ffffff", border: "#9e3945", glow: "rgba(255, 72, 85, .34)", tint: "rgba(25, 25, 29, .42)" },
  { accent: "#d3b14e", secondary: "#557dc2", highlight: "#f5dc7c", border: "#8c7138", glow: "rgba(243, 200, 83, .32)", tint: "rgba(43, 37, 24, .38)" },
  { accent: "#d96bb7", secondary: "#efa5d9", highlight: "#ffe2f4", border: "#a94f8d", glow: "rgba(255, 121, 204, .35)", tint: "rgba(59, 25, 50, .34)" },
  { accent: "#9c4255", secondary: "#c35b66", highlight: "#e88b8d", border: "#632d3a", glow: "rgba(210, 69, 91, .3)", tint: "rgba(29, 18, 25, .46)" },
  { accent: "#719f78", secondary: "#c5ad69", highlight: "#e7d69a", border: "#536d55", glow: "rgba(160, 211, 139, .3)", tint: "rgba(35, 39, 25, .36)" },
  { accent: "#bb3f70", secondary: "#df668e", highlight: "#ffb4c9", border: "#812e53", glow: "rgba(242, 70, 137, .34)", tint: "rgba(48, 17, 35, .42)" },
] as const;

const EMPTY_PARTY: PartyUpdate = {
  connected: false,
  partyCount: 0,
  pokemon: [],
};

const EMPTY_PC_STORAGE: PcStorageUpdate = {
  connected: false,
  supported: true,
  available: false,
  currentBox: null,
  totalPokemon: 0,
  boxes: [],
  message: "Start the scanner to read PC storage.",
};

const EMPTY_GAME_STATE: GameStateUpdate = {
  connected: false,
  state: "disconnected",
  detail: "Start the scanner to monitor verified game states.",
  partyCount: 0,
  battleTypeFlags: 0,
  emeraldCallback2: 0,
  starterSelectionReady: false,
  starterConfirmationReady: false,
  emeraldStarterSelection: null,
  battleActionMenuReady: false,
  battleActionMenuCursor: null,
  wildAppearedTextReady: false,
  wildEscapedTextReady: false,
  battleTextReady: false,
  emeraldInBattle: null,
  emeraldPokeballCount: null,
  emeraldBagMenuReady: false,
  emeraldBagPocket: null,
  emeraldBagCursor: null,
  emeraldNicknamePromptReady: false,
  emeraldNicknamePromptCursor: null,
  emeraldDexRegistrationReady: false,
  emeraldCatchConfirmed: false,
  emeraldOverworldReady: false,
  emeraldBattleOutcome: null,
  emeraldBattleCommunication: null,
  emeraldBattleTextStringId: null,
  emeraldCaughtSpecies: null,
  emeraldWildSpecies: null,
  emeraldDexTaskId: null,
  emeraldDexTaskFunction: null,
  emeraldDexTaskActive: null,
  emeraldDexPageInputReady: false,
  emeraldNamingScreenActive: false,
};

const EMPTY_HUNTS: HuntStorage = {
  starter: null,
  wild: null,
};

const IV_STATS: { key: keyof PokemonIvs; label: string }[] = [
  { key: "hp", label: "HP" },
  { key: "attack", label: "Attack" },
  { key: "defense", label: "Defense" },
  { key: "specialAttack", label: "Sp. Attack" },
  { key: "specialDefense", label: "Sp. Defense" },
  { key: "speed", label: "Speed" },
];

const TYPE_COLORS: Record<string, string> = {
  normal: "#B9AE91",
  fire: "#BD795F",
  water: "#7299BD",
  electric: "#B9A34F",
  grass: "#789C78",
  ice: "#79AFAE",
  fighting: "#B87569",
  poison: "#9A7AA5",
  ground: "#B49A6B",
  flying: "#8C9CC1",
  psychic: "#B77C9B",
  bug: "#91A05C",
  rock: "#A69770",
  ghost: "#8275A5",
  dragon: "#8274B9",
  dark: "#81766F",
  steel: "#8799A7",
  fairy: "#BD91A9",
};

function loadHunts(): HuntStorage {
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    return saved ? { ...EMPTY_HUNTS, ...JSON.parse(saved) } : EMPTY_HUNTS;
  } catch {
    return EMPTY_HUNTS;
  }
}

function loadLatestHuntPokemon(): { starter: ScannedPokemon | null; wild: ScannedPokemon | null } {
  try {
    const saved = localStorage.getItem(HUNT_POKEMON_CACHE_KEY);
    return saved ? JSON.parse(saved) : { starter: null, wild: null };
  } catch {
    return { starter: null, wild: null };
  }
}

function loadBoxTheme() {
  const saved = Number.parseInt(
    localStorage.getItem(BOX_THEME_STORAGE_KEY) ?? "0",
    10,
  );

  return Number.isInteger(saved) && saved >= 0 && saved < BOX_THEMES.length
    ? saved
    : 0;
}

function loadGame(): GameId {
  const saved = localStorage.getItem(GAME_STORAGE_KEY);
  return GAMES.some((game) => game.id === saved && game.available)
    ? (saved as GameId)
    : "emerald";
}

function getBoxWallpaperUrl(themeIndex: number) {
  const filename = `wallpaper_${String(themeIndex).padStart(2, "0")}.png`;
  return pcBoxWallpapers[`./assets/pc_box/${filename}`] ?? "";
}

function getSpriteUrl(pokemon: ScannedPokemon) {
  if (!pokemon.nationalDex) return "";
  const spriteSet = pokemon.shiny ? shinyPokemonSprites : normalPokemonSprites;
  return (
    spriteSet[
      `./assets/pokemon${pokemon.shiny ? "/shiny" : ""}/${pokemon.nationalDex}.png`
    ] ?? ""
  );
}

function getPokemonColor(pokemon: ScannedPokemon) {
  const type = pokemon.primaryType?.toLowerCase() ?? "grass";
  return TYPE_COLORS[type] ?? TYPE_COLORS.grass;
}

function formatPid(pid: number) {
  return pid.toString(16).toUpperCase().padStart(8, "0");
}

function getTotalIv(pokemon: ScannedPokemon) {
  return Object.values(pokemon.ivs).reduce((total, value) => total + value, 0);
}

function getEvTrackKey(game: GameId, pokemon: ScannedPokemon) {
  return `${game}:${pokemon.trainerId}:${pokemon.pid}:${pokemon.species}`;
}

function normalizeEvValues(value: unknown): EvValues {
  const source = value && typeof value === "object" ? value as Record<string, unknown> : {};
  return Object.fromEntries(EV_STATS.map(({ id }) => {
    const number = Number(source[id] ?? 0);
    return [id, Number.isFinite(number) ? Math.min(255, Math.max(0, Math.floor(number))) : 0];
  })) as EvValues;
}

function loadEvTracks(): EvTrackStorage {
  try {
    const stored = localStorage.getItem(EV_STORAGE_KEY);
    if (!stored) return {};
    const parsed: unknown = JSON.parse(stored);
    if (!parsed || typeof parsed !== "object") return {};
    return Object.fromEntries(Object.entries(parsed).map(([key, value]) => {
      const track = value && typeof value === "object" ? value as Record<string, unknown> : {};
      return [key, { target: normalizeEvValues(track.target) }];
    }));
  } catch {
    return {};
  }
}

function playUiSound(source: string) {
  const audio = new Audio(source);
  audio.volume = 0.35;
  void audio.play().catch(() => undefined);
}

function FirstLaunchPicker({
  selectedGame,
  onSelect,
}: {
  selectedGame: GameId;
  onSelect: (game: GameId) => void;
}) {
  return (
    <div className="first-launch-backdrop">
      <section className="first-launch-picker" aria-labelledby="scanner-choice-title">
        <span className="eyebrow">POKÉMON SHINY SCANNER</span>
        <h2 id="scanner-choice-title">Choose your region</h2>
        <p className="first-launch-intro">
          Select the game family you want to scan. More generations will join the
          scanner as their emulator memory profiles are added.
        </p>

        <div className="region-choice-grid">
          {REGIONS.map((region) => (
            <article className="region-choice" key={region.id}>
              <div className={`region-emblem region-emblem-${region.id}`}>
                {region.id === "hoenn" ? "H" : "K"}
              </div>
              <div>
                <span className="region-kicker">GEN III · {region.id.toUpperCase()}</span>
                <h3>{region.name}</h3>
                <p>{region.subtitle}</p>
              </div>
              <div className="region-game-list">
                {region.games.map((gameId) => {
                  const game = GAMES.find((candidate) => candidate.id === gameId)!;
                  return (
                    <button
                      className={`region-game-button ${selectedGame === game.id ? "selected" : ""}`}
                      disabled={!game.available}
                      key={game.id}
                      onClick={() => onSelect(game.id)}
                    >
                      <span>{game.name.replace("Pokémon ", "")}</span>
                      {!game.available && <small>COMING SOON</small>}
                    </button>
                  );
                })}
              </div>
            </article>
          ))}
        </div>
      </section>
    </div>
  );
}

function getRandomFloatingSprites() {
  const ids = Array.from({ length: 386 }, (_, index) => index + 1);

  for (let index = ids.length - 1; index > 0; index -= 1) {
    const randomIndex = Math.floor(Math.random() * (index + 1));
    [ids[index], ids[randomIndex]] = [ids[randomIndex], ids[index]];
  }

  return ids.slice(0, 3).map((nationalDex) => ({
    nationalDex,
    shiny: Math.random() < 1 / 1024,
  }));
}

function PokemonSprite({
  pokemon,
  className = "",
}: {
  pokemon: ScannedPokemon;
  className?: string;
}) {
  const [failed, setFailed] = useState(false);
  const url = getSpriteUrl(pokemon);

  if (!url || failed) {
    return (
      <div className={`sprite-placeholder ${className}`}>
        <span>?</span>
      </div>
    );
  }

  return (
    <img
      className={className}
      src={url}
      alt={pokemon.speciesName}
      onError={() => setFailed(true)}
    />
  );
}

function PokemonCard({
  pokemon,
  slot,
  selected,
  onClick,
}: {
  pokemon: ScannedPokemon;
  slot: number;
  selected: boolean;
  onClick: () => void;
}) {
  const totalIv = getTotalIv(pokemon);
  const color = getPokemonColor(pokemon);

  return (
    <button
      className={`party-card ${selected ? "selected" : ""}`}
      style={{ "--pokemon-color": color } as CSSProperties}
      onClick={onClick}
    >
      <span className="party-card-slot">
        SLOT {String(slot).padStart(2, "0")}
      </span>

      <PokemonSprite pokemon={pokemon} className="party-card-sprite" />

      <div className="party-card-info">
        <div className="party-card-name-row">
          <strong>{pokemon.speciesName}</strong>
          {pokemon.shiny && <span className="shiny-mark">✦</span>}
        </div>

        <span className="party-card-meta">
          #{pokemon.nationalDex || "?"} · {pokemon.natureName}
        </span>

        <div className="party-card-footer">
          <span>IV TOTAL</span>
          <strong>{totalIv}/186</strong>
        </div>
      </div>
    </button>
  );
}

function EmptyPartyCard({ slot }: { slot: number }) {
  return (
    <div className="party-card empty-party-card">
      <span className="party-card-slot">
        SLOT {String(slot).padStart(2, "0")}
      </span>
      <div className="empty-party-art">＋</div>
      <div className="party-card-info">
        <strong>Empty Slot</strong>
        <span className="party-card-meta">No Pokémon detected</span>
      </div>
    </div>
  );
}

function PcStoragePage({
  storage,
  selectedBox,
  onSelectBox,
  onSelectPokemon,
}: {
  storage: PcStorageUpdate;
  selectedBox: number;
  onSelectBox: (box: number) => void;
  onSelectPokemon: (pokemon: ScannedPokemon) => void;
}) {
  const activeBox = storage.boxes.find((box) => box.index === selectedBox);

  return (
    <div className="page-content pcbox-screen pc-storage-page">
      <div className="page-heading">
        <div>
          <span className="eyebrow">READ-ONLY EMULATOR MEMORY</span>
          <h2>Pokemon PC Storage</h2>
          <p>Browse the Pokemon stored in your game. This view reads live save memory and never changes it.</p>
        </div>
        <span className="party-count">{storage.available ? `${storage.totalPokemon} STORED` : "LIVE SCAN"}</span>
      </div>

      {!storage.connected || !storage.available ? (
        <div className="empty-state pc-storage-empty">
          <div className="empty-icon">PC</div>
          <h3>{storage.connected ? "PC storage not available" : "Waiting for emulator scan"}</h3>
          <p>{storage.message}</p>
        </div>
      ) : (
        <>
          {activeBox && (
            <section className="pc-storage-box" aria-label={`Box ${activeBox.index + 1}`}>
              <div className="pc-storage-slots">
                {activeBox.slots.map((pokemon, index) => pokemon ? (
                  <button
                    type="button"
                    className={`pc-storage-slot occupied${pokemon.shiny ? " shiny" : ""}`}
                    key={index}
                    onClick={() => onSelectPokemon(pokemon)}
                    aria-label={`${pokemon.shiny ? "Shiny " : ""}${pokemon.speciesName}, box ${activeBox.index + 1}, slot ${index + 1}, shiny value ${pokemon.shinyValue}`}
                  >
                    <span className="pc-slot-index">{String(index + 1).padStart(2, "0")}</span>
                    <PokemonSprite pokemon={pokemon} className="pc-storage-sprite" />
                    {pokemon.shiny && <span className="pc-storage-shiny">SHINY</span>}
                    <strong>{pokemon.speciesName}</strong>
                    <small>SV {pokemon.shinyValue}</small>
                  </button>
                ) : activeBox.invalidSlots.includes(index + 1) ? (
                  <div className="pc-storage-slot invalid" key={index} aria-label={`Unverified Pokemon data in slot ${index + 1}`}>
                    <span className="pc-slot-index">{String(index + 1).padStart(2, "0")}</span>
                    <span className="pc-invalid-mark" aria-hidden="true">!</span>
                    <small>Unverified</small>
                  </div>
                ) : (
                  <div className="pc-storage-slot empty" key={index} aria-label={`Empty slot ${index + 1}`}>
                    <span className="pc-slot-index">{String(index + 1).padStart(2, "0")}</span>
                    <span className="pc-empty-ball" aria-hidden="true">+</span>
                    <small>Empty</small>
                  </div>
                ))}
              </div>
            </section>
          )}

          {activeBox && (
            <nav className="pc-box-pager" aria-label="PC box pagination">
              <button
                type="button"
                className="pc-box-page-button"
                disabled={selectedBox <= 0}
                onClick={() => onSelectBox(Math.max(0, selectedBox - 1))}
                aria-label="Previous box"
              >
                <span aria-hidden="true">&lt;</span>
                <small>PREVIOUS</small>
              </button>
              <div className="pc-box-page-current" aria-live="polite">
                <small>PC BOX</small>
                <div><strong>{String(activeBox.index + 1).padStart(2, "0")}</strong><span>/ 14</span></div>
                <em>{activeBox.occupiedCount} / 30 STORED{storage.currentBox === activeBox.index ? " | GAME SELECTED" : ""}</em>
              </div>
              <button
                type="button"
                className="pc-box-page-button"
                disabled={selectedBox >= storage.boxes.length - 1}
                onClick={() => onSelectBox(Math.min(storage.boxes.length - 1, selectedBox + 1))}
                aria-label="Next box"
              >
                <small>NEXT</small>
                <span aria-hidden="true">&gt;</span>
              </button>
            </nav>
          )}
        </>
      )}
    </div>
  );
}
function PokemonDetails({
  pokemon,
  hideArt = false,
  huntCounter,
  huntCounterLabel,
  onReset,
  evTarget,
  onEvTargetChange,
}: {
  pokemon: ScannedPokemon;
  hideArt?: boolean;
  huntCounter?: number;
  huntCounterLabel?: string;
  onReset?: () => void;
  evTarget?: EvValues;
  onEvTargetChange?: (stat: EvStat, value: number) => void;
}) {
  const totalIv = getTotalIv(pokemon);
  const color = getPokemonColor(pokemon);

  return (
    <section
      className={`pokemon-detail-panel summary-panel ${hideArt ? "details-only" : ""}`}
      style={{ "--pokemon-color": color } as CSSProperties}
    >
      {/* Pokémon profile */}
      <div className="summary-profile">
        <div className="summary-art">
          <div className="summary-dex">
            NATIONAL DEX
            <strong>#{String(pokemon.nationalDex).padStart(3, "0")}</strong>
          </div>

          <PokemonSprite pokemon={pokemon} className="summary-sprite" />

          {huntCounter !== undefined && (
            <div className="hunt-counter-chip summary-counter-chip">
              <span>{huntCounterLabel}</span>
              <strong>{huntCounter.toLocaleString()}</strong>
            </div>
          )}

          {pokemon.shiny && <span className="summary-shiny">✦ SHINY</span>}
        </div>

        <div className="summary-identity">
          <span className="eyebrow">POKÉMON SUMMARY</span>
          <div className="summary-title-row">
            <span className="eyebrow">POKÉMON SUMMARY</span>
            {onReset && (
              <button className="reset-hunt-button" onClick={onReset}>
                ↻ Reset Counter
              </button>
            )}
          </div>
          <h2>{pokemon.speciesName}</h2>

          <div className="summary-badges">
            <span className="summary-badge">{pokemon.natureName} Nature</span>
            <span
              className={`badge ${pokemon.checksumValid ? "valid" : "invalid"}`}
            >
              {pokemon.checksumValid ? "DATA VERIFIED" : "INVALID DATA"}
            </span>
          </div>

          <div className="summary-info-grid">
            <div>
              <span>ABILITY</span>
              <strong>{pokemon.abilityName}</strong>
            </div>

            <div>
              <span>SHINY</span>
              <div className="summary-shiny-status">
                <strong className={pokemon.shiny ? "shiny-text" : ""}>
                  {pokemon.shiny ? "Yes" : "No"}
                </strong>
                <small title="Generation III shiny values 0 through 7 are shiny">
                  SV {pokemon.shinyValue.toLocaleString()} / 65,536
                </small>
              </div>
            </div>

            <div>
              <span>PERSONALITY</span>
              <strong>{formatPid(pokemon.pid)}</strong>
            </div>

            <div>
              <span>IV TOTAL</span>
              <strong>{totalIv} / 186</strong>
            </div>
          </div>
        </div>
      </div>

      {/* IV summary */}
      <div className="summary-iv-section">
        <div className="summary-section-title">
          <div>
            <span className="eyebrow">INDIVIDUAL VALUES</span>
            <h3>Stat Potential</h3>
          </div>

          <strong>{totalIv}/186</strong>
        </div>

        <div className="summary-iv-grid">
          {IV_STATS.map(({ key, label }) => {
            const value = pokemon.ivs[key];
            const percentage = (value / 31) * 100;

            return (
              <div className="summary-iv-card" key={key}>
                <div className="summary-iv-header">
                  <span>{label}</span>
                  <strong>
                    {value}
                    <small>/31</small>
                  </strong>
                </div>

                <div className="summary-iv-track">
                  <div
                    className="summary-iv-fill"
                    style={{ width: `${percentage}%` }}
                  />
                </div>
              </div>
            );
          })}
        </div>
      </div>

      {evTarget && onEvTargetChange && (
        <EvTrainingSummary evs={pokemon.evs} target={evTarget} onTargetChange={onEvTargetChange} />
      )}

      {/* Technical data */}
      <details className="pokemon-details summary-technical">
        <summary>Trainer and Pokémon data</summary>

        <div className="details-grid">
          <div>
            <span>Trainer ID</span>
            <strong>{pokemon.trainerId}</strong>
          </div>
          <div>
            <span>Secret ID</span>
            <strong>{pokemon.secretId}</strong>
          </div>
          <div>
            <span>Nature ID</span>
            <strong>{pokemon.natureId}</strong>
          </div>
          <div>
            <span>Ability Number</span>
            <strong>{pokemon.abilityNumber}</strong>
          </div>
          <div>
            <span>Original Trainer ID</span>
            <strong>{formatPid(pokemon.otid)}</strong>
          </div>
          <div>
            <span>Personality Value</span>
            <strong>{formatPid(pokemon.pid)}</strong>
          </div>
        </div>
      </details>
    </section>
  );
}


function totalEvs(values: EvValues) {
  return EV_STATS.reduce((sum, { id }) => sum + values[id], 0);
}

function EvTrainingSummary({
  evs,
  target,
  onTargetChange,
}: {
  evs: PokemonEvs;
  target: EvValues;
  onTargetChange: (stat: EvStat, value: number) => void;
}) {
  const total = totalEvs(evs);
  const targetTotal = totalEvs(target);
  const progress = targetTotal === 0
    ? 0
    : Math.min(100, Math.round(
      (EV_STATS.reduce((sum, { id }) => sum + Math.min(evs[id], target[id]), 0) / targetTotal) * 100,
    ));
  const remaining = EV_STATS.reduce(
    (sum, { id }) => sum + Math.max(0, target[id] - evs[id]),
    0,
  );

  return (
    <div className="summary-ev-section">
      <div className="summary-section-title">
        <div>
          <span className="eyebrow">EFFORT VALUES</span>
          <h3>EV Training</h3>
        </div>
        <strong>{total}/510 <small className="ev-available-points">{Math.max(0, 510 - total)} available</small></strong>
      </div>

      <div className="ev-progress-summary">
        <div>
          <span>SPREAD GOAL</span>
          <strong>{targetTotal}/510 EVs planned</strong>
        </div>
        <div>
          <span>LEFT TO GOAL</span>
          <strong>{remaining} EVs</strong>
        </div>
        <div className="ev-progress-track">
          <div className="ev-progress-fill" style={{ width: `${progress}%` }} />
        </div>
        <small>{progress}% of planned spread reached · Gained EVs are read from emulator memory.</small>
      </div>

      <div className="ev-stat-list">
        {EV_STATS.map(({ id, label }) => (
          <div className={`ev-stat-row ev-stat-${id}`} data-stat={id} key={id}>
            <div className="ev-stat-name">
              <span className="ev-stat-hex" aria-hidden="true">⬡</span>
              <span><strong>{label}</strong><small>MAX 255</small></span>
            </div>
            <label>
              <span>GAINED</span>
              <output aria-label={`${label} EVs gained`}>{evs[id]}</output>
            </label>
            <label>
              <span>GOAL</span>
              <input
                aria-label={`${label} EV goal`}
                type="number"
                min={0}
                max={255}
                value={target[id]}
                onChange={(event) => onTargetChange(id, Number(event.target.value))}
              />
              <small className="ev-to-go">{Math.max(0, target[id] - evs[id])} to goal</small>
            </label>
            <div className="ev-stat-meter" aria-label={`${label}: ${evs[id]} of 255 EVs`}>
              <span style={{ width: `${Math.min(100, (evs[id] / 255) * 100)}%` }} />
            </div>
            <div className="ev-training-tip">
              <span>TRAIN WITH</span>
              {EV_TRAINING_GUIDES[id].map(({ pokemon, yield: evYield }) => (
                <strong key={pokemon}>{pokemon} +{evYield}</strong>
              ))}
              <small>EV{EV_TRAINING_GUIDES[id].some(({ yield: evYield }) => evYield > 1) ? "s" : ""} per defeat</small>
            </div>
          </div>
        ))}
      </div>
      <small className="ev-limit-note">Generation III limits: 255 EVs per stat and 510 EVs total. Target spreads are saved on this device.</small>
    </div>
  );
}

function EvTrainingPage({
  party,
  selectedPokemon,
  target,
  onSelectPokemon,
  onEvTargetChange,
}: {
  party: PartyUpdate;
  selectedPokemon: ScannedPokemon | null;
  target: EvValues | null;
  onSelectPokemon: (pokemon: ScannedPokemon) => void;
  onEvTargetChange: (pokemon: ScannedPokemon, stat: EvStat, value: number) => void;
}) {
  return (
    <div className="page-content ev-training-page">
      <div className="page-heading">
        <div>
          <span className="eyebrow">GEN III COMPETITIVE PREPARATION</span>
          <h2>EV Training</h2>
          <p>EV totals are read from emulator memory. Set target spreads for your party members here.</p>
        </div>
        <span className="party-count">{party.partyCount} / 6 MEMBERS</span>
      </div>

      {!party.pokemon.length ? (
        <div className="empty-state">
          <div className="empty-icon">✦</div>
          <h3>No party Pokémon detected</h3>
          <p>Start the scanner to read the EV totals stored in your party.</p>
        </div>
      ) : (
        <>
          <div className="ev-party-picker">
            {party.pokemon.map((pokemon) => (
              <PokemonCard
                key={`${pokemon.pid}-${pokemon.partySlot}`}
                pokemon={pokemon}
                slot={pokemon.partySlot}
                selected={selectedPokemon?.partySlot === pokemon.partySlot}
                onClick={() => onSelectPokemon(pokemon)}
              />
            ))}
          </div>

          {selectedPokemon && target && (
            <PokemonDetails
              pokemon={selectedPokemon}
              evTarget={target}
              onEvTargetChange={(stat, value) => onEvTargetChange(selectedPokemon, stat, value)}
            />
          )}
        </>
      )}
    </div>
  );
}

function HuntPage({
  mode,
  session,
  partySlotOne,
  latestWild,
  onIncrement,
  onRequestReset,
  onPressDiagnosticA,
  diagnosticStatus,
  onStartAutomaticHunt,
  onStopAutomaticHunt,
  automaticHuntRunning,
  wildHuntStatus,
  onStartAutomaticWildHunt,
  onStopAutomaticWildHunt,
  automaticWildHuntRunning,
  battleUiDiagnosticEnabled,
  battleUiDiagnosticSampleCount,
  onToggleBattleUiDiagnostic,
  starterTarget,
  onStarterTargetChange,
  scanning,
}: {
  mode: HuntMode;
  session: HuntSession | null;
  partySlotOne: ScannedPokemon | null;
  latestWild: ScannedPokemon | null;
  onIncrement: () => void;
  onRequestReset: () => void;
  onPressDiagnosticA: () => void;
  diagnosticStatus: string;
  onStartAutomaticHunt: () => void;
  onStopAutomaticHunt: () => void;
  automaticHuntRunning: boolean;
  wildHuntStatus: string;
  onStartAutomaticWildHunt: () => void;
  onStopAutomaticWildHunt: () => void;
  automaticWildHuntRunning: boolean;
  battleUiDiagnosticEnabled: boolean;
  battleUiDiagnosticSampleCount: number;
  onToggleBattleUiDiagnostic: () => void;
  starterTarget: StarterTarget;
  onStarterTargetChange: (target: StarterTarget) => void;
  scanning: boolean;
}) {
  const isStarter = mode === "starter";
  const title = isStarter ? "Starter Hunting" : "Wild Encounter Hunting";
  const description = isStarter
    ? "Soft resets are counted automatically while the scanner is running."
    : "New wild encounters are counted automatically while the scanner is running.";

  const [view, setView] = useState<"summary" | "counter">("summary");
  const attempts = session?.attempts ?? 0;
  const latestPokemon = isStarter ? partySlotOne : latestWild;

  return (
    <div className="page-content">
      <div className="page-heading">
        <div>
          <span className="eyebrow">SHINY HUNTING</span>
          <h2>{title}</h2>
          <p>{description}</p>
        </div>
        <span className="hunt-status active">
          <span />
          PAGE ACTIVE · COUNTING
        </span>
      </div>

      <div className="hunt-view-switcher" role="tablist" aria-label="Hunt view">
        <button
          className={view === "summary" ? "selected" : ""}
          onClick={() => setView("summary")}
        >
          Summary
        </button>
        <button
          className={view === "counter" ? "selected" : ""}
          onClick={() => setView("counter")}
        >
          Counter
        </button>
      </div>

      {isStarter && (
        <section className="starter-input-diagnostic">
          <div>
            <span className="eyebrow">INPUT DIAGNOSTIC</span>
            <p>Watch Emerald's game state, retry A until the chooser is active, then verify the selected starter before confirming.</p>
            <small>Checks Emerald's input memory after each press. Uses VBA's arrow keys and A binding.</small>
          </div>
          <label className="starter-target-control">
            <span>Starter target</span>
            <select value={starterTarget} onChange={(event) => onStarterTargetChange(event.target.value as StarterTarget)} disabled={automaticHuntRunning}>
              <option value="treecko">Treecko</option>
              <option value="mudkip">Mudkip</option>
              <option value="torchic">Torchic</option>
            </select>
          </label>
          <button
            className="secondary-button"
            onClick={onPressDiagnosticA}
            disabled={!scanning}
          >
            Send one A (test)
          </button>
          {!automaticHuntRunning ? (
            <button
              className="primary-button"
              onClick={onStartAutomaticHunt}
              disabled={!scanning}
            >
              Start automatic hunt
            </button>
          ) : (
            <button className="reset-button" onClick={onStopAutomaticHunt}>
              Stop automatic hunt
            </button>
          )}
          <strong className="starter-input-status">{diagnosticStatus}</strong>
        </section>
      )}

      {!isStarter && (
        <section className="starter-input-diagnostic wild-auto-hunt">
          <div>
            <span className="eyebrow">AUTOMATIC WILD HUNT</span>
            <p>Stand in grass and start the hunt. It starts the scanner if needed, turns in place with one-tick Left, Right, Up, and Down taps, runs from non-shiny encounters, and stops on a shiny.</p>
            <small>After selecting Run, it acknowledges “Got away safely!” once, then waits for the overworld before moving again.</small>
          </div>
          {!automaticWildHuntRunning ? (
            <button className="primary-button" onClick={onStartAutomaticWildHunt}>
              Start automatic wild hunt
            </button>
          ) : (
            <button className="reset-button" onClick={onStopAutomaticWildHunt}>
              Stop wild hunt
            </button>
          )}
          <strong className="starter-input-status">{wildHuntStatus}</strong>
          {import.meta.env.DEV && <div className="battle-ui-diagnostic-controls">
            <div>
              <span className="eyebrow">BATTLE UI DIAGNOSTICS</span>
            <small>Captures only when candidate memory signals change. State labels are provisional; no emulator input is sent.</small>
            </div>
            <button className="secondary-button" onClick={onToggleBattleUiDiagnostic} disabled={!scanning}>
              {battleUiDiagnosticEnabled ? "Stop capture" : "Start capture"}
            </button>
            <small>{battleUiDiagnosticSampleCount} state changes captured in memory for development.</small>
          </div>}
        </section>
      )}

      <section
        className={`hunt-summary-shell ${view === "counter" ? "counter-view" : ""}`}
      >
        <div className="hunt-summary-art">
          {latestPokemon ? (
            <PokemonSprite
              pokemon={latestPokemon}
              className="hunt-summary-sprite"
            />
          ) : (
            <div className="hunt-starter-art">{isStarter ? "✦" : "✦"}</div>
          )}
          <span className="summary-shiny">✦ SHINY HUNT</span>
          <div className="hunt-counter-chip">
            <span>{isStarter ? "RESETS" : "ENCOUNTERS"}</span>
            <strong>{attempts.toLocaleString()}</strong>
          </div>
        </div>

        <div className="hunt-summary-details">
          {latestPokemon ? (
            <PokemonDetails pokemon={latestPokemon} hideArt />
          ) : (
            <div className="hunt-empty-summary">
              <span className="eyebrow">SHINY HUNT SUMMARY</span>
              <h3>{isStarter ? "Starter Pokémon" : "Wild Pokémon"}</h3>
              <p>
                {isStarter
                  ? "Party slot 01 will appear here when detected."
                  : "The latest encounter will appear here after a wild battle."}
              </p>
            </div>
          )}
        </div>
        {/* Legacy duplicate hunt identity removed:
          <span className="eyebrow">SHINY HUNT SUMMARY</span>
          <h3>
            {latestPokemon?.speciesName ??
              (isStarter ? "Starter Pokémon" : "Wild Pokémon")}
          </h3>
          <p>
            {isStarter
              ? "Party slot 01 · starter resets"
              : "Latest wild encounter"}
          </p>

          <div className="hunt-summary-info">
            <div>
              <span>COUNTING</span>
              <strong>Only while this page is open</strong>
            </div>
            <div>
              <span>STATUS</span>
              <strong className="shiny-text">Active</strong>
            </div>
          </div>
        */}
      </section>

      {view === "counter" && (
        <section className="hunt-counter-panel">
          <div className="hunt-counter">
            <span>{isStarter ? "CURRENT RESETS" : "WILD ENCOUNTERS"}</span>
            <strong>{attempts.toLocaleString()}</strong>
            <small>Automatically counted while the scanner is running</small>
          </div>
          <div className="hunt-actions">
            {isStarter && (
              <button className="secondary-button" onClick={onIncrement}>
                +1 Manual Count
              </button>
            )}
            <button className="reset-button" onClick={onRequestReset}>
              ↻ Reset Counter
            </button>
          </div>
        </section>
      )}

      <section className="wild-latest-panel hunt-full-summary">
        <div className="section-heading">
          <div>
            <span className="eyebrow">
              {isStarter ? "PARTY SCANNER" : "WILD ENCOUNTER SCANNER"}
            </span>
            <h3>{isStarter ? "Party Pokémon" : "Latest Encounter"}</h3>
          </div>

          {latestPokemon && <span className="party-count">SLOT 01</span>}
        </div>

        {latestPokemon ? (
          <PokemonDetails
            pokemon={latestPokemon}
            huntCounter={attempts}
            huntCounterLabel={isStarter ? "RESETS" : "ENCOUNTERS"}
            onReset={onRequestReset}
          />
        ) : (
          <div className="empty-state">
            <div className="empty-icon">✦</div>
            <h3>
              {isStarter
                ? "Waiting for party data"
                : "Waiting for a wild Pokémon"}
            </h3>
            <p>
              {isStarter
                ? "Party slot 01 will appear here when a Pokémon is detected."
                : "Start the scanner and encounter a wild Pokémon in the selected game."}
            </p>
          </div>
        )}
      </section>
    </div>
  );
}

function App() {
  const [party, setParty] = useState<PartyUpdate>(EMPTY_PARTY);
  const [pcStorage, setPcStorage] = useState<PcStorageUpdate>(EMPTY_PC_STORAGE);
  const pcStorageRef = useRef(pcStorage);
  const [selectedPcBox, setSelectedPcBox] = useState(0);
  const [selectedStoredPokemon, setSelectedStoredPokemon] = useState<ScannedPokemon | null>(null);
  const [gameState, setGameState] = useState<GameStateUpdate>(EMPTY_GAME_STATE);
  const gameStateRef = useRef(gameState);
  const [inputDiagnosticStatus, setInputDiagnosticStatus] = useState("Scanner must be running.");
  const inputDiagnosticPendingRef = useRef(false);
  const [automaticHuntRunning, setAutomaticHuntRunning] = useState(false);
  const automaticHuntRunningRef = useRef(false);
  const [automaticWildHuntRunning, setAutomaticWildHuntRunning] = useState(false);
  const automaticWildHuntRunningRef = useRef(false);
  const [wildHuntStatus, setWildHuntStatus] = useState("Scanner must be running.");
  const [wildHuntWarning, setWildHuntWarning] = useState<string | null>(null);
  const [battleUiDiagnosticEnabled, setBattleUiDiagnosticEnabled] = useState(false);
  const [battleUiDiagnosticSamples, setBattleUiDiagnosticSamples] = useState<BattleUiDiagnosticSample[]>([]);
  const wildEncounterVersionRef = useRef(0);
  const [starterTarget, setStarterTarget] = useState<StarterTarget>("treecko");
  const partyRef = useRef(party);
  const [floatingSprites] = useState(getRandomFloatingSprites);
  const [latestStarter, setLatestStarter] = useState(() => loadLatestHuntPokemon().starter);
  const [latestWild, setLatestWild] = useState(() => loadLatestHuntPokemon().wild);
  const latestWildRef = useRef(latestWild);
  const [scanning, setScanning] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [page, setPage] = useState<Page>("dashboard");
  const [pageTransitioning, setPageTransitioning] = useState(false);
  const [selectedSlot, setSelectedSlot] = useState(0);
  const [hunts, setHunts] = useState<HuntStorage>(loadHunts);
  const [evTracks, setEvTracks] = useState<EvTrackStorage>(loadEvTracks);
  const [boxTheme, setBoxTheme] = useState(loadBoxTheme);
  const [selectedGame, setSelectedGame] = useState<GameId>(loadGame);
  const [showFirstLaunch, setShowFirstLaunch] = useState(
    () => !localStorage.getItem(GAME_STORAGE_KEY),
  );
  const [availableUpdate, setAvailableUpdate] = useState<Update | null>(null);
  const [updateState, setUpdateState] = useState<
    "checking" | "available" | "up-to-date" | "offline" | "installing"
  >("checking");
  const [showUpdateModal, setShowUpdateModal] = useState(false);
  const [updateProgress, setUpdateProgress] = useState(0);
  const dismissedUpdateVersionRef = useRef<string | null>(null);
  const updateCheckInFlightRef = useRef(false);
  const evTracksHydratedRef = useRef(false);
  const [resetMode, setResetMode] = useState<HuntMode | null>(null);
  const pageRef = useRef<Page>(page);

  function navigateTo(nextPage: Page) {
    if (nextPage === page || pageTransitioning) return;

    setPageTransitioning(true);

    // Change the page only after the dark cover has reached the center.
    window.setTimeout(() => setPage(nextPage), 350);
    window.setTimeout(() => setPageTransitioning(false), 760);
  }
  const scanningRef = useRef(false);

  useEffect(() => {
    pageRef.current = page;
  }, [page]);

  useEffect(() => {
    partyRef.current = party;
  }, [party]);

  useEffect(() => {
    gameStateRef.current = gameState;
  }, [gameState]);

  useEffect(() => {
    latestWildRef.current = latestWild;
  }, [latestWild]);

  useEffect(() => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(hunts));
  }, [hunts]);

  useEffect(() => {
    let active = true;
    void invoke<EvTrackStorage>("load_ev_targets")
      .then((stored) => {
        if (!active) return;
        setEvTracks((current) => ({ ...current, ...stored }));
      })
      .catch(() => undefined)
      .finally(() => {
        evTracksHydratedRef.current = true;
      });

    return () => {
      active = false;
    };
  }, []);

  useEffect(() => {
    localStorage.setItem(EV_STORAGE_KEY, JSON.stringify(evTracks));
    if (evTracksHydratedRef.current) {
      void invoke("save_ev_targets", { targets: evTracks }).catch(() => undefined);
    }
  }, [evTracks]);

  useEffect(() => {
    localStorage.setItem(
      HUNT_POKEMON_CACHE_KEY,
      JSON.stringify({ starter: latestStarter, wild: latestWild }),
    );
  }, [latestStarter, latestWild]);

  useEffect(() => {
    localStorage.setItem(BOX_THEME_STORAGE_KEY, String(boxTheme));
  }, [boxTheme]);

  useEffect(() => {
    localStorage.setItem(GAME_STORAGE_KEY, selectedGame);
  }, [selectedGame]);

  useEffect(() => {
    const checkOnStartup = window.setTimeout(() => {
      void checkForUpdates();
    }, 1200);
    const checkPeriodically = window.setInterval(() => {
      void checkForUpdates();
    }, UPDATE_CHECK_INTERVAL_MS);
    const checkWhenFocused = () => void checkForUpdates();
    window.addEventListener("focus", checkWhenFocused);

    return () => {
      window.clearTimeout(checkOnStartup);
      window.clearInterval(checkPeriodically);
      window.removeEventListener("focus", checkWhenFocused);
    };
    // checkForUpdates is intentionally invoked by startup, focus, and interval events.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    const preventAppReload = (event: KeyboardEvent) => {
      if (event.ctrlKey && event.key.toLowerCase() === "r") {
        event.preventDefault();
        event.stopPropagation();
      }
    };

    window.addEventListener("keydown", preventAppReload, true);

    return () => {
      window.removeEventListener("keydown", preventAppReload, true);
    };
  }, []);

  useEffect(() => {
    const playButtonSound = (event: MouseEvent) => {
      const target = event.target as HTMLElement | null;
      if (target?.closest("button")) playUiSound(buttonSoundUrl);
    };

    window.addEventListener("click", playButtonSound);
    return () => window.removeEventListener("click", playButtonSound);
  }, []);

  useEffect(() => {
    let disposed = false;
    let unlistenFunctions: (() => void)[] = [];

    async function registerListeners() {
      try {
        const listeners = await Promise.all([
          listen<PartyUpdate>("party-updated", (event) => {
            partyRef.current = event.payload;
            setParty(event.payload);
            const detectedStarter = event.payload.pokemon.find((pokemon) => pokemon.partySlot === 1);
            if (detectedStarter) setLatestStarter(detectedStarter);
            if (inputDiagnosticPendingRef.current && event.payload.pokemon.length > 0) {
              inputDiagnosticPendingRef.current = false;
              setInputDiagnosticStatus(
                `Party data detected: ${event.payload.pokemon[0].speciesName} in slot ${event.payload.pokemon[0].partySlot}.`,
              );
            }
            setError(null);
            setSelectedSlot((current) =>
              Math.min(current, Math.max(0, event.payload.pokemon.length - 1)),
            );
          }),
          listen<PcStorageUpdate>("pc-storage-updated", (event) => {
            const wasAvailable = pcStorageRef.current.available;
            pcStorageRef.current = event.payload;
            if (!wasAvailable && event.payload.currentBox !== null) {
              setSelectedPcBox(event.payload.currentBox);
            }
            setPcStorage(event.payload);
          }),
          listen<GameStateUpdate>("game-state-updated", (event) => {
            gameStateRef.current = event.payload;
            setGameState(event.payload);
            if (inputDiagnosticPendingRef.current) {
              setInputDiagnosticStatus(
                `Waiting for party data. Observed game state: ${event.payload.state.replaceAll("_", " ")}. ${event.payload.detail}`,
              );
            }
          }),
          listen<BattleUiDiagnosticSample>("battle-ui-diagnostic-sample", (event) => {
            setBattleUiDiagnosticSamples((current) => [...current.slice(-11999), event.payload]);
          }),
          listen<string>("scanner-error", (event) => setError(event.payload)),

           listen("scanner-stopped", () => {
             scanningRef.current = false;
             setScanning(false);
             setBattleUiDiagnosticEnabled(false);
           }),

          listen("starter-reset", () => {
            if (!scanningRef.current) return;
            setHunts((current) => {
              const session = current.starter ?? { attempts: 0 };

              return {
                ...current,
                starter: {
                  ...session,
                  attempts: session.attempts + 1,
                },
              };
            });
          }),

          listen<ScannedPokemon>("wild-encounter", (event) => {
            // A scan can finish just as Stop is pressed. Ignore that queued event.
            if (!scanningRef.current) return;
            const pokemon = event.payload;

            latestWildRef.current = pokemon;
            wildEncounterVersionRef.current += 1;
            setLatestWild(pokemon);

            setHunts((current) => {
              const session = current.wild ?? { attempts: 0 };

              return {
                ...current,
                wild: {
                  ...session,
                  attempts: session.attempts + 1,
                },
              };
            });
          }),
        ]);

        if (disposed) {
          listeners.forEach((unlisten) => unlisten());
          return;
        }

        unlistenFunctions = listeners;
      } catch (err) {
        if (!disposed)
          setError(`Failed to register scanner events: ${String(err)}`);
      }
    }

    registerListeners();

    return () => {
      disposed = true;
      unlistenFunctions.forEach((unlisten) => unlisten());
    };
  }, []);

  async function startScanning() {
    setError(null);
    const game = GAMES.find((candidate) => candidate.id === selectedGame);

    if (!game?.available) {
      setError(`${game?.name ?? "This game"} scanning is coming soon.`);
      return;
    }

    try {
      await invoke("start_scanner", { game: selectedGame });
      playUiSound(pcOnSoundUrl);
      scanningRef.current = true;
      setScanning(true);
    } catch (err) {
      setError(String(err));
    }
  }

  async function stopScanning() {
    setError(null);
    // Close the event gate before waiting for the worker to join.
    scanningRef.current = false;
    try {
      await invoke("stop_scanner");
      playUiSound(pcOffSoundUrl);
      setScanning(false);
    } catch (err) {
      scanningRef.current = true;
      setError(String(err));
    }
  }

  async function pressDiagnosticA() {
    if (!scanning) {
      setInputDiagnosticStatus("Start the scanner first.");
      return;
    }
    if (party.pokemon.length > 0) {
      setInputDiagnosticStatus("A party Pokémon is already present; clear the party before testing detection.");
      return;
    }

    setInputDiagnosticStatus("Focusing VBA and sending one A press...");
    try {
      const result = await invoke<string>("send_emulator_key", {
        key: "a",
        game: selectedGame,
        allowStarterSelectionA: true,
      });
      inputDiagnosticPendingRef.current = true;
      setInputDiagnosticStatus(`${result} Waiting for a Pokémon in party memory...`);
    } catch (inputError) {
      inputDiagnosticPendingRef.current = false;
      setInputDiagnosticStatus(String(inputError));
    }
  }

  async function startAutomaticHunt() {
    if (!scanning) {
      setInputDiagnosticStatus("Start the scanner before starting the automatic hunt.");
      return;
    }
    if (partyRef.current.pokemon.length > 0) {
      setInputDiagnosticStatus("The party must be empty before this starter hunt can begin.");
      return;
    }
    if (automaticHuntRunningRef.current || automaticWildHuntRunningRef.current) return;
    if (selectedGame !== "emerald") {
      setInputDiagnosticStatus("State-driven starter selection is currently mapped only for US Pokémon Emerald.");
      return;
    }
    const confirmed = window.confirm(
      `Start automatic ${STARTER_NAME[starterTarget]} hunting? This will immediately soft reset the emulator, then keep soft resetting after each non-shiny starter until you stop the hunt or a shiny is found.`,
    );
    if (!confirmed) {
      setInputDiagnosticStatus("Automatic hunt cancelled. No reset was sent.");
      return;
    }

    automaticHuntRunningRef.current = true;
    setAutomaticHuntRunning(true);
    const wait = (milliseconds: number) =>
      new Promise<void>((resolve) => window.setTimeout(resolve, milliseconds));
    let lastInputSentAt = 0;
    const sendKey = async (
      key: string,
      shouldSkip?: () => boolean,
      allowStarterSelectionA = false,
    ): Promise<string | null> => {
      while (Date.now() - lastInputSentAt < 100) {
        if (!automaticHuntRunningRef.current || shouldSkip?.()) return null;
        await wait(50);
      }
      if (!automaticHuntRunningRef.current || shouldSkip?.()) return null;
      const result = await invoke<string>("send_emulator_key", {
        key,
        game: selectedGame,
        allowStarterSelectionA,
      });
      lastInputSentAt = Date.now();
      return result;
    };

    try {
      let attempt = 0;
      while (automaticHuntRunningRef.current) {
        if (selectedGame !== "emerald") {
          throw new Error("State-driven starter selection is currently mapped only for US Pokémon Emerald.");
        }
        attempt += 1;
        setInputDiagnosticStatus(`Attempt ${attempt}: sending Ctrl+R soft reset...`);
        await sendKey("soft_reset");
        setInputDiagnosticStatus(`Attempt ${attempt}: waiting 1.5 seconds after reset...`);
        await wait(1500);

        const partyClearDeadline = Date.now() + 10000;
        while (
          automaticHuntRunningRef.current &&
          partyRef.current.partyCount !== 0 &&
          Date.now() < partyClearDeadline
        ) {
          await wait(100);
        }
        if (automaticHuntRunningRef.current && partyRef.current.partyCount !== 0) {
          throw new Error("Party memory did not clear after soft reset; stopped before sending starter inputs.");
        }

        if (!automaticHuntRunningRef.current) break;
        setInputDiagnosticStatus(`Attempt ${attempt}: tapping A until Emerald reports the starter-choice task...`);
        const selectionDeadline = Date.now() + 90000;
        while (automaticHuntRunningRef.current && !gameStateRef.current.starterSelectionReady) {
          if (gameStateRef.current.starterConfirmationReady) {
            throw new Error("Emerald reached the YES/NO prompt before the target was verified. Stopped without pressing A again.");
          }
          const ack = await sendKey("a", () =>
            gameStateRef.current.starterSelectionReady || gameStateRef.current.starterConfirmationReady,
          );
          if (ack === null) {
            if (gameStateRef.current.starterConfirmationReady) {
              throw new Error("Emerald reached the YES/NO prompt before the target was verified. Stopped without pressing A again.");
            }
            continue;
          }
          setInputDiagnosticStatus(
            `Attempt ${attempt}: ${ack} Waiting for the starter-choice state...`,
          );
          const nextStateDeadline = Date.now() + 150;
          while (
            automaticHuntRunningRef.current &&
            !gameStateRef.current.starterSelectionReady &&
            Date.now() < nextStateDeadline
          ) {
            if (gameStateRef.current.starterConfirmationReady) {
              throw new Error("Emerald reached the YES/NO prompt before the target was verified. Stopped without pressing A again.");
            }
            await wait(50);
          }
          if (Date.now() >= selectionDeadline) break;
        }
        if (!automaticHuntRunningRef.current) break;
        if (!gameStateRef.current.starterSelectionReady) {
          throw new Error("Emerald starter-selection state was not detected within 90 seconds.");
        }

        const targetIndex = STARTER_INDEX[starterTarget];
        const targetName = STARTER_NAME[starterTarget];
        const moveDeadline = Date.now() + 10000;
        let moves = 0;
        while (automaticHuntRunningRef.current && Date.now() < moveDeadline) {
          const current = gameStateRef.current.emeraldStarterSelection;
          if (current === null || current < 0 || current > 2) {
            await wait(100);
            continue;
          }
          if (current === targetIndex) {
            if (gameStateRef.current.starterSelectionReady) break;
            await wait(100);
            continue;
          }
          if (!gameStateRef.current.starterSelectionReady) {
            await wait(100);
            continue;
          }
          const key = current > targetIndex ? "left" : "right";
          setInputDiagnosticStatus(`Attempt ${attempt}: memory shows starter ${current + 1}/3; sending ${key} toward ${targetName}...`);
          const inputResult = await sendKey(key, () =>
            !gameStateRef.current.starterSelectionReady ||
            gameStateRef.current.emeraldStarterSelection !== current,
          );
          if (inputResult === null) continue;
          if (!inputResult.includes("Emerald acknowledged the input")) {
            throw new Error(`${inputResult} Stopped before confirming ${targetName}.`);
          }
          moves += 1;
          const changeDeadline = Date.now() + 2500;
          while (automaticHuntRunningRef.current && Date.now() < changeDeadline) {
            const selection = gameStateRef.current.emeraldStarterSelection;
            if (selection !== null && selection !== current) break;
            await wait(50);
          }
          if (gameStateRef.current.emeraldStarterSelection === current && moves >= 4) {
            throw new Error(`Sent ${key} four times, but Emerald's selected-starter memory did not change. Stopped without confirming.`);
          }
        }
        if (!automaticHuntRunningRef.current) break;
        if (gameStateRef.current.emeraldStarterSelection !== targetIndex || !gameStateRef.current.starterSelectionReady) {
          throw new Error(`Could not verify ${targetName} in Emerald's chooser memory. Stopped without confirming.`);
        }
        setInputDiagnosticStatus(`Attempt ${attempt}: memory confirms ${targetName}; opening its confirmation prompt...`);
        const openConfirmationResult = await sendKey(
          "a",
          () =>
            !gameStateRef.current.starterSelectionReady ||
            gameStateRef.current.emeraldStarterSelection !== targetIndex,
          true,
        );
        if (openConfirmationResult === null) {
          throw new Error(`Selection changed while waiting to confirm ${targetName}; stopped without sending A.`);
        }
        const confirmationDeadline = Date.now() + 10000;
        while (
          automaticHuntRunningRef.current &&
          !gameStateRef.current.starterConfirmationReady &&
          Date.now() < confirmationDeadline
        ) {
          await wait(50);
        }
        if (!automaticHuntRunningRef.current) break;
        if (!gameStateRef.current.starterConfirmationReady) {
          throw new Error(`${targetName} confirmation state was not detected after the first A press.`);
        }
        if (gameStateRef.current.emeraldStarterSelection !== targetIndex) {
          throw new Error(`Selection changed before confirmation; expected ${targetName}. Stopped without accepting it.`);
        }
        const confirmResult = await sendKey(
          "a",
          () =>
            !gameStateRef.current.starterConfirmationReady ||
            gameStateRef.current.emeraldStarterSelection !== targetIndex,
          true,
        );
        if (confirmResult === null) {
          throw new Error(`Confirmation state changed before accepting ${targetName}; stopped without sending A.`);
        }

        setInputDiagnosticStatus(`Attempt ${attempt}: waiting for party slot 1...`);
        const deadline = Date.now() + 90000;
        while (automaticHuntRunningRef.current && Date.now() < deadline) {
          const starter = partyRef.current.pokemon.find((pokemon) => pokemon.partySlot === 1);
          if (starter) {
            if (starter.shiny) {
              setInputDiagnosticStatus(`Shiny ${starter.speciesName} detected in party slot 1! Automatic hunt stopped.`);
              automaticHuntRunningRef.current = false;
              break;
            }
            setInputDiagnosticStatus(`${starter.speciesName} detected in party slot 1 (not shiny). Resetting for the next attempt...`);
            break;
          }
          await wait(250);
        }

        if (!automaticHuntRunningRef.current) break;
        if (Date.now() >= deadline && !partyRef.current.pokemon.some((pokemon) => pokemon.partySlot === 1)) {
          throw new Error("No Pokémon appeared in party slot 1 within 90 seconds; stopped to avoid continuing blindly.");
        }
      }
    } catch (huntError) {
      setInputDiagnosticStatus(`Automatic hunt stopped: ${String(huntError)}`);
    } finally {
      automaticHuntRunningRef.current = false;
      setAutomaticHuntRunning(false);
    }
  }

  async function startAutomaticWildHunt() {
    if (automaticWildHuntRunningRef.current || automaticHuntRunningRef.current) return;
    setWildHuntWarning(null);
    if (selectedGame !== "emerald") {
      setWildHuntStatus("Battle action-menu detection is currently mapped only for US Pokémon Emerald.");
      return;
    }
    automaticWildHuntRunningRef.current = true;
    setAutomaticWildHuntRunning(true);
    const wait = (milliseconds: number) =>
      new Promise<void>((resolve) => window.setTimeout(resolve, milliseconds));
    let lastInputSentAt = 0;
    const sendKey = async (key: string): Promise<string | null> => {
      while (Date.now() - lastInputSentAt < 100) {
        if (!automaticWildHuntRunningRef.current) return null;
        await wait(20);
      }
      if (!automaticWildHuntRunningRef.current) return null;
      const result = await invoke<string>("send_emulator_key", {
        key,
        game: selectedGame,
        allowStarterSelectionA: false,
      });
      lastInputSentAt = Date.now();
      return result;
    };
    const sendBattleKey = async (key: string) => {
      const result = await sendKey(key);
      if (result === null) return false;
      if (key !== "a" && selectedGame === "emerald" && result.includes("no Emerald input acknowledgment")) {
        throw new Error(`${result} Stopped the hunt before continuing.`);
      }
      return true;
    };
    const completePostCapture = async (status: string) => {
      setWildHuntStatus(status);
      const deadline = Date.now() + 90000;
      let dexInputTaskId: number | null = null;
      let dexInputDeadline = 0;
      let nicknameDeclineSent = false;
      let nicknameDeclineDeadline = 0;
      let nicknameDeclined = false;
      let textAdvanceSent = false;
      let textAdvanceId: number | null = null;
      let textAdvanceDeadline = 0;
      let textAdvanceLastAttemptAt = 0;
      let textAdvanceAttempts = 0;
      let overworldSince: number | null = null;
      while (automaticWildHuntRunningRef.current && Date.now() < deadline) {
        const state = gameStateRef.current;
        const communication = state.emeraldBattleCommunication;
        const communicationState = communication?.[0] ?? null;

        if (state.emeraldNamingScreenActive) {
          throw new Error("Emerald opened the nickname editor. No input was sent; stopped to avoid editing the nickname.");
        }

        if (state.emeraldOverworldReady && !state.battleTextReady) {
          overworldSince ??= Date.now();
          if (Date.now() - overworldSince >= 500) return true;
        } else {
          overworldSince = null;
        }

        if (state.battleActionMenuReady && !state.emeraldCatchConfirmed) {
          throw new Error("The battle action menu appeared before a post-capture state was confirmed. Stopped without sending a menu input.");
        }

        if (state.emeraldDexPageInputReady) {
          const taskId = state.emeraldDexTaskId;
          if (taskId === null) {
            throw new Error("Pokédex page input state was detected without a task ID. Stopped without sending a key.");
          }
          if (dexInputTaskId === null) {
            setWildHuntStatus("Pokédex page input task verified; sending one B (X) to close the registration page...");
            if (!(await sendBattleKey("b"))) return false;
            dexInputTaskId = taskId;
            dexInputDeadline = Date.now() + 8000;
          } else if (Date.now() >= dexInputDeadline) {
            throw new Error("B was sent to the verified Pokédex page task, but its input handler did not change. Stopped without repeating the key.");
          }
          await wait(50);
          continue;
        }

        if (nicknameDeclineSent) {
          if (communicationState === 4) {
            nicknameDeclined = true;
          } else if (Date.now() >= nicknameDeclineDeadline && !nicknameDeclined) {
            throw new Error("B was sent at the verified nickname decision, but Emerald did not enter the decline state. Stopped without repeating the key.");
          }
        } else if (state.emeraldNicknamePromptReady) {
          setWildHuntStatus("Nickname Yes/No decision verified; sending one B (X) to decline. A will not be sent.");
          if (!(await sendBattleKey("b"))) return false;
          nicknameDeclineSent = true;
          nicknameDeclineDeadline = Date.now() + 8000;
          await wait(50);
          continue;
        }

        if (state.battleTextReady && state.emeraldCatchConfirmed) {
          const textId = state.emeraldBattleTextStringId;
          if (textId === null) {
            throw new Error("Post-capture text is active, but its battle string ID is unreadable. Stopped without sending a key.");
          }
          const textChanged = textAdvanceId !== null && textAdvanceId !== textId;
          const retryDue = textAdvanceSent &&
            textAdvanceId === textId &&
            Date.now() - textAdvanceLastAttemptAt >= 1500;
          if (!textAdvanceSent || textChanged || retryDue) {
            if (textChanged || textAdvanceId !== textId) textAdvanceAttempts = 0;
            if (textAdvanceAttempts >= 4) {
              throw new Error(`Emerald remains on verified post-capture text ${textId} after four B inputs. Hunt stopped without further input.`);
            }
            textAdvanceAttempts += 1;
            setWildHuntStatus(
              textAdvanceAttempts === 1
                ? "Post-capture battle text verified; sending B (X)..."
                : `Emerald still shows the same verified message; retrying B (X) (${textAdvanceAttempts}/4)...`,
            );
            if (!(await sendBattleKey("b"))) return false;
            textAdvanceSent = true;
            textAdvanceId = textId;
            textAdvanceLastAttemptAt = Date.now();
            textAdvanceDeadline = Date.now() + 8000;
          } else if (Date.now() >= textAdvanceDeadline) {
            throw new Error(`B was sent to verified post-capture text ${textId}, but its state did not change. Hunt stopped without further input.`);
          }
        } else if (!state.battleTextReady) {
          textAdvanceSent = false;
          textAdvanceId = null;
          textAdvanceAttempts = 0;
        } else if (
          textAdvanceSent &&
          state.emeraldBattleTextStringId !== textAdvanceId
        ) {
          textAdvanceSent = false;
        } else if (textAdvanceSent && Date.now() >= textAdvanceDeadline) {
          throw new Error("B was sent to verified post-capture text, but the text state did not change. Stopped without repeating the key.");
        }

        if (state.emeraldDexRegistrationReady || communicationState === 2 || nicknameDeclined) {
          setWildHuntStatus(
            state.emeraldDexRegistrationReady
              ? "Waiting for the verified Pokédex task to finish..."
              : nicknameDeclined
                ? "Nickname declined; waiting for Emerald's capture script to finish..."
                : "Pokédex registration transition detected; waiting for the next verified capture state...",
          );
          await wait(50);
          continue;
        }

        if (state.emeraldCatchConfirmed || state.emeraldNicknamePromptReady) {
          await wait(50);
          continue;
        }

        throw new Error(
          `Post-capture state became ambiguous; stopped without more input. outcome=${state.emeraldBattleOutcome}, comm=${JSON.stringify(communication)}, caughtSpecies=${state.emeraldCaughtSpecies}, wildSpecies=${state.emeraldWildSpecies}, callback2=0x${state.emeraldCallback2.toString(16)}, inBattle=${state.emeraldInBattle}`,
        );
      }
      if (!automaticWildHuntRunningRef.current) return false;
      throw new Error("The post-capture sequence did not reach a verified overworld state within 90 seconds. Hunt stopped.");
    };
    const sendTurnKey = async (key: string) => {
      while (Date.now() - lastInputSentAt < 100) {
        if (!automaticWildHuntRunningRef.current) return false;
        await wait(20);
      }
      if (!automaticWildHuntRunningRef.current) return false;
      const result = await invoke<string>("send_emulator_turn", { key, game: selectedGame });
      lastInputSentAt = Date.now();
      if (result.includes("did not acknowledge")) {
        throw new Error(`${result} Stopped the hunt before continuing.`);
      }
      return true;
    };

    try {
      if (!scanningRef.current) {
        setWildHuntStatus("Starting the game scanner...");
        gameStateRef.current = EMPTY_GAME_STATE;
        setGameState(EMPTY_GAME_STATE);
        await invoke("start_scanner", { game: selectedGame });
        scanningRef.current = true;
        setScanning(true);
      }
      const connectionDeadline = Date.now() + 10000;
      while (
        automaticWildHuntRunningRef.current &&
        !gameStateRef.current.connected &&
        Date.now() < connectionDeadline
      ) await wait(50);
      if (!automaticWildHuntRunningRef.current) return;
      if (!gameStateRef.current.connected) {
        throw new Error("The scanner could not connect to VBA. Open the Emerald game and try Start again.");
      }
      const ballCountDeadline = Date.now() + 5000;
      while (
        automaticWildHuntRunningRef.current &&
        gameStateRef.current.emeraldPokeballCount === null &&
        Date.now() < ballCountDeadline
      ) await wait(50);
      if (!automaticWildHuntRunningRef.current) return;
      if (gameStateRef.current.emeraldPokeballCount === null) {
        throw new Error("Poké Ball inventory could not be read. Automatic capture stopped before movement.");
      }
      const recoveringCapture =
        gameStateRef.current.emeraldCatchConfirmed ||
        gameStateRef.current.emeraldDexRegistrationReady ||
        gameStateRef.current.emeraldNicknamePromptReady ||
        gameStateRef.current.emeraldNamingScreenActive;
      if (gameStateRef.current.emeraldPokeballCount === 0 && !recoveringCapture) {
        setWildHuntStatus("No Poké Balls are available. Automatic wild hunt stopped.");
        setWildHuntWarning("The automatic wild hunt stopped because your Bag has no Poké Balls left.");
        automaticWildHuntRunningRef.current = false;
        return;
      }
      if (recoveringCapture) {
        const recovered = await completePostCapture("Post-capture screen detected; identifying each screen before sending B (X)...");
        if (!recovered || !automaticWildHuntRunningRef.current) return;
        if (gameStateRef.current.emeraldPokeballCount === 0) {
          setWildHuntStatus("Post-catch screens cleared. No balls remain; automatic wild hunt stopped.");
          setWildHuntWarning("The post-catch screens were cleared, but your Bag has no Poké Balls left.");
          automaticWildHuntRunningRef.current = false;
          return;
        }
      } else if (gameStateRef.current.emeraldInBattle !== false || gameStateRef.current.battleActionMenuReady) {
        throw new Error("Finish the current battle before starting the automatic wild hunt.");
      }

      const turnDirections = ["left", "right", "up", "down"] as const;
      let turnDirectionIndex = 0;
      let pendingRunPokemon: ScannedPokemon | null = null;
      let awaitingBattleMenu = false;
      let handledEncounterVersion = wildEncounterVersionRef.current;
      while (automaticWildHuntRunningRef.current) {
        if (gameStateRef.current.emeraldPokeballCount === 0) {
          setWildHuntStatus("No Poké Balls remain. Automatic wild hunt stopped.");
          setWildHuntWarning("The automatic wild hunt stopped because your Bag has no Poké Balls left.");
          automaticWildHuntRunningRef.current = false;
          break;
        }
        if (wildEncounterVersionRef.current > handledEncounterVersion) {
          handledEncounterVersion = wildEncounterVersionRef.current;
          const encounteredPokemon = latestWildRef.current;
          if (!encounteredPokemon) {
            throw new Error("Wild encounter data is unavailable; stopped before choosing a battle action.");
          }
          const ballCount = gameStateRef.current.emeraldPokeballCount;
          if (ballCount === null) {
            setWildHuntStatus(`Wild ${encounteredPokemon.speciesName} detected, but Poké Ball inventory could not be verified. Hunt stopped; battle left untouched.`);
            automaticWildHuntRunningRef.current = false;
            break;
          }
          if (ballCount === 0) {
            setWildHuntStatus(`Wild ${encounteredPokemon.speciesName} detected, but no Poké Balls remain. Hunt stopped; battle left untouched.`);
            setWildHuntWarning("A wild encounter started after your last Poké Ball was used. The hunt stopped without choosing a battle action.");
            automaticWildHuntRunningRef.current = false;
            break;
          }
          const encounteredShiny = encounteredPokemon.shinyValue < 8;
          setWildHuntStatus(
            encounteredShiny
              ? `Shiny ${encounteredPokemon.speciesName} detected (SV ${encounteredPokemon.shinyValue}/65,536) with ${ballCount} Poké Ball${ballCount === 1 ? "" : "s"}; preparing an automatic capture...`
              : `${encounteredPokemon.speciesName} has SV ${encounteredPokemon.shinyValue}/65,536 and is not shiny; it will be run from.`,
          );
          pendingRunPokemon = encounteredPokemon;
          awaitingBattleMenu = true;
          setWildHuntStatus(
            `${encounteredShiny ? "Shiny" : "Non-shiny"} ${encounteredPokemon.speciesName} (SV ${encounteredPokemon.shinyValue}/65,536); waiting for Emerald's intro text state...`,
          );
          const introDeadline = Date.now() + 20000;
          while (
            automaticWildHuntRunningRef.current &&
            !gameStateRef.current.wildAppearedTextReady &&
            !gameStateRef.current.battleActionMenuReady &&
            Date.now() < introDeadline
          ) await wait(50);
          if (!automaticWildHuntRunningRef.current) break;
          if (!gameStateRef.current.battleActionMenuReady) {
            if (!gameStateRef.current.wildAppearedTextReady) {
              throw new Error("Emerald did not expose the wild intro text state; stopped without sending A.");
            }
            let introAcknowledged = false;
            for (let attempt = 1; attempt <= 5; attempt += 1) {
              if (!automaticWildHuntRunningRef.current) break;
              if (gameStateRef.current.battleActionMenuReady) {
                introAcknowledged = true;
                break;
              }
              if (!gameStateRef.current.wildAppearedTextReady) {
                // Intro transitioned to another battle animation/text state;
                // wait for the command menu without sending more A presses.
                introAcknowledged = true;
                break;
              }
              setWildHuntStatus(`Wild appearance text detected for ${encounteredPokemon.speciesName}; sending A (${attempt}/5)...`);
              if (!(await sendBattleKey("a"))) break;
              const introInputDeadline = Date.now() + 350;
              while (
                automaticWildHuntRunningRef.current &&
                !gameStateRef.current.battleActionMenuReady &&
                gameStateRef.current.wildAppearedTextReady &&
                Date.now() < introInputDeadline
              ) await wait(50);
            }
            if (!automaticWildHuntRunningRef.current) break;
            if (!introAcknowledged && !gameStateRef.current.battleActionMenuReady && gameStateRef.current.wildAppearedTextReady) {
              throw new Error("Emerald kept the wild intro text active after five A presses. Hunt stopped before the battle menu.");
            }
          }
          const transitionDeadline = Date.now() + 10000;
          while (
            automaticWildHuntRunningRef.current &&
            !gameStateRef.current.battleActionMenuReady &&
            Date.now() < transitionDeadline
          ) await wait(50);
          if (automaticWildHuntRunningRef.current && !gameStateRef.current.battleActionMenuReady) {
            throw new Error("The action menu did not appear after the detected intro text. Stopped without sending another A or selecting Fight.");
          }
        }

        if (gameStateRef.current.battleActionMenuReady) {
          if (!pendingRunPokemon) {
            setWildHuntStatus("Battle action menu detected; waiting for this encounter's Pokémon data...");
            await wait(50);
            continue;
          }

          let cursor: number | null = gameStateRef.current.battleActionMenuCursor;
          if (cursor === null || cursor < 0 || cursor > 3) {
            throw new Error("The battle action menu is active, but its cursor could not be read.");
          }

          if (pendingRunPokemon.shinyValue < 8) {
            const ballsBeforeThrow = gameStateRef.current.emeraldPokeballCount;
            if (ballsBeforeThrow === null) {
              throw new Error("Poké Ball inventory became unreadable. Stopped before opening the Bag.");
            }
            if (ballsBeforeThrow === 0) {
              setWildHuntStatus("No Poké Balls remain. Automatic wild hunt stopped.");
              setWildHuntWarning("The automatic wild hunt stopped because your Bag has no Poké Balls left.");
              automaticWildHuntRunningRef.current = false;
              break;
            }

            // Battle action menu layout is Fight / Bag on the first row and
            // Pokémon / Run on the second row. Move one verified step at a time.
            while (automaticWildHuntRunningRef.current && cursor !== 1) {
              const previousCursor: number = cursor;
              const key = cursor === 0 ? "right" : cursor === 1 ? "left" : cursor === 2 ? "up" : "up";
              setWildHuntStatus(`${pendingRunPokemon.speciesName}: moving the battle menu to Bag (${previousCursor} → 1)...`);
              if (!(await sendBattleKey(key))) break;
              const cursorDeadline = Date.now() + 3000;
              while (automaticWildHuntRunningRef.current && Date.now() < cursorDeadline) {
                const nextCursor = gameStateRef.current.battleActionMenuCursor;
                if (gameStateRef.current.battleActionMenuReady && nextCursor !== null && nextCursor !== previousCursor) {
                  cursor = nextCursor;
                  break;
                }
                await wait(50);
              }
              if (cursor === previousCursor) {
                throw new Error(`Sent ${key}, but the battle menu did not move toward Bag. Stopped without selecting an action.`);
              }
            }
            if (!automaticWildHuntRunningRef.current) break;
            if (!gameStateRef.current.battleActionMenuReady || gameStateRef.current.battleActionMenuCursor !== 1) {
              throw new Error("Could not verify Bag in the battle menu. Stopped before opening it.");
            }

            setWildHuntStatus(`Bag selected for ${pendingRunPokemon.speciesName}; opening Bag...`);
            const battleCallbackBeforeBag = gameStateRef.current.emeraldCallback2;
            if (!(await sendBattleKey("a"))) break;
            const bagOpenDeadline = Date.now() + 10000;
            while (
              automaticWildHuntRunningRef.current &&
              (!gameStateRef.current.emeraldBagMenuReady ||
                gameStateRef.current.emeraldCallback2 === battleCallbackBeforeBag) &&
              Date.now() < bagOpenDeadline
            ) await wait(50);
            if (!automaticWildHuntRunningRef.current) break;
            if (
              !gameStateRef.current.emeraldBagMenuReady ||
              gameStateRef.current.emeraldCallback2 === battleCallbackBeforeBag ||
              gameStateRef.current.emeraldBagPocket === null
            ) {
              throw new Error("Could not verify Emerald's Bag screen and active pocket. Stopped without item inputs.");
            }

            // Pocket indices in Emerald are Items=0, Balls=1, TM/HM=2,
            // Berries=3, and Key Items=4. Navigate from the detected pocket.
            while (
              automaticWildHuntRunningRef.current &&
              gameStateRef.current.emeraldBagPocket !== 1
            ) {
              const detectedPocket = gameStateRef.current.emeraldBagPocket;
              if (detectedPocket === null || detectedPocket < 0 || detectedPocket > 4) {
                throw new Error("Emerald's active Bag pocket became unreadable. Stopped before choosing an item.");
              }
              const pocket: number = detectedPocket;
              const rightDistance = (1 - pocket + 5) % 5;
              const leftDistance = (pocket - 1 + 5) % 5;
              const key = rightDistance <= leftDistance ? "right" : "left";
              setWildHuntStatus(`Bag pocket ${pocket}/4 detected; switching to Poké Balls...`);
              if (!(await sendBattleKey(key))) break;
              const pocketDeadline = Date.now() + 3000;
              while (
                automaticWildHuntRunningRef.current &&
                gameStateRef.current.emeraldBagMenuReady &&
                gameStateRef.current.emeraldBagPocket === pocket &&
                Date.now() < pocketDeadline
              ) await wait(50);
              if (gameStateRef.current.emeraldBagPocket === pocket) {
                throw new Error(`Sent ${key}, but Emerald's Bag pocket did not change from ${pocket}. Stopped before choosing an item.`);
              }
            }
            if (!automaticWildHuntRunningRef.current) break;
            if (!gameStateRef.current.emeraldBagMenuReady || gameStateRef.current.emeraldBagPocket !== 1) {
              throw new Error("Could not verify the Poké Ball pocket. Stopped before choosing an item.");
            }

            let bagCursor = gameStateRef.current.emeraldBagCursor;
            if (bagCursor === null || bagCursor < 0 || bagCursor > 64) {
              throw new Error("Poké Ball list cursor could not be read. Stopped before choosing an item.");
            }
            while (automaticWildHuntRunningRef.current && bagCursor > 0) {
              const previousCursor: number = bagCursor;
              setWildHuntStatus(`Poké Ball pocket confirmed; moving its item cursor ${previousCursor} to the first row...`);
              if (!(await sendBattleKey("up"))) break;
              const cursorDeadline = Date.now() + 3000;
              while (
                automaticWildHuntRunningRef.current &&
                gameStateRef.current.emeraldBagMenuReady &&
                gameStateRef.current.emeraldBagPocket === 1 &&
                gameStateRef.current.emeraldBagCursor === previousCursor &&
                Date.now() < cursorDeadline
              ) await wait(50);
              bagCursor = gameStateRef.current.emeraldBagCursor;
              if (bagCursor === previousCursor) {
                throw new Error("Sent Up, but the Poké Ball list cursor did not move. Stopped before selecting an item.");
              }
              if (bagCursor === null || bagCursor < 0 || bagCursor > 64) {
                throw new Error("Poké Ball list cursor became unreadable. Stopped before selecting an item.");
              }
            }
            if (!automaticWildHuntRunningRef.current) break;
            if (
              !gameStateRef.current.emeraldBagMenuReady ||
              gameStateRef.current.emeraldBagPocket !== 1 ||
              gameStateRef.current.emeraldBagCursor !== 0
            ) {
              throw new Error("Poké Ball pocket and first-row cursor were not confirmed. Stopped before choosing an item.");
            }

            setWildHuntStatus("Poké Ball pocket and first item row confirmed; opening its Use menu...");
            await wait(250);
            if (!(await sendBattleKey("a"))) break;
            await wait(350);
            if (!(await sendBattleKey("a"))) break;
            if (!automaticWildHuntRunningRef.current) break;
            setWildHuntStatus(`Throwing a Poké Ball at ${pendingRunPokemon.speciesName}...`);

            const throwDeadline = Date.now() + 20000;
            while (
              automaticWildHuntRunningRef.current &&
              (gameStateRef.current.emeraldPokeballCount === null ||
                gameStateRef.current.emeraldPokeballCount >= ballsBeforeThrow) &&
              Date.now() < throwDeadline
            ) await wait(50);
            if (!automaticWildHuntRunningRef.current) break;
            const ballsAfterThrow = gameStateRef.current.emeraldPokeballCount;
            if (ballsAfterThrow === null || ballsAfterThrow >= ballsBeforeThrow) {
              throw new Error("Emerald did not confirm a Poké Ball was used. Stopped without sending more inputs.");
            }

            const outcomeDeadline = Date.now() + 30000;
            setWildHuntStatus(`Poké Ball used (${ballsAfterThrow} remain); identifying Emerald's result...`);
            let acknowledgedTextId: number | null = null;
            let textAcknowledgementDeadline = 0;
            while (
              automaticWildHuntRunningRef.current &&
              !gameStateRef.current.battleActionMenuReady &&
              !gameStateRef.current.emeraldCatchConfirmed &&
              !gameStateRef.current.emeraldDexRegistrationReady &&
              !gameStateRef.current.emeraldNicknamePromptReady &&
              !gameStateRef.current.emeraldNamingScreenActive &&
              Date.now() < outcomeDeadline
            ) {
              const state = gameStateRef.current;
              if (state.battleTextReady) {
                const textId = state.emeraldBattleTextStringId;
                if (textId === null) {
                  throw new Error("Emerald result text is active, but its string ID is unreadable. Stopped without sending a key.");
                }
                if (acknowledgedTextId === null || textId !== acknowledgedTextId) {
                  setWildHuntStatus(`Emerald result text ${textId} verified; sending one B (X)...`);
                  if (!(await sendBattleKey("b"))) break;
                  acknowledgedTextId = textId;
                  textAcknowledgementDeadline = Date.now() + 8000;
                } else if (Date.now() >= textAcknowledgementDeadline) {
                  throw new Error(`B was sent for Emerald result text ${textId}, but the text did not change. Stopped without repeating the key.`);
                }
              } else {
                acknowledgedTextId = null;
              }
              await wait(50);
            }
            if (!automaticWildHuntRunningRef.current) break;
            if (
              gameStateRef.current.emeraldCatchConfirmed ||
              gameStateRef.current.emeraldDexRegistrationReady ||
              gameStateRef.current.emeraldNicknamePromptReady ||
              gameStateRef.current.emeraldNamingScreenActive
            ) {
              const caughtSpeciesName = pendingRunPokemon.speciesName;
              const recovered = await completePostCapture(`${caughtSpeciesName} caught; handling confirmed post-capture screens...`);
              if (!recovered || !automaticWildHuntRunningRef.current) break;
              if (gameStateRef.current.emeraldPokeballCount === 0) {
                setWildHuntStatus(`${caughtSpeciesName} caught. No Poké Balls remain; automatic wild hunt stopped.`);
                setWildHuntWarning(`${caughtSpeciesName} was caught, but your last Poké Ball was used. The automatic wild hunt has stopped.`);
                automaticWildHuntRunningRef.current = false;
                break;
              }
              pendingRunPokemon = null;
              awaitingBattleMenu = false;
              setWildHuntStatus(`${caughtSpeciesName} caught; overworld confirmed after B inputs, resuming wild encounters.`);
              await wait(500);
              continue;
            }
            if (!gameStateRef.current.battleActionMenuReady) {
              throw new Error("Poké Ball was used, but Emerald did not return to a recognized battle state. Hunt stopped.");
            }
            setWildHuntStatus(`The shiny broke free; ${ballsAfterThrow} Poké Ball${ballsAfterThrow === 1 ? "" : "s"} remain. Trying again...`);
            continue;
          }

          while (automaticWildHuntRunningRef.current && cursor !== 3) {
            const previousCursor: number = cursor;
            const key = (cursor & 2) === 0 ? "down" : "right";
            setWildHuntStatus(`${pendingRunPokemon.speciesName} is not shiny; menu cursor ${cursor}/3, sending ${key} toward Run...`);
            if (!(await sendBattleKey(key))) break;
            const cursorDeadline = Date.now() + 3000;
            while (automaticWildHuntRunningRef.current && Date.now() < cursorDeadline) {
              const nextCursor = gameStateRef.current.battleActionMenuCursor;
              if (gameStateRef.current.battleActionMenuReady && nextCursor !== null && nextCursor !== previousCursor) {
                cursor = nextCursor;
                break;
              }
              await wait(50);
            }
            if (cursor === previousCursor) {
              throw new Error(`Sent ${key}, but the battle menu cursor did not move. Stopped without pressing A.`);
            }
            if (!gameStateRef.current.battleActionMenuReady) {
              throw new Error("Battle action menu closed before Run was selected. Stopped without pressing A.");
            }
          }
          if (!automaticWildHuntRunningRef.current) break;
          if (!gameStateRef.current.battleActionMenuReady || gameStateRef.current.battleActionMenuCursor !== 3) {
            throw new Error("Could not verify Run in the battle menu. Stopped without pressing A.");
          }

          setWildHuntStatus(`Run is selected for ${pendingRunPokemon.speciesName}; sending A...`);
          const runSpeciesName = pendingRunPokemon.speciesName;
          if (!(await sendBattleKey("a"))) break;
          const overworldDeadline = Date.now() + 15000;
          setWildHuntStatus(`Run sent for ${runSpeciesName}; waiting for the “Got away safely!” text...`);
          while (
            automaticWildHuntRunningRef.current &&
            !gameStateRef.current.wildEscapedTextReady &&
            gameStateRef.current.emeraldInBattle !== false &&
            Date.now() < overworldDeadline
          ) await wait(50);
          if (!automaticWildHuntRunningRef.current) break;
          if (gameStateRef.current.wildEscapedTextReady) {
            // The capture shows this callback can be replaced in about 230 ms.
            // Press immediately, then retry quickly only if the same escape
            // message remains active; this cannot choose a battle menu action.
            let acknowledgedEscape = false;
            for (let attempt = 1; attempt <= 3; attempt += 1) {
              if (!automaticWildHuntRunningRef.current) break;
              if (!gameStateRef.current.wildEscapedTextReady) {
                acknowledgedEscape = true;
                break;
              }
              setWildHuntStatus(`“Got away safely!” detected; acknowledging text (${attempt}/3)...`);
              if (!(await sendBattleKey("a"))) break;
              const acknowledgementDeadline = Date.now() + 180;
              while (
                automaticWildHuntRunningRef.current &&
                gameStateRef.current.wildEscapedTextReady &&
                gameStateRef.current.emeraldInBattle !== false &&
                Date.now() < acknowledgementDeadline
              ) await wait(50);
              if (
                !gameStateRef.current.wildEscapedTextReady ||
                gameStateRef.current.emeraldInBattle === false
              ) {
                acknowledgedEscape = true;
                break;
              }
            }
            if (!automaticWildHuntRunningRef.current) break;
            if (!acknowledgedEscape && gameStateRef.current.wildEscapedTextReady) {
              throw new Error("Emerald kept the escape text active after three A presses. Hunt stopped without sending movement input.");
            }
          } else if (gameStateRef.current.emeraldInBattle !== false) {
            throw new Error("Run was sent, but neither the escape text nor overworld state was detected. No extra input was sent.");
          }

          const overworldConfirmDeadline = Date.now() + 15000;
          setWildHuntStatus(`Waiting for Emerald to confirm the overworld after ${runSpeciesName}...`);
          while (
            automaticWildHuntRunningRef.current &&
            gameStateRef.current.emeraldInBattle !== false &&
            Date.now() < overworldConfirmDeadline
          ) await wait(50);
          if (!automaticWildHuntRunningRef.current) break;
          if (gameStateRef.current.emeraldInBattle !== false) {
            throw new Error("Run was sent once, but Emerald did not confirm return to the overworld. No further input was sent.");
          }
          await wait(250);
          pendingRunPokemon = null;
          awaitingBattleMenu = false;
          setWildHuntStatus(`Overworld confirmed after running from ${runSpeciesName}; resuming encounter movement.`);
          await wait(300);
          continue;
        }

        if (awaitingBattleMenu) {
          await wait(50);
          continue;
        }

        if (gameStateRef.current.emeraldInBattle !== false) {
          setWildHuntStatus("Waiting for confirmed overworld state; no movement input sent.");
          await wait(50);
          continue;
        }

        const direction = turnDirections[turnDirectionIndex];
        const sent = await sendTurnKey(direction);
        if (!sent) break;
        turnDirectionIndex = (turnDirectionIndex + 1) % turnDirections.length;
        await wait(70);
      }
    } catch (huntError) {
      setWildHuntStatus(`Automatic wild hunt stopped: ${String(huntError)}`);
    } finally {
      automaticWildHuntRunningRef.current = false;
      setAutomaticWildHuntRunning(false);
    }
  }

  async function toggleBattleUiDiagnostic() {
    const enabled = !battleUiDiagnosticEnabled;
    try {
      await invoke("set_battle_ui_diagnostic", { enabled });
      setBattleUiDiagnosticEnabled(enabled);
      if (enabled) setWildHuntStatus("Battle UI diagnostics are capturing automatically when detected memory signals change. Play through overworld, wild intro, and battle menu, then export the samples.");
      else setWildHuntStatus(`Battle UI diagnostic capture stopped (${battleUiDiagnosticSamples.length} samples).`);
    } catch (diagnosticError) {
      setWildHuntStatus(`Could not change diagnostic capture: ${String(diagnosticError)}`);
    }
  }

  function stopAutomaticHunt() {
    automaticHuntRunningRef.current = false;
    setAutomaticHuntRunning(false);
    setInputDiagnosticStatus("Automatic hunt stopped by user.");
  }

  function stopAutomaticWildHunt() {
    automaticWildHuntRunningRef.current = false;
    setAutomaticWildHuntRunning(false);
    setWildHuntStatus("Automatic wild hunt stopped by user.");
  }

  async function checkForUpdates() {
    if (updateCheckInFlightRef.current || updateState === "installing") return;

    updateCheckInFlightRef.current = true;
    setUpdateState("checking");

    try {
      const update = await check({ timeout: 5000 });

      if (update) {
        setAvailableUpdate(update);
        setUpdateState("available");
        if (dismissedUpdateVersionRef.current !== update.version) {
          setShowUpdateModal(true);
        }
      } else {
        setUpdateState("up-to-date");
      }
    } catch {
      setUpdateState("offline");
    } finally {
      updateCheckInFlightRef.current = false;
    }
  }

  async function dismissUpdate() {
    if (availableUpdate) {
      dismissedUpdateVersionRef.current = availableUpdate.version;
    }
    await availableUpdate?.close().catch(() => undefined);
    setAvailableUpdate(null);
    setShowUpdateModal(false);
  }

  async function installUpdate() {
    if (!availableUpdate) return;

    setUpdateState("installing");
    setUpdateProgress(0);

    try {
      await availableUpdate.downloadAndInstall((event) => {
        if (event.event === "Progress") {
          setUpdateProgress((progress) => Math.min(99, progress + 1));
        } else if (event.event === "Finished") {
          setUpdateProgress(100);
        }
      });

      await relaunch();
    } catch (updateError) {
      setUpdateState("available");
      setError(`Update failed: ${String(updateError)}`);
    }
  }

  async function exitRegion() {
    if (scanning) await stopScanning();

    localStorage.removeItem(GAME_STORAGE_KEY);
    setParty(EMPTY_PARTY);
    setLatestStarter(null);
    setLatestWild(null);
    latestWildRef.current = null;
    setSelectedSlot(0);
    setError(null);
    setPage("dashboard");
    setShowFirstLaunch(true);
  }

  function incrementHunt(mode: HuntMode) {
    setHunts((current) => {
      if (pageRef.current !== mode) return current;
      const session = current[mode] ?? {
        attempts: 0,
      };

      return {
        ...current,
        [mode]: { ...session, attempts: session.attempts + 1 },
      };
    });
  }

  function resetHuntCounter(mode: HuntMode) {
    setHunts((current) => {
      const session = current[mode] ?? {
        attempts: 0,
      };

      return {
        ...current,
        [mode]: { ...session, attempts: 0 },
      };
    });
    setResetMode(null);
  }

  const connectionStatus = !scanning
    ? "Scanner stopped"
    : party.connected
      ? "VisualBoyAdvance connected"
      : "Waiting for VisualBoyAdvance";

  const currentGame = GAMES.find((game) => game.id === selectedGame) ?? GAMES[0];
  const currentRegionName = currentGame.region === "kanto" ? "KANTO" : "HOENN";

  function clearPcStorageForGameChange() {
    pcStorageRef.current = EMPTY_PC_STORAGE;
    setPcStorage(EMPTY_PC_STORAGE);
    setSelectedPcBox(0);
    setSelectedStoredPokemon(null);
  }

  function chooseFirstLaunchGame(game: GameId) {
    setSelectedGame(game);
    clearPcStorageForGameChange();
    setParty(EMPTY_PARTY);
    setLatestStarter(null);
    setLatestWild(null);
    latestWildRef.current = null;
    setSelectedSlot(0);
    setError(null);
    setShowFirstLaunch(false);
  }

  const selectedPokemon = selectedStoredPokemon ?? party.pokemon.find(
    (pokemon) => pokemon.partySlot === selectedSlot + 1,
  );

  function changePokemonEvTarget(
    pokemon: ScannedPokemon,
    stat: EvStat,
    value: number,
  ) {
    const key = getEvTrackKey(selectedGame, pokemon);
    setEvTracks((current) => {
      const existing = current[key] ?? {
        target: { ...EMPTY_EV_VALUES },
      };
      const values = { ...existing.target };
      const otherTotal = EV_STATS.reduce((sum, entry) => (
        entry.id === stat ? sum : sum + values[entry.id]
      ), 0);
      const parsed = Number.isFinite(value) ? Math.floor(value) : 0;
      values[stat] = Math.min(255, Math.max(0, Math.min(parsed, 510 - otherTotal)));
      return { ...current, [key]: { target: values } };
    });
  }

  const evPagePokemon = selectedPokemon ?? party.pokemon[0] ?? null;
  const evPageTarget = evPagePokemon
    ? evTracks[getEvTrackKey(selectedGame, evPagePokemon)]?.target ?? { ...EMPTY_EV_VALUES }
    : null;

  const dashboardShinies = Array.from(
    new Map(
      [...party.pokemon, latestStarter, latestWild]
        .filter((pokemon): pokemon is ScannedPokemon => pokemon !== null && pokemon.shiny)
        .map((pokemon) => [`${pokemon.pid}:${pokemon.species}`, pokemon]),
    ).values(),
  );
  const dashboardWildEncounters = hunts.wild?.attempts ?? 0;
  const dashboardStarterResets = hunts.starter?.attempts ?? 0;
  const activeHuntLabel = automaticWildHuntRunning
    ? `Wild shiny hunt · ${latestWild?.speciesName ?? "searching for an encounter"}`
    : automaticHuntRunning
      ? `${STARTER_NAME[starterTarget]} starter hunt`
      : "No automatic hunt running";
  const activeHuntProgress = automaticWildHuntRunning
    ? Math.min(100, (1 - Math.pow(8191 / 8192, dashboardWildEncounters)) * 100)
    : Math.min(100, (1 - Math.pow(8191 / 8192, dashboardStarterResets)) * 100);

  const navigation: { id: Page; label: string; icon: string }[] = [
    { id: "dashboard", label: "Dashboard", icon: "⌂" },
    { id: "party", label: "Pokémon Party", icon: "◈" },
    { id: "starter", label: "Starter Hunt", icon: "✦" },
    { id: "wild", label: "Wild Hunt", icon: "⌖" },
    { id: "ev-training", label: "EV Training", icon: "⬡" },
    { id: "settings", label: "Settings", icon: "⚙" },
  ];

  navigation.splice(2, 0, { id: "pc-storage", label: "PC Storage", icon: "▦" });

  return (
    <main
      className="app-shell"
      style={
        {
          "--pc-box-background": `url("${getBoxWallpaperUrl(boxTheme)}")`,
          "--pcbox-wallpaper": `url("${getBoxWallpaperUrl(boxTheme)}")`,
          "--pcbox-accent": BOX_THEME_PALETTES[boxTheme].accent,
          "--pcbox-accent-secondary": BOX_THEME_PALETTES[boxTheme].secondary,
          "--pcbox-highlight": BOX_THEME_PALETTES[boxTheme].highlight,
          "--pcbox-border": BOX_THEME_PALETTES[boxTheme].border,
          "--pcbox-glow": BOX_THEME_PALETTES[boxTheme].glow,
          "--pcbox-background-tint": BOX_THEME_PALETTES[boxTheme].tint,
        } as CSSProperties
      }
    >
      <div className="floating-pokemon-layer" aria-hidden="true">
        {floatingSprites.map(({ nationalDex, shiny }, index) => (
          <img
            className={`floating-pokemon floating-pokemon-${index + 1}`}
            src={
              (shiny ? shinyPokemonSprites : normalPokemonSprites)[
                `./assets/pokemon${shiny ? "/shiny" : ""}/${nationalDex}.png`
              ]
            }
            alt=""
          />
        ))}
      </div>

      <header className="topbar">
        <div className="brand">
          <div className="brand-mark" aria-hidden="true"><span /></div>
          <div>
            <h1>POKÉMON SHINY SCANNER</h1>
            <p>{currentGame.name.toUpperCase()} · {currentGame.generation}</p>
          </div>
        </div>

        <div
          className={`connection-pill ${party.connected && scanning ? "online" : ""}`}
        >
          <span className="status-dot" />
          {connectionStatus}
        </div>

        <label className="box-theme-control">
          <span>BOX THEME</span>
          <select
            value={boxTheme}
            onChange={(event) => setBoxTheme(Number(event.target.value))}
            aria-label="Box theme"
          >
            {BOX_THEMES.map((theme, index) => (
              <option value={index} key={theme}>
                {theme}
              </option>
            ))}
          </select>
        </label>

        <label className="game-selector-control">
          <span>GAME</span>
          <select
            value={selectedGame}
            onChange={(event) => {
              if (scanning) return;
              setSelectedGame(event.target.value as GameId);
              clearPcStorageForGameChange();
              setParty(EMPTY_PARTY);
              setLatestStarter(null);
              setLatestWild(null);
              latestWildRef.current = null;
              setSelectedSlot(0);
              setError(null);
            }}
            disabled={scanning}
            aria-label="Game to scan"
          >
            {GAMES.map((game) => (
              <option value={game.id} key={game.id}>
                {game.name}{game.available ? "" : " — Coming soon"}
              </option>
            ))}
          </select>
        </label>
      </header>

      <div className="app-layout">
        <aside className="sidebar">
          <span className="sidebar-label">POKÉDEX INDEX</span>
          {navigation.map((item) => (
            <button
              key={item.id}
              className={`nav-item ${page === item.id ? "active" : ""}`}
              aria-current={page === item.id ? "page" : undefined}
              onClick={() => navigateTo(item.id)}
            >
              <span className="nav-icon" aria-hidden="true">{item.icon}</span>
              <span>{item.label}</span>
            </button>
          ))}

          <div className="sidebar-bottom">
            <span className="sidebar-label">EMULATOR</span>
            <div className="sidebar-emulator">
              <span
                className={`status-dot ${party.connected ? "connected" : ""}`}
              />
              <div>
                <strong>
                  {party.connected ? "Connected" : "Disconnected"}
                </strong>
                <small>VisualBoyAdvance</small>
              </div>
            </div>

            {!scanning ? (
              <button className="sidebar-scan-button" onClick={startScanning}>
                ▶ Start Scanner
              </button>
            ) : (
              <button className="sidebar-stop-button" onClick={stopScanning}>
                ■ Stop Scanner
              </button>
            )}

            <button className="region-exit-button" onClick={exitRegion}>
              ? Change Region
            </button>
          </div>
        </aside>

        <section key={page} className="main-content">
          {error && (
            <div className="error-banner">
              <strong>Scanner error</strong>
              <p>{error}</p>
            </div>
          )}

          {page === "dashboard" && (
            <div className="page-content dashboard-page">
              <section className="dashboard-hero">
                <div className="dashboard-hero-copy">
                  <span className="eyebrow">{currentRegionName} REGION · TRAINER CONSOLE</span>
                  <h2>Welcome back, Trainer</h2>
                  <p>{currentGame.name} · {currentGame.generation} live scan overview</p>
                  <div className="dashboard-target-row">
                    <div className={`dashboard-connection-mark ${party.connected && scanning ? "is-online" : ""}`} />
                    <span>{connectionStatus}</span>
                    <span className="dashboard-divider" />
                    <span>{activeHuntLabel}</span>
                  </div>
                  <div className="dashboard-progress-block">
                    <div className="dashboard-progress-labels">
                      <span>{automaticWildHuntRunning ? "WILD HUNT PROBABILITY ESTIMATE" : automaticHuntRunning ? "STARTER HUNT PROBABILITY ESTIMATE" : "HUNT SESSION"}</span>
                      <strong>{automaticWildHuntRunning || automaticHuntRunning ? `${activeHuntProgress.toFixed(2)}%` : "Ready"}</strong>
                    </div>
                    <div className="dashboard-progress-track">
                      <span style={{ width: `${activeHuntProgress}%` }} />
                    </div>
                    <small>Probability estimate from Gen III base odds. Each encounter is independent.</small>
                  </div>
                  <div className="dashboard-hero-actions">
                    <button className="primary-button" onClick={() => navigateTo(automaticHuntRunning ? "starter" : "wild")}>
                      {automaticWildHuntRunning ? "Open Wild Hunt" : automaticHuntRunning ? "Open Starter Hunt" : "Open Wild Hunt"}
                    </button>
                    <button className="secondary-button" onClick={() => navigateTo("party")}>View Party Data</button>
                  </div>
                </div>
                <div className="dashboard-hero-art" aria-label="Latest Pokémon scan">
                  {(latestWild ?? latestStarter ?? party.pokemon[0]) ? (
                    <PokemonSprite pokemon={(latestWild ?? latestStarter ?? party.pokemon[0])!} className="dashboard-hero-sprite" />
                  ) : <div className="dashboard-hero-sigil">◇</div>}
                  <span className="dashboard-art-caption">LIVE MEMORY SCAN</span>
                  <span className="dashboard-art-orbit orbit-one" />
                  <span className="dashboard-art-orbit orbit-two" />
                </div>
              </section>

              <section className="dashboard-metrics" aria-label="Hunt statistics">
                <article className="dashboard-metric">
                  <span className="dashboard-metric-icon">⌖</span>
                  <div><small>WILD ENCOUNTERS</small><strong>{dashboardWildEncounters.toLocaleString()}</strong><span>tracked this device</span></div>
                </article>
                <article className="dashboard-metric">
                  <span className="dashboard-metric-icon">↻</span>
                  <div><small>STARTER RESETS</small><strong>{dashboardStarterResets.toLocaleString()}</strong><span>tracked this device</span></div>
                </article>
                <article className="dashboard-metric shiny-metric">
                  <span className="dashboard-metric-icon">✦</span>
                  <div><small>VISIBLE SHINY RECORDS</small><strong>{dashboardShinies.length}</strong><span>party and latest encounter cache</span></div>
                </article>
                <article className="dashboard-metric">
                  <span className="dashboard-metric-icon">◈</span>
                  <div><small>PARTY REGISTER</small><strong>{party.partyCount}<i> / 6</i></strong><span>{party.connected ? "memory data available" : "waiting for emulator"}</span></div>
                </article>
              </section>

              <div className="dashboard-lower-grid">
                <section className="device-panel dashboard-party-panel">
                  <div className="device-panel-heading">
                    <div><span className="eyebrow">ACTIVE PARTY</span><h3>Pokémon register</h3></div>
                    <button className="text-action" onClick={() => navigateTo("party")}>Open party <span>↗</span></button>
                  </div>
                  <div className="dashboard-party-strip">
                    {Array.from({ length: 6 }, (_, index) => {
                      const pokemon = party.pokemon.find((entry) => entry.partySlot === index + 1);
                      return (
                        <button className={`dashboard-party-slot ${pokemon ? "filled" : ""} ${pokemon?.shiny ? "shiny-slot" : ""}`} key={index} onClick={() => pokemon ? (setSelectedStoredPokemon(null), setSelectedSlot(pokemon.partySlot - 1), navigateTo("pokemon-details")) : navigateTo("party")} aria-label={pokemon ? `${pokemon.speciesName}, party slot ${index + 1}` : `Empty party slot ${index + 1}`}>
                          {pokemon ? <PokemonSprite pokemon={pokemon} className="dashboard-party-sprite" /> : <span className="dashboard-empty-slot">{String(index + 1).padStart(2, "0")}</span>}
                          <small>{pokemon?.speciesName ?? "EMPTY"}</small>
                          {pokemon?.shiny && <i>✦</i>}
                        </button>
                      );
                    })}
                  </div>
                  <div className="dashboard-party-foot"><span className={`status-dot ${party.connected ? "connected" : ""}`} />{party.connected ? `${party.partyCount} Pokémon decoded from emulator memory` : "Connect VisualBoyAdvance to read party data"}</div>
                </section>

                <section className="device-panel dashboard-discoveries-panel">
                  <div className="device-panel-heading">
                    <div><span className="eyebrow">DISCOVERY LOG</span><h3>Recent shiny records</h3></div>
                    <span className="panel-index">{String(dashboardShinies.length).padStart(2, "0")}</span>
                  </div>
                  {dashboardShinies.length ? (
                    <div className="dashboard-discovery-list">
                      {dashboardShinies.slice(0, 3).map((pokemon) => (
                        <button className="dashboard-discovery" key={`${pokemon.pid}-${pokemon.species}`} onClick={() => navigateTo("party")}>
                          <PokemonSprite pokemon={pokemon} className="dashboard-discovery-sprite" />
                          <span><strong>{pokemon.speciesName}</strong><small>#{String(pokemon.nationalDex).padStart(3, "0")} · SV {pokemon.shinyValue}</small></span>
                          <i>✦</i>
                        </button>
                      ))}
                    </div>
                  ) : (
                    <div className="dashboard-no-discoveries"><span>✧</span><p>No shiny Pokémon in the current party or latest encounter record.</p></div>
                  )}
                </section>
              </div>

              <section className="dashboard-module-grid">
                <button className="dashboard-module-card" onClick={() => navigateTo("starter")}><span className="module-symbol starter-symbol">✦</span><span className="module-copy"><small>HOENN STARTERS</small><strong>Starter Hunt</strong><em>{automaticHuntRunning ? `${STARTER_NAME[starterTarget]} · active` : `${dashboardStarterResets.toLocaleString()} resets tracked`}</em></span><span className="module-arrow">↗</span></button>
                <button className="dashboard-module-card" onClick={() => navigateTo("wild")}><span className="module-symbol wild-symbol">⌖</span><span className="module-copy"><small>ENCOUNTER TRACKING</small><strong>Wild Hunt</strong><em>{automaticWildHuntRunning ? "Automatic hunt active" : `${dashboardWildEncounters.toLocaleString()} encounters tracked`}</em></span><span className="module-arrow">↗</span></button>
                <button className="dashboard-module-card" onClick={() => navigateTo("ev-training")}><span className="module-symbol ev-symbol">⬡</span><span className="module-copy"><small>COMPETITIVE PREP</small><strong>EV Training</strong><em>Memory-read stats · spread goals</em></span><span className="module-arrow">↗</span></button>
              </section>
            </div>
          )}

          {page === "pc-storage" && (
            <PcStoragePage
              storage={pcStorage}
              selectedBox={selectedPcBox}
              onSelectBox={setSelectedPcBox}
              onSelectPokemon={(pokemon) => {
                setSelectedStoredPokemon(pokemon);
                navigateTo("pokemon-details");
              }}
            />
          )}

          {page === "party" && (
            <div className="page-content pcbox-screen">
              <div className="page-heading">
                <div>
                  <span className="eyebrow">{currentRegionName} REGION</span>
                  <h2>Pokémon Party</h2>
                  <p>Select a Pokémon card to inspect its individual values.</p>
                </div>
                <span className="party-count">
                  {party.partyCount} / 6 MEMBERS
                </span>
              </div>

              {!party.connected ? (
                <div className="empty-state">
                  <div className="empty-icon">◇</div>
                  <h3>Waiting for emulator</h3>
                  <p>
                    Open {currentGame.name} in VisualBoyAdvance and start the
                    scanner.
                  </p>
                </div>
              ) : (
                <>
                  <div className="pokemon-grid">
                    {Array.from({ length: 6 }, (_, index) => {
                      const pokemon = party.pokemon.find(
                        (candidate) => candidate.partySlot === index + 1,
                      );

                      return pokemon ? (
                        <PokemonCard
                          key={`${pokemon.pid}-${index}`}
                          pokemon={pokemon}
                          slot={index + 1}
                          selected={selectedSlot === index}
                          onClick={() => {
                            setSelectedStoredPokemon(null);
                            setSelectedSlot(index);
                            navigateTo("pokemon-details");
                          }}
                        />
                      ) : (
                        <EmptyPartyCard
                          key={`empty-${index}`}
                          slot={index + 1}
                        />
                      );
                    })}
                  </div>

                  {party.pokemon.length === 0 && (
                    <div className="empty-state compact-empty">
                      <h3>Your party is empty</h3>
                      <p>No Pokémon are currently registered in your party.</p>
                    </div>
                  )}
                </>
              )}
            </div>
          )}

          {page === "pokemon-details" && (
            <div className="page-content pokemon-details-page pcbox-screen">
              <div className="page-heading">
                <div>
                  <span className="eyebrow">POKÉMON INFORMATION</span>
                  <h2>Pokémon Details</h2>
                  <p>
                    Inspect your Pokémon's characteristics and individual
                    values.
                  </p>
                </div>

                <button
                  className="secondary-button"
                  onClick={() => {
                    if (selectedStoredPokemon) {
                      setSelectedStoredPokemon(null);
                      navigateTo("pc-storage");
                    } else {
                      navigateTo("party");
                    }
                  }}
                >
                  ← Back to Party
                </button>
              </div>

              {selectedPokemon ? (
                <PokemonDetails
                  pokemon={selectedPokemon}
                  evTarget={evTracks[getEvTrackKey(selectedGame, selectedPokemon)]?.target ?? { ...EMPTY_EV_VALUES }}
                  onEvTargetChange={(stat, value) => changePokemonEvTarget(selectedPokemon, stat, value)}
                />
              ) : (
                <div className="empty-state">
                  <h3>No Pokémon selected</h3>
                  <p>
                    Select a Pokémon from your party to inspect its information.
                  </p>
                  <button
                    className="primary-button"
                    onClick={() => navigateTo("party")}
                  >
                    Return to Party
                  </button>
                </div>
              )}
            </div>
          )}

          {page === "ev-training" && (
            <EvTrainingPage
              party={party}
              selectedPokemon={evPagePokemon}
              target={evPageTarget}
              onSelectPokemon={(pokemon) => setSelectedSlot(pokemon.partySlot - 1)}
              onEvTargetChange={changePokemonEvTarget}
            />
          )}

          {page === "settings" && (
            <div className="page-content settings-page">
              <div className="page-heading">
                <div>
                  <span className="eyebrow">DEVICE CONFIGURATION</span>
                  <h2>Settings</h2>
                  <p>Configure the game source, PC theme, and scanner connection.</p>
                </div>
                <span className="settings-version">SCANNER · v{APP_VERSION}</span>
              </div>
              <div className="settings-grid">
                <section className="device-panel settings-card settings-wallpaper-card">
                  <span className="settings-card-icon">◉</span>
                  <span className="eyebrow">GAME SOURCE</span>
                  <h3>Emulator profile</h3>
                  <p>Choose the supported Generation III game profile to scan.</p>
                  <label className="settings-field">
                    <span>Active game</span>
                    <select
                      value={selectedGame}
                      onChange={(event) => {
                        if (scanning) return;
                        setSelectedGame(event.target.value as GameId);
                        clearPcStorageForGameChange();
                        setParty(EMPTY_PARTY);
                        setLatestStarter(null);
                        setLatestWild(null);
                        latestWildRef.current = null;
                        setSelectedSlot(0);
                        setError(null);
                      }}
                      disabled={scanning}
                    >
                      {GAMES.map((game) => <option value={game.id} key={game.id}>{game.name}</option>)}
                    </select>
                  </label>
                  <small>{scanning ? "Stop the scanner before switching game profiles." : `${currentRegionName} memory profile selected.`}</small>
                </section>

                <section className="device-panel settings-card">
                  <span className="settings-card-icon">▧</span>
                  <span className="eyebrow">DISPLAY</span>
                  <h3>PC Box theme</h3>
                  <p>Choose a wallpaper. It updates the scanner background immediately and is saved automatically.</p>
                  <label className="settings-field">
                    <span>Selected wallpaper: {String(boxTheme).padStart(2, "0")} · {BOX_THEMES[boxTheme]}</span>
                    <select value={boxTheme} onChange={(event) => setBoxTheme(Number(event.target.value))}>
                      {BOX_THEMES.map((theme, index) => <option value={index} key={theme}>{theme}</option>)}
                    </select>
                  </label>
                  <div className="pc-wallpaper-grid" role="group" aria-label="PC Box wallpapers">
                    {BOX_THEMES.map((theme, index) => {
                      const selected = boxTheme === index;
                      const wallpaperUrl = getBoxWallpaperUrl(index);
                      return (
                        <button
                          className={`pc-wallpaper-choice${selected ? " selected" : ""}`}
                          type="button"
                          key={theme}
                          onClick={() => setBoxTheme(index)}
                          aria-pressed={selected}
                          aria-label={`Wallpaper ${index}: ${theme}${selected ? ", selected" : ""}`}
                        >
                          <span className="pc-wallpaper-thumbnail" style={{ backgroundImage: `url("${wallpaperUrl}")` }} />
                          <span className="pc-wallpaper-label"><span>{String(index).padStart(2, "0")}</span>{theme}</span>
                          {selected && <span className="pc-wallpaper-check" aria-hidden="true">✓</span>}
                        </button>
                      );
                    })}
                  </div>
                </section>

                <section className="device-panel settings-card settings-emulator-card">
                  <span className="settings-card-icon">⌁</span>
                  <span className="eyebrow">MEMORY CONNECTION</span>
                  <h3>VisualBoyAdvance</h3>
                  <p>Read-only scanning for party data, wild encounters, and game state.</p>
                  <div className="settings-connection-state">
                    <span className={`status-dot ${party.connected ? "connected" : ""}`} />
                    <strong>{party.connected ? "Emulator connected" : scanning ? "Waiting for emulator" : "Scanner stopped"}</strong>
                  </div>
                  {!scanning ? (
                    <button className="primary-button" onClick={startScanning}>Start Scanner</button>
                  ) : (
                    <button className="reset-button" onClick={() => void stopScanning()}>Stop Scanner</button>
                  )}
                </section>

                <section className="device-panel settings-card settings-region-card">
                  <span className="settings-card-icon">⌖</span>
                  <span className="eyebrow">REGION</span>
                  <h3>{currentGame.region === "hoenn" ? "Hoenn" : "Kanto"}</h3>
                  <p>{REGIONS.find((region) => region.id === currentGame.region)?.subtitle}</p>
                  <button className="secondary-button" onClick={() => void exitRegion()}>Change Region</button>
                </section>
              </div>
            </div>
          )}

          {page === "starter" && (
            <HuntPage
              mode="starter"
              session={hunts.starter}
              partySlotOne={latestStarter}
              onIncrement={() => incrementHunt("starter")}
               onRequestReset={() => resetHuntCounter("starter")}
              onPressDiagnosticA={() => void pressDiagnosticA()}
              diagnosticStatus={inputDiagnosticStatus}
              onStartAutomaticHunt={() => void startAutomaticHunt()}
              onStopAutomaticHunt={stopAutomaticHunt}
              automaticHuntRunning={automaticHuntRunning}
              wildHuntStatus=""
              onStartAutomaticWildHunt={() => undefined}
              onStopAutomaticWildHunt={() => undefined}
              automaticWildHuntRunning={false}
              battleUiDiagnosticEnabled={false}
              battleUiDiagnosticSampleCount={0}
              onToggleBattleUiDiagnostic={() => undefined}
              starterTarget={starterTarget}
              onStarterTargetChange={setStarterTarget}
              scanning={scanning}
              latestWild={null}
            />
          )}

          {page === "wild" && (
            <HuntPage
              mode="wild"
              session={hunts.wild}
              partySlotOne={null}
              onIncrement={() => incrementHunt("wild")}
               onRequestReset={() => resetHuntCounter("wild")}
              onPressDiagnosticA={() => undefined}
              diagnosticStatus={inputDiagnosticStatus}
              onStartAutomaticHunt={() => void startAutomaticWildHunt()}
              onStopAutomaticHunt={() => undefined}
              automaticHuntRunning={false}
              wildHuntStatus={wildHuntStatus}
              onStartAutomaticWildHunt={() => void startAutomaticWildHunt()}
              onStopAutomaticWildHunt={stopAutomaticWildHunt}
              automaticWildHuntRunning={automaticWildHuntRunning}
              battleUiDiagnosticEnabled={battleUiDiagnosticEnabled}
              battleUiDiagnosticSampleCount={battleUiDiagnosticSamples.length}
              onToggleBattleUiDiagnostic={() => void toggleBattleUiDiagnostic()}
              starterTarget="treecko"
              onStarterTargetChange={() => undefined}
              scanning={scanning}
              latestWild={latestWild}
            />
          )}
        </section>
      </div>

      <footer className="app-footer">
        <span>POKÉMON SHINY SCANNER</span>
        <div className="app-footer-update">
          <span>v{APP_VERSION}</span>
          <button
            className={`update-status update-status-${updateState}`}
            disabled={updateState === "checking" || updateState === "installing"}
            onClick={() =>
              updateState === "available"
                ? setShowUpdateModal(true)
                : void checkForUpdates()
            }
          >
            {updateState === "checking" && "Checking for updates…"}
            {updateState === "available" && "Update available"}
            {updateState === "up-to-date" && "Up to date · Check again"}
            {updateState === "offline" && "Offline · Check for updates"}
            {updateState === "installing" && `Installing… ${updateProgress}%`}
          </button>
        </div>
      </footer>

      {pageTransitioning && (
        <div className="menu-encounter-transition" aria-hidden="true">
          <div className="transition-scanline" />
          <div className="transition-corner-mark">◆</div>
        </div>
      )}

      {resetMode && (
        <div className="modal-backdrop">
          <section className="confirm-modal">
            <div className="modal-icon">↻</div>
            <span className="eyebrow">RESET CONFIRMATION</span>
            <h2>Reset hunting counter?</h2>
            <p>
              Your current {resetMode === "starter" ? "reset" : "encounter"}{" "}
              count will return to zero.
            </p>
            <div className="modal-actions">
              <button
                className="secondary-button"
                onClick={() => setResetMode(null)}
              >
                Cancel
              </button>
              <button
                className="reset-button"
                onClick={() => resetHuntCounter(resetMode)}
              >
                Reset Counter
              </button>
            </div>
          </section>
        </div>
      )}

      {wildHuntWarning && (
        <div className="modal-backdrop" role="presentation">
          <section className="confirm-modal wild-hunt-warning" role="alertdialog" aria-modal="true" aria-labelledby="wild-hunt-warning-title">
            <div className="modal-icon">!</div>
            <span className="eyebrow">AUTOMATIC WILD HUNT STOPPED</span>
            <h2 id="wild-hunt-warning-title">No Poké Balls left</h2>
            <p>{wildHuntWarning}</p>
            <div className="modal-actions">
              <button className="primary-button" onClick={() => setWildHuntWarning(null)}>
                Got it
              </button>
            </div>
          </section>
        </div>
      )}

      {showUpdateModal && availableUpdate && (
        <div className="modal-backdrop">
          <section className="confirm-modal update-modal">
            <div className="modal-icon">↟</div>
            <span className="eyebrow">SOFTWARE UPDATE</span>
            <h2>Version {availableUpdate.version} is ready</h2>
            <p>
              You are running v{APP_VERSION}. Review the release notes before
              downloading the signed update.
            </p>
            <div className="update-notes">
              {availableUpdate.body || "Release notes were not provided."}
            </div>
            <a
              className="update-release-link"
              href={`${RELEASES_URL}/v${encodeURIComponent(availableUpdate.version)}`}
              target="_blank"
              rel="noreferrer"
            >
              View full release notes on GitHub
            </a>
            <div className="modal-actions">
              <button className="secondary-button" onClick={() => void dismissUpdate()}>
                Later
              </button>
              <button className="primary-button" onClick={() => void installUpdate()}>
                Download and Install
              </button>
            </div>
          </section>
        </div>
      )}

      {showFirstLaunch && (
        <FirstLaunchPicker
          selectedGame={selectedGame}
          onSelect={chooseFirstLaunchGame}
        />
      )}
    </main>
  );
}

export default App;
