import { useEffect, useRef, useState, type CSSProperties } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { relaunch } from "@tauri-apps/plugin-process";
import { check, type Update } from "@tauri-apps/plugin-updater";
import "./App.css";
import pcBoxBackground from "./assets/pc_box/background-pc.png";
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

interface PokemonIvs {
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
  shiny: boolean;
  abilityNumber: number;
  abilityName: string;
  checksumValid: boolean;
  ivs: PokemonIvs;
  primaryType?: string;
}

interface PartyUpdate {
  connected: boolean;
  partyCount: number;
  pokemon: ScannedPokemon[];
}

type Page = "dashboard" | "party" | "pokemon-details" | "starter" | "wild";
type HuntMode = "starter" | "wild";

interface HuntSession {
  attempts: number;
}

interface HuntStorage {
  starter: HuntSession | null;
  wild: HuntSession | null;
}

const STORAGE_KEY = "emerald-iv-scanner-hunts";
const BOX_THEME_STORAGE_KEY = "emerald-iv-scanner-box-theme";
const GAME_STORAGE_KEY = "pokemon-shiny-scanner-game";
const APP_VERSION = "0.2.0";

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

// Accent colors sampled from the visual mood of each PC box theme.
const BOX_THEME_COLORS = [
  ["#55c981", "#193f2b"], ["#a6b3bd", "#26343b"], ["#d6a35f", "#49341e"],
  ["#7ecb66", "#1d4024"], ["#c98bd8", "#402448"], ["#e36c45", "#4a241c"],
  ["#79d8e6", "#173f49"], ["#9b8add", "#2c2351"], ["#e5b86b", "#4a361e"],
  ["#5ba8e8", "#1b3552"], ["#4fc9d0", "#16454a"], ["#86b8f0", "#243d5c"],
  ["#e06b7d", "#4b202b"], ["#9ba8b7", "#303943"], ["#e85a5a", "#4c1d25"],
  ["#dce8d6", "#38483a"], ["#d8c4a0", "#493d2c"], ["#6d8fda", "#202f5a"],
  ["#ee6f77", "#4e2327"], ["#f0d477", "#4b3b1b"], ["#e78ac0", "#4a2540"],
  ["#756f88", "#282633"], ["#8ed0a1", "#21402e"], ["#dd8f76", "#482b24"],
] as const;

