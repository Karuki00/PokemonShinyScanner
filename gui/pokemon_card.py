
from pathlib import Path

from PySide6.QtWidgets import (
    QFrame, QVBoxLayout, QHBoxLayout,
    QLabel, QProgressBar, QGridLayout
)
from PySide6.QtCore import Qt
from PySide6.QtGui import QPixmap, QPainter, QColor

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


class ProfileDivider(QFrame):
    def paintEvent(self, event):
        painter = QPainter(self)
        painter.setPen(QColor("#8FE8D1"))
        line_height = max(1, int(self.height() * 0.8))
        top = (self.height() - line_height) // 2
        painter.drawLine(self.width() // 2, top, self.width() // 2,
                         top + line_height)
        painter.end()


class PokemonCard(QFrame):
    def __init__(self, parent=None):
        super().__init__(parent)

        self.setObjectName("pokemonCard")

        outer_layout = QVBoxLayout(self)
        self.outer_layout = outer_layout
        outer_layout.setContentsMargins(12, 10, 12, 10)
        outer_layout.setSpacing(8)

        # Card heading
        heading = QHBoxLayout()
        self.heading_layout = heading

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
        self.overview_layout = overview
        overview.setSpacing(14)

        self.sprite_container = QFrame()
        self.sprite_container.setObjectName("spriteContainer")
        self.sprite_container.setFixedSize(156, 156)
        self._sprite_size = 128

        sprite_layout = QVBoxLayout(self.sprite_container)
        sprite_layout.setContentsMargins(4, 4, 4, 4)

        self.sprite_label = QLabel("No sprite")
        self.sprite_label.setAlignment(
            Qt.AlignmentFlag.AlignCenter
        )

        sprite_layout.addWidget(self.sprite_label)

        overview.addWidget(self.sprite_container)

        self.details_panel = QFrame()
        self.details_panel.setObjectName("detailsPanel")

        details_layout = QVBoxLayout(self.details_panel)
        details_layout.setSpacing(5)
        details_layout.setContentsMargins(14, 14, 14, 14)

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

        overview.addWidget(self.details_panel, 1)

        outer_layout.addLayout(overview)

        # Advanced Pokémon data
        separator = QFrame()
        self.separator = separator
        separator.setObjectName("separator")
        separator.setFrameShape(QFrame.Shape.HLine)

        outer_layout.addWidget(separator)

        advanced_title = QLabel("ADVANCED DATA")
        self.iv_title = advanced_title
        advanced_title.setObjectName("ivTitle")

        outer_layout.addWidget(advanced_title)

        identity_grid = QGridLayout()
        self.identity_grid = identity_grid
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
        self.separator2 = separator2
        separator2.setObjectName("separator")
        separator2.setFrameShape(QFrame.Shape.HLine)

        outer_layout.addWidget(separator2)

        # Individual values
        iv_title = QLabel("INDIVIDUAL VALUES (IVs)")
        self.iv_title_label = iv_title
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
        self.iv_grid = iv_grid
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
                self._sprite_size,
                self._sprite_size,
                Qt.AspectRatioMode.KeepAspectRatio,
                Qt.TransformationMode.FastTransformation
            )
        )

    def set_showcase_sprite(self):
        """Use a profile layout with the sprite separated from its data."""

        self._sprite_size = 224
        self.sprite_container.setFixedSize(260, 260)
        self.sprite_container.layout().setContentsMargins(0, 0, 0, 0)

        while self.outer_layout.count():
            self.outer_layout.takeAt(0)

        while self.overview_layout.count():
            self.overview_layout.takeAt(0)

        while self.heading_layout.count():
            self.heading_layout.takeAt(0)

        self.separator.hide()
        self.separator2.hide()

        identity_items = []
        while self.identity_grid.count():
            identity_items.append(self.identity_grid.takeAt(0))
        for row, item in enumerate(identity_items):
            self.identity_grid.addWidget(
                item.widget(),
                row,
                0,
                alignment=Qt.AlignmentFlag.AlignLeft
            )

        for label in (
            self.pid_label,
            self.otid_label,
            self.ability_slot_label
        ):
            label.setAlignment(Qt.AlignmentFlag.AlignLeft)

        sprite_panel = QFrame(self)
        sprite_panel.setObjectName("summarySpritePanel")
        sprite_panel_layout = QVBoxLayout(sprite_panel)
        sprite_panel_layout.setContentsMargins(18, 18, 18, 18)
        sprite_panel_layout.setSpacing(10)
        sprite_panel_layout.addWidget(
            self.slot_label,
            alignment=Qt.AlignmentFlag.AlignLeft
        )
        sprite_panel_layout.addWidget(
            self.sprite_container,
            alignment=Qt.AlignmentFlag.AlignCenter
        )
        sprite_panel_layout.addWidget(
            self.shiny_label,
            alignment=Qt.AlignmentFlag.AlignCenter
        )
        sprite_panel_layout.addStretch()

        data_panel = QFrame(self)
        data_panel.setObjectName("summaryDataPanel")
        data_layout = QVBoxLayout(data_panel)
        data_layout.setContentsMargins(22, 18, 22, 18)
        data_layout.setSpacing(9)
        data_layout.addWidget(self.details_panel)

        advanced_section = QFrame(data_panel)
        advanced_section.setObjectName("summaryAdvancedSection")
        advanced_layout = QVBoxLayout(advanced_section)
        advanced_layout.setContentsMargins(14, 12, 14, 12)
        advanced_layout.setSpacing(7)
        advanced_layout.addWidget(self.iv_title)
        advanced_layout.addLayout(self.identity_grid)
        advanced_layout.addWidget(self.iv_title_label)
        advanced_layout.addLayout(self.iv_grid)
        data_layout.addWidget(advanced_section)

        profile_layout = QHBoxLayout()
        profile_layout.setSpacing(18)
        profile_layout.addWidget(sprite_panel, 0)
        profile_divider = ProfileDivider(self)
        profile_divider.setObjectName("profileDivider")
        profile_divider.setFrameShape(QFrame.Shape.VLine)
        profile_divider.setFrameShadow(QFrame.Shadow.Plain)
        profile_layout.addWidget(profile_divider)
        profile_layout.addWidget(data_panel, 1)
        self.outer_layout.setContentsMargins(18, 18, 18, 18)
        self.outer_layout.addLayout(profile_layout)

        self.setProperty("showcase", True)
        self.style().unpolish(self)
        self.style().polish(self)

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
