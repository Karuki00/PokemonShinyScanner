
from pathlib import Path

from PySide6.QtWidgets import (
    QFrame, QVBoxLayout, QHBoxLayout,
    QLabel, QProgressBar, QGridLayout
)
from PySide6.QtCore import Qt
from PySide6.QtGui import QPixmap

from core.pokemon_data import get_pokemon, get_ability


BASE_DIR = Path(__file__).resolve().parent.parent
SPRITE_DIR = BASE_DIR / "assets" / "pokemon"


NATURE_NAMES = [
    "Hardy", "Lonely", "Brave", "Adamant", "Naughty",
    "Bold", "Docile", "Relaxed", "Impish", "Lax",
    "Timid", "Hasty", "Serious", "Jolly", "Naive",
    "Modest", "Mild", "Quiet", "Bashful", "Rash",
    "Calm", "Gentle", "Sassy", "Careful", "Quirky"
]


class PokemonCard(QFrame):
    def __init__(self, parent=None):
        super().__init__(parent)

        self.setObjectName("pokemonCard")

        outer_layout = QVBoxLayout(self)
        outer_layout.setContentsMargins(12, 10, 12, 10)
        outer_layout.setSpacing(8)

        # Card heading
        heading = QHBoxLayout()

        self.slot_label = QLabel("PARTY SLOT 1")
        self.slot_label.setObjectName("slotLabel")

        self.shiny_label = QLabel("")
        self.shiny_label.setObjectName("shinyLabel")
        self.shiny_label.setAlignment(
            Qt.AlignmentFlag.AlignRight
        )

        heading.addWidget(self.slot_label)
        heading.addStretch()
        heading.addWidget(self.shiny_label)

        outer_layout.addLayout(heading)

        # Pokémon overview
        overview = QHBoxLayout()
        overview.setSpacing(14)

        self.sprite_container = QFrame()
        self.sprite_container.setObjectName("spriteContainer")
        self.sprite_container.setFixedSize(156, 156)

        sprite_layout = QVBoxLayout(self.sprite_container)
        sprite_layout.setContentsMargins(4, 4, 4, 4)

        self.sprite_label = QLabel("No sprite")
        self.sprite_label.setAlignment(
            Qt.AlignmentFlag.AlignCenter
        )

        sprite_layout.addWidget(self.sprite_label)

        overview.addWidget(self.sprite_container)

        details_layout = QVBoxLayout()
        details_layout.setSpacing(5)

        self.name_label = QLabel("Waiting for Pokémon")
        self.name_label.setObjectName("pokemonName")
        self.name_label.setWordWrap(True)

        self.info_label = QLabel("Nature: --")
        self.info_label.setObjectName("pokemonInfo")

        self.ability_label = QLabel("Ability: --")
        self.ability_label.setObjectName("pokemonInfo")
        self.ability_label.setWordWrap(True)

        self.species_label = QLabel("Species ID: --")
        self.species_label.setObjectName("identityLabel")

        details_layout.addWidget(self.name_label)
        details_layout.addWidget(self.info_label)
        details_layout.addWidget(self.ability_label)
        details_layout.addWidget(self.species_label)
        details_layout.addStretch()

        overview.addLayout(details_layout, 1)

        outer_layout.addLayout(overview)

        # Advanced Pokémon data
        separator = QFrame()
        separator.setObjectName("separator")
        separator.setFrameShape(QFrame.Shape.HLine)

        outer_layout.addWidget(separator)

        advanced_title = QLabel("ADVANCED DATA")
        advanced_title.setObjectName("ivTitle")

        outer_layout.addWidget(advanced_title)

        identity_grid = QGridLayout()
        identity_grid.setHorizontalSpacing(14)
        identity_grid.setVerticalSpacing(5)

        self.pid_label = QLabel("PID: --")
        self.otid_label = QLabel("OTID: --")
        self.ability_slot_label = QLabel("Ability Slot: --")

        for label in (
            self.pid_label,
            self.otid_label,
            self.ability_slot_label
        ):
            label.setObjectName("identityLabel")
            label.setWordWrap(True)

        identity_grid.addWidget(self.pid_label, 0, 0)
        identity_grid.addWidget(self.otid_label, 0, 1)
        identity_grid.addWidget(self.ability_slot_label, 1, 0)

        outer_layout.addLayout(identity_grid)

        separator2 = QFrame()
        separator2.setObjectName("separator")
        separator2.setFrameShape(QFrame.Shape.HLine)

        outer_layout.addWidget(separator2)

        # Individual values
        iv_title = QLabel("INDIVIDUAL VALUES (IVs)")
        iv_title.setObjectName("ivTitle")

        outer_layout.addWidget(iv_title)

        self.iv_rows = {}

        stats = [
            ("HP", "HP"),
            ("Attack", "ATK"),
            ("Defense", "DEF"),
            ("SpAttack", "SP. ATK"),
            ("SpDefense", "SP. DEF"),
            ("Speed", "SPEED"),
        ]

        iv_grid = QGridLayout()
        iv_grid.setHorizontalSpacing(12)
        iv_grid.setVerticalSpacing(7)

        for index, (stat_key, stat_label) in enumerate(stats):
            row = QHBoxLayout()
            row.setSpacing(5)

            label = QLabel(stat_label)
            label.setObjectName("statLabel")
            label.setFixedWidth(57)

            bar = QProgressBar()
            bar.setRange(0, 31)
            bar.setValue(0)
            bar.setFormat("%v/31")
            bar.setTextVisible(True)
            bar.setObjectName("ivBar")
            bar.setMinimumHeight(17)

            row.addWidget(label)
            row.addWidget(bar, 1)

            iv_grid.addLayout(row, index // 2, index % 2)
            self.iv_rows[stat_key] = bar

        outer_layout.addLayout(iv_grid)
        outer_layout.addStretch()

        self.set_empty()

    def load_sprite(self, national_dex, shiny):
        sprite_path = (
            SPRITE_DIR / "shiny" / f"{national_dex}.png"
            if shiny
            else SPRITE_DIR / f"{national_dex}.png"
        )

        pixmap = QPixmap(str(sprite_path))

        if pixmap.isNull():
            self.sprite_label.setPixmap(QPixmap())
            self.sprite_label.setText("Sprite unavailable")
            return

        self.sprite_label.setText("")

        self.sprite_label.setPixmap(
            pixmap.scaled(
                128,
                128,
                Qt.AspectRatioMode.KeepAspectRatio,
                Qt.TransformationMode.FastTransformation
            )
        )

    def set_empty(self):
        self.name_label.setText("Waiting for Pokémon")
        self.info_label.setText("Nature: --")
        self.ability_label.setText("Ability: --")
        self.species_label.setText("Species ID: --")

        self.sprite_label.setPixmap(QPixmap())
        self.sprite_label.setText("No sprite")

        self.pid_label.setText("PID: --")
        self.otid_label.setText("OTID: --")
        self.ability_slot_label.setText("Ability Slot: --")
        self.shiny_label.setText("")

        for bar in self.iv_rows.values():
            bar.setValue(0)

        self.setProperty("shiny", False)
        self.style().unpolish(self)
        self.style().polish(self)

    def set_pokemon(self, pokemon):
        species_id = pokemon["species"]
        species_data = get_pokemon(species_id)

        if species_data is None:
            species_name = f"Unknown Species #{species_id}"
            national_dex = None
            ability_name = "Unknown"
        else:
            species_name = species_data["name"]
            national_dex = species_data["national_dex"]

            ability_name = get_ability(
                species_id,
                pokemon["ability_number"]
            )

        nature_id = pokemon["nature_id"]

        nature_name = (
            NATURE_NAMES[nature_id]
            if 0 <= nature_id < len(NATURE_NAMES)
            else f"Nature #{nature_id}"
        )

        shiny = pokemon["shiny"]

        self.name_label.setText(species_name)
        self.info_label.setText(f"Nature: {nature_name}")
        self.ability_label.setText(f"Ability: {ability_name}")
        self.species_label.setText(f"Species ID: {species_id}")

        self.pid_label.setText(f"PID: {pokemon['pid']}")
        self.otid_label.setText(f"OTID: {pokemon['otid']}")

        self.ability_slot_label.setText(
            f"Ability Slot: {pokemon['ability_number']}"
        )

        self.shiny_label.setText(
            "★ SHINY" if shiny else "NORMAL"
        )

        if national_dex is not None:
            self.load_sprite(national_dex, shiny)
        else:
            self.sprite_label.setPixmap(QPixmap())
            self.sprite_label.setText("Sprite unavailable")

        self.setProperty("shiny", shiny)
        self.style().unpolish(self)
        self.style().polish(self)

        for stat_key, bar in self.iv_rows.items():
            bar.setValue(pokemon["ivs"][stat_key])
