
from pathlib import Path

from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QGridLayout,
    QLabel, QToolButton
)
from PySide6.QtCore import Qt, Signal, QSize
from PySide6.QtGui import QPixmap, QIcon

from core.pokemon_data import get_pokemon


BASE_DIR = Path(__file__).resolve().parent.parent
SPRITE_DIR = BASE_DIR / "assets" / "pokemon"


class PartyView(QWidget):
    pokemon_selected = Signal(dict)

    def __init__(self, parent=None):
        super().__init__(parent)

        self.pokemon_list = []
        self.slot_buttons = []

        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(10)

        self.grid = QGridLayout()
        self.grid.setSpacing(10)

        for index in range(6):
            button = QToolButton()
            button.setObjectName("partySlot")
            button.setToolButtonStyle(
                Qt.ToolButtonStyle.ToolButtonTextUnderIcon
            )
            button.setIconSize(QSize(64, 64))
            button.setMinimumSize(115, 110)
            button.setText(f"EMPTY SLOT {index + 1}")
            button.setEnabled(False)

            button.clicked.connect(
                lambda checked=False, slot=index:
                    self.select_slot(slot)
            )

            self.grid.addWidget(
                button,
                index // 2,
                index % 2
            )

            self.slot_buttons.append(button)

        layout.addLayout(self.grid)

        self.status_label = QLabel(
            "Waiting for Pokémon party data..."
        )
        self.status_label.setObjectName("panelStatus")

        layout.addWidget(self.status_label)

    def update_party(self, pokemon_list):
        self.pokemon_list = sorted(
            pokemon_list,
            key=lambda pokemon: pokemon.get("party_slot", 99)
        )

        for index, button in enumerate(self.slot_buttons):
            slot_number = index + 1

            pokemon = next(
                (
                    item for item in self.pokemon_list
                    if item.get("party_slot") == slot_number
                ),
                None
            )

            button.setIcon(QIcon())

            if pokemon is None:
                button.setText(f"EMPTY SLOT {slot_number}")
                button.setEnabled(False)
                continue

            species_data = get_pokemon(pokemon["species"])

            if species_data:
                name = species_data["name"]
                national_dex = species_data["national_dex"]
            else:
                name = f"Unknown #{pokemon['species']}"
                national_dex = None

            button.setText(name)
            button.setEnabled(True)

            if national_dex is not None:
                sprite_path = (
                    SPRITE_DIR / "shiny" / f"{national_dex}.png"
                    if pokemon["shiny"]
                    else SPRITE_DIR / f"{national_dex}.png"
                )

                pixmap = QPixmap(str(sprite_path))

                if not pixmap.isNull():
                    button.setIcon(QIcon(pixmap))

        self.status_label.setText(
            f"{len(self.pokemon_list)}/6 Pokémon detected"
        )

    def select_slot(self, slot_index):
        slot_number = slot_index + 1

        pokemon = next(
            (
                item for item in self.pokemon_list
                if item.get("party_slot") == slot_number
            ),
            None
        )

        if pokemon is not None:
            self.pokemon_selected.emit(pokemon)
