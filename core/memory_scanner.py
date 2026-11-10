
import threading
import time

from .memory_reader import VBAMemoryReader
from .wild_reader import WildPokemonReader


class MemoryScanner:
    def __init__(self, interval=0.5):
        self.reader = VBAMemoryReader()
        self.wild_reader = WildPokemonReader(self.reader)

        self.interval = interval

        self.running = False
        self.thread = None

        self.last_snapshot = None
        self.last_wild_snapshot = None

    def connect(self):
        return self.reader.connect()

    def disconnect(self):
        self.stop()
        self.reader.disconnect()

    def scan_once(self):
        """Read the current Pokemon party."""
        return self.reader.read_party()

    def scan_wild_once(self):
        """Read the opponent candidate and battle classification."""

        flags = self.wild_reader.read_battle_type_flags()
        pokemon = self.wild_reader.read_candidate()

        is_trainer = self.wild_reader.is_trainer_battle(flags)

        return {
            "pokemon": pokemon,
            "flags": flags,
            "is_trainer": is_trainer
        }

    def create_snapshot(self, pokemon_list):
        """Create a comparable representation of the party."""

        return tuple(
            (
                pokemon["pid"],
                pokemon["otid"],
                pokemon["species"],
                pokemon["nature_id"],
                pokemon["shiny"],
                tuple(sorted(pokemon["ivs"].items()))
            )
            for pokemon in pokemon_list
        )

    def create_wild_snapshot(self, pokemon):
        """Create a comparable representation of an opponent."""

        return (
            pokemon["pid"],
            pokemon["otid"],
            pokemon["species"],
            pokemon["nature_id"],
            pokemon["shiny"],
            tuple(sorted(pokemon["ivs"].items()))
        )

    def start(self, on_update, on_error=None, on_wild_update=None):
        """Start party monitoring and wild encounter detection."""

        if self.running:
            return

        self.running = True

        # Reset snapshots so the first scan establishes a baseline.
        self.last_snapshot = None
        self.last_wild_snapshot = None

        def scan_loop():
            while self.running:
                try:
                    # Existing party monitoring.
                    pokemon_list = self.scan_once()
                    snapshot = self.create_snapshot(pokemon_list)

                    if snapshot != self.last_snapshot:
                        self.last_snapshot = snapshot
                        on_update(pokemon_list)

                    # New wild encounter monitoring.
                    result = self.scan_wild_once()

                    wild_pokemon = result["pokemon"]

                    if wild_pokemon is not None:
                        wild_snapshot = self.create_wild_snapshot(
                            wild_pokemon
                        )

                        # Always remember the latest valid opponent,
                        # including trainer Pokemon.
                        is_new_candidate = (
                            wild_snapshot != self.last_wild_snapshot
                        )

                        self.last_wild_snapshot = wild_snapshot

                        if (
                            is_new_candidate
                            and not result["is_trainer"]
                            and on_wild_update is not None
                        ):
                            on_wild_update(wild_pokemon)

                except Exception as error:
                    if on_error:
                        on_error(error)

                time.sleep(self.interval)

        self.thread = threading.Thread(
            target=scan_loop,
            daemon=True
        )

        self.thread.start()

    def stop(self):
        """Stop continuous monitoring."""

        self.running = False

        if self.thread and self.thread.is_alive():
            self.thread.join(timeout=2)

        self.thread = None