const EMPTY_PARTY: PartyUpdate = {
  connected: false,
  partyCount: 0,
  pokemon: [],
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

function getBoxThemePosition(themeIndex: number) {
  const column = themeIndex % 4;
  const row = Math.floor(themeIndex / 4);
  const cropLeft = 15 + column * 172;
  const cropTop = 43 + row * 164;

  return `${(cropLeft / (692 - 145)) * 100}% ${(cropTop / (1060 - 105)) * 100}%`;
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


function PokemonDetails({
  pokemon,
  hideArt = false,
  huntCounter,
  huntCounterLabel,
  onReset,
}: {
  pokemon: ScannedPokemon;
  hideArt?: boolean;
  huntCounter?: number;
  huntCounterLabel?: string;
  onReset?: () => void;
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
              <strong className={pokemon.shiny ? "shiny-text" : ""}>
                {pokemon.shiny ? "Yes" : "No"}
              </strong>
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


function HuntPage({
  mode,
  session,
  partySlotOne,
  latestWild,
  onIncrement,
  onRequestReset,
}: {
  mode: HuntMode;
  session: HuntSession | null;
  partySlotOne: ScannedPokemon | null;
  latestWild: ScannedPokemon | null;
  onIncrement: () => void;
  onRequestReset: () => void;
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
  const [floatingSprites] = useState(getRandomFloatingSprites);
  const [latestWild, setLatestWild] = useState<ScannedPokemon | null>(null);
  const [scanning, setScanning] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [page, setPage] = useState<Page>("dashboard");
  const [pageTransitioning, setPageTransitioning] = useState(false);
  const [selectedSlot, setSelectedSlot] = useState(0);
  const [hunts, setHunts] = useState<HuntStorage>(loadHunts);
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
    localStorage.setItem(STORAGE_KEY, JSON.stringify(hunts));
  }, [hunts]);

  useEffect(() => {
    localStorage.setItem(BOX_THEME_STORAGE_KEY, String(boxTheme));
  }, [boxTheme]);

  useEffect(() => {
    localStorage.setItem(GAME_STORAGE_KEY, selectedGame);
  }, [selectedGame]);

  useEffect(() => {
    const timer = window.setTimeout(() => {
      void checkForUpdates();
    }, 1200);

    return () => window.clearTimeout(timer);
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
            setParty(event.payload);
            setError(null);
            setSelectedSlot((current) =>
              Math.min(current, Math.max(0, event.payload.pokemon.length - 1)),
            );
          }),
          listen<string>("scanner-error", (event) => setError(event.payload)),

           listen("scanner-stopped", () => {
             scanningRef.current = false;
             setScanning(false);
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

  async function checkForUpdates() {
    if (updateState === "checking" || updateState === "installing") return;

    setUpdateState("checking");

    try {
      const update = await check({ timeout: 5000 });

      if (update) {
        setAvailableUpdate(update);
        setUpdateState("available");
        setShowUpdateModal(true);
      } else {
        setUpdateState("up-to-date");
      }
    } catch {
      setUpdateState("offline");
    }
  }

  async function dismissUpdate() {
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
    setLatestWild(null);
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

  function chooseFirstLaunchGame(game: GameId) {
    setSelectedGame(game);
    setParty(EMPTY_PARTY);
    setLatestWild(null);
    setSelectedSlot(0);
    setError(null);
    setShowFirstLaunch(false);
  }

  const selectedPokemon = party.pokemon.find(
    (pokemon) => pokemon.partySlot === selectedSlot + 1,
  );

  const navigation: { id: Page; label: string; icon: string }[] = [
    { id: "dashboard", label: "Dashboard", icon: "⌂" },
    { id: "party", label: "Pokémon Party", icon: "◈" },
    { id: "starter", label: "Starter Hunting", icon: "✦" },
    { id: "wild", label: "Wild Hunting", icon: "♧" },
  ];

  return (
    <main
      className="app-shell"
      style={
        {
          "--pc-box-background": `url(${pcBoxBackground})`,
          "--pc-box-position": getBoxThemePosition(boxTheme),
          "--theme-accent": BOX_THEME_COLORS[boxTheme][0],
          "--theme-accent-soft": BOX_THEME_COLORS[boxTheme][1],
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
          <div className="brand-mark">E</div>
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
              setParty(EMPTY_PARTY);
              setLatestWild(null);
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
          <span className="sidebar-label">MENU</span>
          {navigation.map((item) => (
            <button
              key={item.id}
              className={`nav-item ${page === item.id ? "active" : ""}`}
              onClick={() => navigateTo(item.id)}
            >
              <span className="nav-icon">{item.icon}</span>
              {item.label}
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
              ⇦ Change Region
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
            <div className="page-content">
              <div className="page-heading">
                <div>
                  <span className="eyebrow">{currentRegionName} REGION</span>
                  <h2>Welcome back, Trainer</h2>
                  <p>
                    Your {currentGame.name} scanning and shiny hunting overview.
                  </p>
                </div>
              </div>

              <div className="overview-grid">
                <div className="overview-card">
                  <span>EMULATOR</span>
                  <strong>
                    {party.connected ? "Connected" : "Disconnected"}
                  </strong>
                  <small>VisualBoyAdvance.exe</small>
                </div>
                <div className="overview-card">
                  <span>PARTY MEMBERS</span>
                  <strong>{party.partyCount} / 6</strong>
                  <small>Currently detected</small>
                </div>
                <div className="overview-card">
                  <span>SCANNER</span>
                  <strong>{scanning ? "Running" : "Stopped"}</strong>
                  <small>
                    {scanning ? "Polling every 500 ms" : "Not scanning"}
                  </small>
                </div>
              </div>

              <div className="dashboard-shortcuts">
                <button onClick={() => navigateTo("party")}>
                  <span>◈</span>
                  <strong>Pokémon Party</strong>
                  <small>View your six party slots</small>
                </button>
                <button onClick={() => navigateTo("starter")}>
                  <span>✦</span>
                  <strong>Starter Hunting</strong>
                  <small>Track starter resets</small>
                </button>
                <button onClick={() => navigateTo("wild")}>
                  <span>♧</span>
                  <strong>Wild Hunting</strong>
                  <small>Track wild encounters</small>
                </button>
              </div>
            </div>
          )}

          {page === "party" && (
            <div className="page-content">
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
            <div className="page-content pokemon-details-page">
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
                  onClick={() => navigateTo("party")}
                >
                  ← Back to Party
                </button>
              </div>

              {selectedPokemon ? (
                <PokemonDetails pokemon={selectedPokemon} />
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

          {page === "starter" && (
            <HuntPage
              mode="starter"
              session={hunts.starter}
              partySlotOne={
                party.pokemon.find((pokemon) => pokemon.partySlot === 1) ?? null
              }
              onIncrement={() => incrementHunt("starter")}
               onRequestReset={() => resetHuntCounter("starter")}
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
